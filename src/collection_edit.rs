pub mod personalization;
pub mod sidebar;

use dioxus::prelude::*;
use personalization::Personalization;
use rust_lib::api::shared_resources::{collection::CollectionId, entry::STORAGE};
use sidebar::EditSidebar;
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    collection_display::GAME_CONTROLLER,
    impl_context_switcher, impl_optional_state_switcher,
    pages::Pages,
    scrollable::Scrollable,
    use_error_handler,
    BaseComponents::{
        atoms::button::{Button, Roundness},
        molecules::switcher::Comparison,
        string_placements::{Alignment, ContentType, Contents},
    },
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
                Self::Personalization => "personalization",
                Self::DataLog => "datalog",
                Self::Export => "export",
                Self::Advanced => "advanced",
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
    let collection_ids = binding
        .keys()
        .map(|x| (x.clone(), Pages::collection_edit(x.clone())));
    rsx! {
        for (collection_id , page) in collection_ids {
            div {
                class: "absolute inset-0 z-0 min-w-full min-h-full",
                id: page.slide_in_id(),
                if page.should_render() {
                    CollectionEdit {
                        key: "{page.slide_in_id()}",
                        collection_id
                    }
                }
            }
        }
    }
}

#[component]
fn CollectionEdit(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let edit_state: Signal<Comparison<EditState>> =
        use_context_provider(|| Signal::new((EditState::Personalization, None)));
    let mut error_handler = use_error_handler();
    use_effect(move || {
        let vec = EditState::iter().collect::<Vec<_>>();
        let error = EditState::scroller_applyer(vec, |x| &edit_state.read().0 == x);
        error_handler.set(Some(error));
    });
    rsx! {
        div {
            class: "flex w-full bg-deep-background group-edit min-h-screen gap-[20px] rounded-[5px] px-[20px] pb-[20px]",
            "data-prev": edit_state().1.map_or_else(String::new, |x| x.to_string()),
            EditSidebar {
                collection_id
            }
            div {
                class: "w-full min-h-screen relative *:overflow-scroll",
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::Personalization.scroller_id(),
                    Personalization {
                        collection_id
                    }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::DataLog.scroller_id(),
                    DataLog {
                        collection_id
                    }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::Export.scroller_id(),
                    Export {
                        collection_id
                    }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::Advanced.scroller_id(),
                    Advanced {
                        collection_id
                    }
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
                        class: "bg-background py-[10px] rounded-t-[30px]"
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
                                    ContentType::text("收藏紀錄")
                                        .css("font-black text-white text-[40px]"),
                                    ContentType::hint("查看這個收藏的資訊"),
                                ],
                                Alignment::Left,
                            )
                            .css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right(),
                    ]
                }
            }
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
                                    ContentType::hint("分享你的收藏或是將它匯出至電腦"),
                                ],
                                Alignment::Left,
                            )
                            .css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right(),
                    ]
                }
            }
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
                                    ContentType::text("進階選項")
                                        .css("font-black text-white text-[40px]"),
                                    ContentType::hint("單獨修改此收藏的進階選項"),
                                ],
                                Alignment::Left,
                            )
                            .css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right(),
                    ]
                }
            }
        }
    }
}
