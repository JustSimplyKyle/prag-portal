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

/// the first value of the tuple is what the selected value is, and the second is the previous value
pub(crate) type Comparison<T> = (T, Option<T>);

#[macro_export]
macro_rules! impl_context_switcher {
    ($type:ty) => {
        impl Switcher for $type {
            fn hashed_value(&self) -> u64 {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                self.hash(&mut hasher);
                hasher.finish()
            }

            fn compare(&self) -> bool {
                use dioxus::signals::Readable;
                let top = use_context::<
                    dioxus::signals::Signal<crate::collection_display::Comparison<Self>>,
                >();
                let x = &top.read().0 == self;
                x
            }

            fn switch_active_to_self(&self) {
                use dioxus::signals::Writable;
                let mut global = use_context::<
                    dioxus::signals::Signal<crate::collection_display::Comparison<Self>>,
                >();
                let prev = global().0;
                if &prev != self {
                    global.write().1 = Some(prev);
                }
                global.write().0 = *self;
            }
        }
        impl_optional_switcher!($type);
    };
}

#[macro_export]
macro_rules! impl_optional_switcher {
    ($type:ty) => {
        impl From<$type> for Option<std::rc::Rc<dyn Switcher>> {
            fn from(value: $type) -> Self {
                Some(std::rc::Rc::new(value) as std::rc::Rc<dyn Switcher>)
            }
        }
    };
}
