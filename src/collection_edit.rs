use dioxus::prelude::*;
use rust_lib::api::shared_resources::{collection::CollectionId, entry::STORAGE};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    collection_display::{DISPLAY_BACKGROUND, GAME_CONTROLLER, UNDO},
    impl_context_switcher, impl_optional_state_switcher,
    main_page::ARROW_LEFT,
    pages::Pages,
    scrollable::Scrollable,
    BaseComponents::{
        atoms::button::{Button, FillMode, Roundness},
        molecules::switcher::{Comparison, StateSwitcher},
        string_placements::{Alignment, ContentType, Contents},
    },
    ARROW_RIGHT,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, EnumIter)]
pub enum EditState {
    Personalization,
    DataLog,
    Export,
    Advanced,
}

impl std::fmt::Display for EditState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "edit-{}",
            match self {
                EditState::Personalization => "personalization",
                EditState::DataLog => "datalog",
                EditState::Export => "export",
                EditState::Advanced => "advanced",
            }
        )
    }
}

impl Scrollable for EditState {
    const GROUP_SELECTOR: &'static str = "group-edit";
}

impl_context_switcher!(EditState);

impl_optional_state_switcher!(Pages);

#[component]
pub fn CollectionEditContainer() -> Element {
    let binding = STORAGE.collections.read();
    let collection_ids = binding.keys();
    rsx! {
        for collection_id in collection_ids {
            div {
                class: "absolute inset-0 z-0 min-w-full min-h-full",
                id: Pages::collection_edit(collection_id.clone()).slide_in_id(),
                    if Pages::collection_edit(collection_id.clone()).should_render() {
                    CollectionEdit {
                        collection_id: collection_id.clone()
                    }
                }
            }
        }
    }
}

#[component]
fn EditSidebar(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        div {
            class: "flex flex-col min-w-[400px] gap-[20px]",
            EditSidebarInfographic {
                collection_id
            }
            div {
                class: "flex flex-col",
                Button {
                    roundness: Roundness::Top,
                    fill_mode: FillMode::Fit,
                    extended_css_class: "bg-background",
                    focus_color_change: true,
                    switcher: EditState::Personalization,
                    string_placements: vec![
                        ContentType::text("風格化").align_left(),
                        ContentType::svg(ARROW_RIGHT).css("svg-[30px]").align_right(),
                    ]
                }
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "bg-background",
                    fill_mode: FillMode::Fit,
                    focus_color_change: true,
                    switcher: EditState::DataLog,
                    string_placements: vec![
                        ContentType::text("收藏紀錄").align_left(),
                        ContentType::svg(ARROW_RIGHT).css("svg-[30px]").align_right(),
                    ]
                }
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "bg-background",
                    fill_mode: FillMode::Fit,
                    focus_color_change: true,
                    switcher: EditState::Export,
                    string_placements: vec![
                        ContentType::text("分享&匯出").align_left(),
                        ContentType::svg(ARROW_RIGHT).css("svg-[30px]").align_right(),
                    ]
                }
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "bg-background",
                    fill_mode: FillMode::Fit,
                    focus_color_change: true,
                    switcher: EditState::Advanced,
                    string_placements: vec![
                        ContentType::text("進階選項").align_left(),
                        ContentType::svg(ARROW_RIGHT).css("svg-[30px]").align_right(),
                    ]
                }
            }
            div {
                class: "flex justify-stretch w-full gap-[10px]",
                Button {
                    roundness: Roundness::Pill,
                    onclick: move |_| {
                        Pages::collection_display(collection_id()).switch_active_to_self();
                    },
                    extended_css_class: "flex w-auto min-w-auto justify-center items-center bg-background gap-[15px] pl-[20px] pr-[30px]",
                    string_placements: vec![
                        ContentType::svg(UNDO).css("svg-[35px]").align_center(),
                        ContentType::text("返回頁面").align_center()
                    ]
                }
                Button {
                    roundness: Roundness::Pill,
                    extended_css_class: "flex w-auto min-w-auto items-center bg-background gap-[15px] pl-[20px] pr-[30px]",
                    string_placements: vec![
                        ContentType::svg(ARROW_LEFT).align_center(),
                        ContentType::text("返回頁面").align_center()
                    ]
                }
            }

        }
    }
}

#[component]
fn EditSidebarInfographic(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let read = collection_id.read();
    let collection = read.get_collection();
    rsx! {
        div {
            class: "flex flex-col w-full",
            div {
                class: "flex flex-col p-5 justify-end rounded-t-[50px] w-full h-[250px]",
                background: format!("radial-gradient(171.48% 102.52% at 0% 100%, #000 0%, rgba(0, 0, 0, 0.00) 100%), url(\"{}\") lightgray 50% / cover no-repeat", DISPLAY_BACKGROUND),
                {
                    ContentType::image(collection.picture_path.to_string_lossy().to_string()).css("w-[100px] h-[100px] bg-cover rounded-t-[50px] rounded-bl-[15px] rounded-br-[50px] p-[5px]")
                }
            }
            Button {
                roundness: Roundness::Bottom,
                extended_css_class: "bg-background justify-start px-5 pt-[22px]",
                string_placements: vec![
                    Contents::new(
                        vec![
                            ContentType::text(&collection.display_name).css("text-3xl font-balck"),
                            ContentType::hint("由我建立•18 分鐘•不久前開啟").css("font-medium text-[15px]")
                        ],
                        Alignment::Left
                    ).css("flex flex-col gap-[15px]")
                ]
            }
        }
    }
}

#[component]
fn CollectionEdit(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let edit_state: Signal<Comparison<EditState>> =
        use_context_provider(|| Signal::new((EditState::Personalization, None)));
    use_effect(move || {
        let vec = EditState::iter().collect::<Vec<_>>();
        EditState::scroller_applyer(vec, |x| &edit_state.read().0 == x).unwrap();
    });
    rsx! {
        div {
            class: "flex w-full bg-deep-background group-edit min-h-screen gap-[20px] rounded-[5px] px-[20px] pb-[20px]",
            "data-prev": edit_state().1.map_or_else(String::new, |x| x.to_string()),
            EditSidebar { collection_id }
            div {
                class: "w-full min-h-screen relative *:overflow-scroll",
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::Personalization.scroller_id(),
                    Personalization { collection_id }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::DataLog.scroller_id(),
                    DataLog { collection_id  }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::Export.scroller_id(),
                    Export { collection_id  }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::Advanced.scroller_id(),
                    Advanced { collection_id  }
                }
            }

        }
    }
}

#[component]
fn EditTemplate(children: Element, title: Element) -> Element {
    rsx! {
        div {
            class: "flex flex-col min-h-screen bg-background px-[30px] pb-[30px] rounded-[30px]",
            div {
                class: "bg-background sticky top-0 z-50",
                div {
                    class: "flex flex-col bg-background pt-[30px] rounded-b-[30px]",
                    {title}
                    div {
                        class: "bg-background py-[10px] rounded-t-[30px]",
                    }
                }
            }
            div {
                class: "flex flex-col overflow-scroll *:z-10 gap-[20px]",
                {children}
            }
        }
    }
}

#[component]
fn Personalization(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        EditTemplate {
            title: rsx! {
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "rounded-[20px] p-[40px] gap-[20px]",
                    string_placements: vec![
                        Contents::new(
                            vec![
                                ContentType::text("風格化").css("font-black text-white text-[40px]"),
                                ContentType::hint("自訂你的收藏樣式")
                            ],
                            Alignment::Left
                        ).css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right()
                    ]
                }
            },
            for _ in 0..10 {
                Button {
                    roundness: Roundness::Pill,
                    extended_css_class: "rounded-[20px] p-[40px]",
                    string_placements: vec![
                        Contents::new(
                            vec![
                                ContentType::text("not").css("font-black text-white text-[40px]"),
                                ContentType::hint("自訂你的收藏樣式")
                            ],
                            Alignment::Left
                        ).css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right()
                    ]
                }
            }
        }
    }
}

#[component]
fn DataLog(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        EditTemplate {
            title: rsx! {
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "rounded-[20px] p-[40px]",
                    string_placements: vec![
                        Contents::new(
                            vec![
                                ContentType::text("收藏紀錄").css("font-black text-white text-[40px]"),
                                ContentType::hint("查看這個收藏的資訊")
                            ],
                            Alignment::Left
                        ).css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right()
                    ]
                }
            },
        }
    }
}

#[component]
fn Export(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        EditTemplate {
            title: rsx! {
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "rounded-[20px] p-[40px]",
                    string_placements: vec![
                        Contents::new(
                            vec![
                                ContentType::text("分享").css("font-black text-white text-[40px]"),
                                ContentType::hint("分享你的收藏或是將它匯出至電腦")
                            ],
                            Alignment::Left
                        ).css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right()
                    ]
                }
            },
        }
    }
}

#[component]
fn Advanced(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        EditTemplate {
            title: rsx! {
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "rounded-[20px] p-[40px]",
                    string_placements: vec![
                        Contents::new(
                            vec![
                                ContentType::text("進階選項").css("font-black text-white text-[40px]"),
                                ContentType::hint("單獨修改此收藏的進階選項")
                            ],
                            Alignment::Left
                        ).css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right()
                    ]
                }
            }
        }
    }
}
