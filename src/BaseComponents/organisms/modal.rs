use dioxus::prelude::*;

use crate::TOP_LEVEL_COMPONENT;

#[derive(Clone)]
pub struct ComponentPointer<P: Properties> {
    pub name: String,
    pub props: P,
    pub pointer: Component<P>,
}

#[component]
pub fn Modal(
    children: Element,
    name: String,
    active: Signal<bool>,
    #[props(default = true)] close_on_outer_click: bool,
) -> Element {
    let props = __sub_modalProps::builder()
        .children(children)
        .active(active)
        .close_on_outer_click(close_on_outer_click)
        .build();
    if TOP_LEVEL_COMPONENT().into_iter().all(|x| x.name != name) {
        #[allow(deprecated)]
        let pointer = ComponentPointer {
            name,
            props,
            pointer: __sub_modal,
        };
        TOP_LEVEL_COMPONENT.write().push(pointer);
    }
    Ok(VNode::placeholder())
}

#[component]
#[doc(hidden)]
#[deprecated = "DO NOT USE. Use `Modal` instead, this should be private, but Dioxus does not allow it."]
pub fn __sub_modal(
    children: Element,
    mut active: Signal<bool>,
    close_on_outer_click: bool,
) -> Element {
    let mut modal_hover = use_signal(|| false);
    rsx! {
        div {
            class: "contents z-[200] aria-[selected=false]:hidden aria-[selected=false]:z-0 flex justify-center items-center absolute left-0 top-0 w-screen h-screen bg-white/30",
            "aria-selected": active(),
            onclick: move |_| {
                if !modal_hover() && close_on_outer_click {
                    *active.write() = false;
                }
            },
            div {
                class: "w-fit h-fit",
                onmouseenter: move |_| {
                    if close_on_outer_click {
                        *modal_hover.write() = true;
                    }
                },
                onmouseleave: move |_| {
                    if close_on_outer_click {
                        *modal_hover.write() = false;
                    }
                },
                {children}
            }
        }
    }
}
