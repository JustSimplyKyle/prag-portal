use crate::BaseComponents::atoms::button::{Button, FillMode, Roundness};
use crate::BaseComponents::string_placements::StringPlacements;
use dioxus::prelude::*;
use std::rc::Rc;
use strum::IntoEnumIterator;

/// `Switcher` is a trait for objects that can be compared with a global signal,
/// switch the global signal to themselves, and provide a hashed value for equality checks.
pub trait StateSwitcher {
    /// Compares the object with the global signal.
    /// Returns `true` if they are the same, `false` otherwise.
    fn compare(&self) -> bool;

    /// Changes the global signal to match the object.
    fn switch_active_to_self(&self);

    /// Returns a hashed value of the object for use in equality checks.
    fn hashed_value(&self) -> u64;
}

impl<'a> PartialEq<dyn StateSwitcher + 'a> for dyn StateSwitcher {
    fn eq(&self, other: &(dyn StateSwitcher + 'a)) -> bool {
        self.hashed_value() == other.hashed_value()
    }
}

/// the first value of the tuple is what the selected value is, and the second is the previous value
pub(crate) type Comparison<T> = (T, Option<T>);

/// Requires `Hash`, `PartialEq` and `Clone` implemented
/// provides the contect `Signal<Comparison<Self>>`
#[macro_export]
macro_rules! impl_context_switcher {
    ($type:ty) => {
        impl $crate::BaseComponents::molecules::switcher::StateSwitcher for $type {
            fn hashed_value(&self) -> u64 {
                use std::hash::{Hash, Hasher};
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                self.hash(&mut hasher);
                hasher.finish()
            }

            fn compare(&self) -> bool {
                use dioxus::signals::Readable;
                let top = use_context::<
                    dioxus::signals::Signal<
                        $crate::BaseComponents::molecules::switcher::Comparison<Self>,
                    >,
                >();
                let x = &top.read().0 == self;
                x
            }

            fn switch_active_to_self(&self) {
                use dioxus::signals::Writable;
                let mut global = use_context::<
                    dioxus::signals::Signal<
                        $crate::BaseComponents::molecules::switcher::Comparison<Self>,
                    >,
                >();
                let prev = global().0;
                if &prev != self {
                    global.write().1 = Some(prev);
                }
                global.write().0 = self.clone();
            }
        }
        $crate::impl_optional_state_switcher!($type);
    };
}

#[macro_export]
macro_rules! impl_optional_state_switcher {
    ($type:ty) => {
        impl From<$type>
            for Option<std::rc::Rc<dyn $crate::BaseComponents::molecules::switcher::StateSwitcher>>
        {
            fn from(value: $type) -> Self {
                Some(std::rc::Rc::new(value)
                    as std::rc::Rc<
                        dyn $crate::BaseComponents::molecules::switcher::StateSwitcher,
                    >)
            }
        }
    };
}

pub trait ToClass {
    fn to_class(&self) -> String {
        String::new()
    }
}

/// A component for creating a selection bar that switches between different states.
///
/// # Parameters
///
/// - `default_state`: The initial state of the switcher.
/// - `class`: Additional CSS classes to be applied to the root of the component.
/// - `signal`: An optional signal to synchronize the state externally.
///
/// # Type Constraints
///
/// - `T`: A type that implements the `StateSwitcher`, `IntoEnumIterator`, `Clone`.
///         It must also be `'static` and implement `PartialEq`.
/// - `T` must implement `ToClass`, which functionality is providing each Element's class.
/// - `T` must be convertible into `Option<Rc<dyn StateSwitcher + 'static>>`.
/// - `T` must be convertible into `StringPlacements`.
///
/// # Example
///
/// ```rust
/// #[derive(PartialEq, Eq, Clone, Copy, Hash, EnumIter, Debug)]
/// pub enum ProgressState {
///     Running,
///     Finished,
/// }
///
/// impl From<ProgressState> for StringPlacements {
///     fn from(value: ProgressState) -> Self {
///         match value {
///             ProgressState::Running => vec![
///                 ContentType::text("K").align_left(),
///                 ContentType::text("正在進行").align_right(),
///             ],
///             ProgressState::Finished => vec![
///                 ContentType::text("K").align_left(),
///                 ContentType::text("已完成").align_right(),
///             ],
///         }
///         .into()
///     }
/// }
///
/// impl ToClass for ProgressState {
///     fn to_class(&self) -> String {
///         match self {
///             ProgressState::Running => "bg-something",
///             ProgressState::Finished => "bg-you-know",
///         }
///         .into()
///     }
/// }
///
/// impl_context_switcher!(ProgressState);
/// // Usage in rsx!
/// rsx! {
///     StateSwitcherSelectionBar {
///         default_state: ProgressState::Running
///     }
/// }
/// ```
#[allow(clippy::multiple_bound_locations)]
#[component]
pub fn StateSwitcherSelectionBar<T: 'static + PartialEq>(
    default_state: T,
    #[props(default)] class: String,
    signal: Option<Signal<Comparison<T>>>,
) -> Element
where
    T: StateSwitcher + IntoEnumIterator + Clone + ToClass,
    Option<Rc<(dyn StateSwitcher + 'static)>>: From<T>,
    StringPlacements: From<T>,
{
    let comp: Signal<Comparison<T>> = use_context_provider(|| Signal::new((default_state, None)));
    use_effect(move || {
        if let Some(x) = signal.as_mut() {
            x.set(comp());
        }
    });
    rsx! {
        div {
            class: tailwind_fuse::tw_merge!("w-fit flex gap-0 rounded-full bg-deep-background", class),
            for selected in T::iter() {
                Button {
                    roundness: Roundness::Pill,
                    string_placements: Into::<StringPlacements>::into(selected.clone()),
                    switcher: selected,
                    focus_color_change: true,
                    fill_mode: FillMode::Fit,
                }
            }
        }
    }
}
