use std::collections::HashMap;

use dioxus::{prelude::Context, CapturedError};

pub trait Scrollable: Sized + std::hash::Hash + Eq + Clone {
    fn get_order(
        pages_scroller: Vec<Self>,
        filterer: impl Fn(&Self) -> bool,
        failure_lookup: Option<&[Self]>,
    ) -> Result<HashMap<Self, String>, CapturedError> {
        let failure_lookup = failure_lookup.unwrap_or_else(|| &pages_scroller);

        let pos = pages_scroller.iter().position(&filterer).map_or_else(
            || {
                let v = failure_lookup
                    .iter()
                    .rev()
                    .find(|x| pages_scroller.contains(*x))
                    .context("failure lookup should guarantee existance")?;
                let pos = pages_scroller
                    .iter()
                    .position(|x| x == v)
                    .context("failure lookup should guarantee existence")?;
                Ok::<_, CapturedError>(pos)
            },
            Ok,
        )?;

        let transforms = pages_scroller
            .into_iter()
            .enumerate()
            .map(|(u, x)| (x, u))
            .map(|(key, x)| (key, x as isize - pos as isize))
            .map(|(key, x)| (key, format!("{}%", x * 100)))
            .collect::<HashMap<_, _>>();

        Ok(transforms)
    }
}
