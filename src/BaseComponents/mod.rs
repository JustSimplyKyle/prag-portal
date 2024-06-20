use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_core::DynamicNode;
use tailwind_fuse::*;

use crate::{collections::SEARCH, main_page::ARROW_LEFT};

pub mod button;
pub mod modal;
pub mod search_bar;
pub mod string_placements;
pub mod switch;

/// `Switcher` is a trait for objects that can be compared with a global signal,
/// switch the global signal to themselves, and provide a hashed value for equality checks.
pub trait Switcher {
    /// Compares the object with the global signal.
    /// Returns `true` if they are the same, `false` otherwise.
    fn compare(&self) -> bool;

    /// Changes the global signal to match the object.
    fn switch_active_to_self(&self);

    /// Returns a hashed value of the object for use in equality checks.
    fn hashed_value(&self) -> u64;
}

impl<'a, 'b> PartialEq<dyn Switcher + 'b> for dyn Switcher + 'a {
    fn eq(&self, other: &(dyn Switcher + 'b)) -> bool {
        self.hashed_value() == other.hashed_value()
    }
}
