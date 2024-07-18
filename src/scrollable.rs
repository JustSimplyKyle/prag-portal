use dioxus::events::eval;

pub trait Scrollable: Sized + ToString {
    const GROUP_SELECTOR: &'static str;
    fn scroller_id(&self) -> String {
        format!("scrolling-{}", self.to_string())
    }

    fn scroller_applyer(
        pages_scroller: Vec<Self>,
        filterer: impl Fn(&Self) -> bool,
    ) -> anyhow::Result<()> {
        let iter = pages_scroller
            .iter()
            .enumerate()
            .filter(|(_, x)| filterer(&*x));
        for (u, x) in iter {
            let (left, right) = pages_scroller.split_at(u);
            x.apply_scroller_animation(&right[1..], left)?;
        }
        Ok(())
    }

    fn apply_scroller_animation(
        &self,
        bottom: &[Self],
        top: &[Self],
    ) -> Result<(), anyhow::Error> {
        let target = self.to_string();
        let bottom = bottom
            .iter()
            .map(|arg0| arg0.to_string())
            .collect::<Vec<_>>();
        let top = top.iter().map(|arg0| arg0.to_string()).collect::<Vec<_>>();
        let eval = eval(
            r#"
                    function applyStyles(self, bottom, top, group) {
                        const groups = document.querySelectorAll('.' + group);            
                        groups.forEach(group => {
                            const prev = group.getAttribute('data-prev');
                            const target = group.querySelector('#scrolling-' + self);
                            const bottomElems = bottom.map((x) => group.querySelector('#scrolling-' + x));
                            const topElems = top.map((x) => group.querySelector('#scrolling-' + x));

                            // Reset styles first
                            bottomElems.forEach((ele) => {
                                ele.style.display = 'none';
                                ele.style.zIndex = '0';
                                ele.style.animation = '';
                            });
                            topElems.forEach((ele) => {
                                ele.style.display = 'none';
                                ele.style.zIndex = '0';
                                ele.style.animation = '';
                            });

                            target.style.display = 'block';
                            target.style.zIndex = '50';
                            const finded_bottom = bottom.find((ele) => prev === ele);
                            const finded_top = top.find((ele) => prev === ele);
                            if (finded_bottom) {
                                const bottomElem = group.querySelector('#scrolling-' + finded_bottom);
                                target.style.animation = 'slideDown 1000ms';
                                bottomElem.style.display = 'block';
                                bottomElem.style.zIndex = '10';
                                bottomElem.style.animation = 'slideOutDown 1000ms';
                            }
                            else if (finded_top) {
                                const topElem = group.querySelector('#scrolling-' + finded_top);
                                target.style.animation = 'slideUp 1000ms';
                                topElem.style.display = 'block';
                                topElem.style.zIndex = '10';
                                topElem.style.animation = 'slideOutUp 1000ms';
                            }

                        });
                    }
                    const [[self], [group], bottom, top] = await dioxus.recv();
                    applyStyles(self, bottom, top, group);
                "#,
        );
        eval.send(
            vec![
                vec![target],
                vec![Self::GROUP_SELECTOR.to_owned()],
                bottom,
                top,
            ]
            .into(),
        )
        .map_err(|x| anyhow::anyhow!("{x:#?}"))
    }
}
