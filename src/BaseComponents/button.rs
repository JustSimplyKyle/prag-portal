use std::rc::Rc;

use dioxus::prelude::*;
use tailwind_fuse::*;

use super::{string_placements::StringPlacements, Switcher};

#[derive(TwClass, Clone, Copy)]
#[tw(
    class = "transition-all ease-in-out drop-shadow-lg duration-300 text-white bg-deep-background items-center"
)]
pub struct ButtonClass {
    pub roundness: Roundness,
    pub items_count: ItemsCount,
    pub size: Size,
    pub fill_mode: FillMode,
}

impl ButtonClass {
    // automatically roundness correction
    const fn setup(&self) -> &str {
        match self.items_count {
            ItemsCount::One => "",
            ItemsCount::AboveOne => match self.roundness {
                Roundness::Top | Roundness::None | Roundness::Bottom => "pr-8",
                Roundness::Pill => match self.size {
                    Size::Fat => "",
                    Size::Medium => "pr-[25px]",
                    Size::Small => "pr-[5px]",
                },
            },
        }
    }
    #[must_use]
    pub fn to_class(&self) -> String {
        tw_merge!(IntoTailwindClass::to_class(self), self.setup())
    }
    #[must_use]
    pub fn with_class(&self, string: impl AsRef<str>) -> String {
        let class = IntoTailwindClass::with_class(self, string);
        tw_merge!(class, self.setup())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, TwVariant)]
pub enum FillMode {
    #[tw(default, class = "min-w-full")]
    Fill,
    #[tw(class = "min-w-fit space-x-2.5")]
    Fit,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, TwVariant)]
pub enum Size {
    #[tw(default, class = "text-2xl p-5 font-bold")]
    Fat,
    #[tw(class = "pl-[20px] py-[12px] text-lg")]
    Medium,
    #[tw(class = "py-[5px] px-[20px] text-[17px]")]
    Small,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, TwVariant)]
pub enum ItemsCount {
    #[tw(class = "flex justify-center items-center")]
    One,
    #[tw(default, class = "grid grid-flow-col justify-stretch items-center")]
    AboveOne,
}

impl From<usize> for ItemsCount {
    fn from(value: usize) -> Self {
        if value == 1 {
            Self::One
        } else {
            Self::AboveOne
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, TwVariant)]
pub enum Roundness {
    #[tw(class = "rounded-t-3xl")]
    Top,
    #[tw(default, class = "")]
    None,
    #[tw(class = "rounded-b-3xl")]
    Bottom,
    #[tw(class = "rounded-full")]
    Pill,
}

#[component]
pub fn Button(
    roundness: Roundness,
    #[props(into)] string_placements: StringPlacements,
    #[props(default)] extended_css_class: String,
    signal: Option<Rc<dyn Switcher>>,
    #[props(default = true)] clickable: bool,
    #[props(into)] onclick: Option<EventHandler>,
    #[props(extends = GlobalAttributes, extends = div)] mut attributes: Vec<Attribute>,
    #[props(default)] size: Size,
    #[props(default)] fill_mode: FillMode,
    #[props(default = false)] focus_color_change: bool,
) -> Element {
    attributes.retain(|x| x.name != "class");
    let class = ButtonClass {
        roundness,
        items_count: string_placements.len().into(),
        size,
        fill_mode,
    }
    .with_class(if focus_color_change {
        "aria-selected:bg-white aria-selected:text-black"
    } else {
        ""
    });
    let mut clickiness = use_signal(|| false);
    let class = tw_merge!(class, extended_css_class);
    rsx! {
        div {
            class,
            role: if clickable { "button" } else { "" },
            aria_selected: {
                if let Some(x) = signal.as_ref() {
                    x.compare()
                } else {
                    focus_color_change && clickiness()
                }
            },
            onclick: move |_| {
                if signal.is_none() && focus_color_change {
                    clickiness.toggle();
                }
                if let Some(x) = onclick {
                    x(());
                } else if let Some(x) = &mut signal {
                    x.switch_active_to_self();
                }
            },
            ..attributes,
            {
                match string_placements {
                    StringPlacements::Designed(s) => rsx! {
                        for x in s {
                            { x }
                        }
                    },
                    StringPlacements::Custom(x) => x,
                }
            }
        }
    }
}
