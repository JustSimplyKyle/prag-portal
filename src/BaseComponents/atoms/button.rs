use std::rc::Rc;

use dioxus::prelude::*;
use tailwind_fuse::*;

use super::super::{molecules::switcher::StateSwitcher, string_placements::StringPlacements};

#[derive(TwClass, Clone, Copy)]
#[tw(
    class = "transition-all ease-in-out drop-shadow-lg duration-300 text-white bg-deep-background grid grid-flow-col justify-stretch items-center"
)]
pub struct ButtonClass {
    pub roundness: Roundness,
    pub size: Size,
    pub fill_mode: FillMode,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, TwVariant)]
pub enum FillMode {
    #[tw(default, class = "min-w-full")]
    Fill,
    #[tw(class = "min-w-fit space-x-2.5")]
    Fit,
    #[tw(class = "")]
    None,
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
pub enum Roundness {
    #[tw(class = "rounded-t-3xl")]
    Top,
    #[tw(default, class = "")]
    None,
    #[tw(class = "rounded-b-3xl")]
    Bottom,
    #[tw(class = "rounded-full")]
    Pill,
    #[tw(class = "rounded-[30px]")]
    Squircle,
}

#[component]
pub fn Button(
    roundness: Option<Roundness>,
    #[props(into)] string_placements: StringPlacements,
    #[props(default)] extended_css_class: String,
    #[props(default)] style: String,
    #[props(into)] switcher: Option<Rc<dyn StateSwitcher>>,
    #[props(default = true)] clickable: bool,
    #[props(into)] onclick: Option<EventHandler>,
    #[props(into)] onmouseover: Option<EventHandler>,
    #[props(into)] onmouseleave: Option<EventHandler>,
    #[props(extends = GlobalAttributes, extends = div)] mut attributes: Vec<Attribute>,
    #[props(default)] size: Size,
    #[props(default)] fill_mode: FillMode,
    #[props(default = false)] focus_color_change: bool,
) -> Element {
    let roundness = roundness.unwrap_or(Roundness::None);
    attributes.retain(|x| x.name != "class");
    let class = ButtonClass {
        roundness,
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
            style,
            role: if clickable { "button" } else { "" },
            aria_selected: {
                if let Some(x) = switcher.as_ref() {
                    x.compare()
                } else {
                    focus_color_change && clickiness()
                }
            },
            onclick: move |_| {
                if switcher.is_none() && focus_color_change {
                    clickiness.toggle();
                }
                if let Some(x) = onclick {
                    x(());
                }
                if let Some(x) = &mut switcher {
                    x.switch_active_to_self();
                }
            },
            onmouseover: move |_| {
                if let Some(x) = onmouseover {
                    x(());
                }
            },
            onmouseleave: move |_| {
                if let Some(x) = onmouseleave {
                    x(());
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
