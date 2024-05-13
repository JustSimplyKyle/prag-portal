use dioxus::prelude::*;
use rust_lib::api::shared_resources::collection::Collection;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use crate::{
    BaseComponents::{Button, ContentType, FillMode, Roundness, Switcher},
    MainPage::{CollectionBlock, COLLECTION_PIC},
    EXPLORE,
};

pub(crate) type Comparison<T> = (T, Option<T>);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum CollectionDisplayTopSelection {
    Mods,
    World,
    ResourcePack,
    ShaderPacks,
}

impl Switcher for CollectionDisplayTopSelection {
    fn hashed_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn compare(&self) -> bool {
        let top = use_context::<Signal<Comparison<Self>>>();
        &top().0 == self
    }

    fn switch_active_to_self(&self) {
        let mut global = use_context::<Signal<Comparison<Self>>>();
        let prev = global().0;
        if &prev != self {
            global.write().1 = Some(prev);
        }
        global.write().0 = *self;
    }
}

#[component]
pub(crate) fn CollectionDisplay(collection: ReadOnlySignal<Collection>) -> Element {
    let _: Signal<Comparison<CollectionDisplayTopSelection>> =
        use_context_provider(|| Signal::new((CollectionDisplayTopSelection::Mods, None)));
    rsx! {
        div { class: "flex flex-col",
            div { class: "sticky top-0 p-[50px] rounded-2xl bg-slate-800 grid grid-flow-col items-stretch",
                div { class: "flex flex-col space-y-[35px]",
                    div { class: "text-white font-black text-[80px] leading-normal capsize",
                        {collection().display_name}
                    }
                    Button {
                        roundness: Roundness::Pill,
                        string_placements: vec![ContentType::text("F").css("w-[30px] h-[30px]").align_center()],
                        fill_mode: FillMode::Fit,
                        extended_css_class: "w-fit shadow p-[13px]"
                    }
                }
                div { class: "flex justify-end",
                    div { class: "flex flex-col space-y-[3px] w-full max-w-[250px]",
                        CollectionBlock {
                            collection: collection(),
                            extended_class: "rounded-[20px] w-full h-[250px]",
                            picture: COLLECTION_PIC,
                            gradient: false
                        }
                        div { class: "flex space-x-[3px] min-w-full",
                            Button {
                                roundness: Roundness::None,
                                string_placements: vec![ContentType::text("s").align_center()],
                                fill_mode: FillMode::Fill,
                                extended_css_class: "rounded-[5px] rounded-bl-[20px] flex-1 min-w-0 bg-lime-300"
                            }
                            Button {
                                roundness: Roundness::None,
                                string_placements: vec![ContentType::text("...").align_center()],
                                fill_mode: FillMode::Fit,
                                extended_css_class: "rounded-[5px] rounded-br-[20px] bg-white/10 backdrop-blur-[100px] flex-none"
                            }
                        }
                    }
                }
            }
            div { class: "px-[30px] bg-background rounded-2xl min-h-dvh scroll-smooth",
                div { class: "bg-background flex justify-center items-center min-h-full py-[30px]",
                    {ContentType::svg(manganis::mg!(file("public/Line 155.svg"))).get_element()}
                }
                div { class: "grid grid-flow-col items-stretch",
                    div { class: "bg-deep-background rounded-full flex justify-start w-fit",
                        Button {
                            roundness: Roundness::Pill,
                            fill_mode: FillMode::Fit,
                            signal: Rc::new(CollectionDisplayTopSelection::Mods) as Rc<dyn Switcher>,
                            string_placements: vec![ContentType::text("A").align_left(), ContentType::text("模組").align_right()]
                        }
                        Button {
                            roundness: Roundness::Pill,
                            fill_mode: FillMode::Fit,
                            signal: Rc::new(CollectionDisplayTopSelection::World) as Rc<dyn Switcher>,
                            string_placements: vec![ContentType::text("B").align_left(), ContentType::text("世界").align_right()]
                        }
                        Button {
                            roundness: Roundness::Pill,
                            fill_mode: FillMode::Fit,
                            signal: Rc::new(CollectionDisplayTopSelection::ResourcePack) as Rc<dyn Switcher>,
                            string_placements: vec![
                                ContentType::text("C").align_left(),
                                ContentType::text("資源包").align_right(),
                            ]
                        }
                        Button {
                            roundness: Roundness::Pill,
                            fill_mode: FillMode::Fit,
                            signal: Rc::new(CollectionDisplayTopSelection::ShaderPacks) as Rc<dyn Switcher>,
                            string_placements: vec![
                                ContentType::text("D").align_left(),
                                ContentType::text("光影包").align_right(),
                            ]
                        }
                    }
                    div { class: "flex items-center space-x-[7px] h-[55px] *:h-full justify-end",
                        Button {
                            roundness: Roundness::Pill,
                            string_placements: vec![
                                ContentType::svg(EXPLORE)
                                    .css("svg-[25px]")
                                    .align_center(),
                            ],
                            fill_mode: FillMode::Fit,
                            extended_css_class: "px-[25px]"
                        }
                        Button {
                            roundness: Roundness::Pill,
                            string_placements: vec![ContentType::text("F").css("w-[25px] h-[25px]").align_center()],
                            fill_mode: FillMode::Fit,
                            extended_css_class: "px-[25px]"
                        }
                    }
                }
            }
        }
    }
}
