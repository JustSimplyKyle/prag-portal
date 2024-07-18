use dioxus::prelude::*;
use rust_lib::api::shared_resources::entry::STORAGE;

use crate::{
    main_page::{CollectionBlock, STAR},
    BaseComponents::{
        atoms::button::{Button, FillMode, Roundness, Size},
        molecules::search_bar::SearchBar,
        string_placements::{Alignment, ContentType, Contents},
    },
    EXPLORE,
};

pub static NOTE: &str = manganis::mg!(file("./public/note_stack_add.svg"));
pub static CROP_FREE: &str = manganis::mg!(file("./public/crop_free.svg"));
pub static FILTER_LIST: &str = manganis::mg!(file("./public/filter_list.svg"));
pub static SEARCH: &str = manganis::mg!(file("./public/search.svg"));
pub static ARROW_DOWN: &str = manganis::mg!(file("./public/arrow_drop_down.svg"));
pub static BOOKMARK: &str = manganis::mg!(file("./public/bookmark.svg"));
pub static BOOKMARK_ADD: &str = manganis::mg!(file("./public/bookmark_add.svg"));

#[component]
pub fn Collections() -> Element {
    let read = STORAGE.collections.read();
    let collection_ids = read.keys().cloned();
    let sender = use_signal(String::new);
    rsx! {
        div {
            class: "flex flex-col space-y-[10px]",
            div {
                class: "grid grid-flow-col justify-stretch items-center h-[55px]",
                div {
                    class: "flex h-full items-center space-x-[10px]",
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![
                            ContentType::svg(EXPLORE).css("svg-[30px]").align_left(),
                            ContentType::text("全部").align_right(),
                        ],
                        extended_css_class: "pl-[20px] pr-[25px] h-full"
                    }
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![ContentType::svg(STAR).css("svg-[30px]").align_center()],
                        extended_css_class: "px-[20px] h-full"
                    }
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![
                            ContentType::svg(BOOKMARK).css("svg-[30px]").align_left(),
                            ContentType::text("分類夾").align_right(),
                        ],
                        extended_css_class: "px-[20px] h-full my-0"
                    }
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![ContentType::svg(BOOKMARK_ADD).css("svg-[30px]").align_center()],
                        extended_css_class: "pl-[20px] pr-[25px] h-full"
                    }
                }
                div {
                    class: "flex h-full items-center gap-[10px] flex-row-reverse justify-self-end",
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![
                            Contents::new(
                                    vec![
                                        ContentType::svg(FILTER_LIST).css("svg-[25px]"),
                                        ContentType::svg(ARROW_DOWN).css("svg-[20px]"),
                                    ],
                                    Alignment::Center,
                                )
                                .css("gap-[5px]"),
                        ],
                        extended_css_class: "pl-[20px] pr-[10px] h-full"
                    }
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![
                            Contents::new(
                                    vec![
                                        ContentType::svg(CROP_FREE).css("svg-[25px]"),
                                        ContentType::svg(ARROW_DOWN).css("svg-[20px]"),
                                    ],
                                    Alignment::Center,
                                )
                                .css("gap-[5px]"),
                        ],
                        extended_css_class: "pl-[20px] pr-[10px] h-full"
                    }
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![ContentType::svg(NOTE).css("svg-[30px]").align_center()],
                        extended_css_class: "hover:bg-green px-[20px] h-full"
                    }
                    SearchBar {
                        sender
                    }
                }
            }
            div {
                class: "grid grid-flow-row grid-cols-[repeat(auto-fill,280px)] gap-[20px]",
                for collection_id in collection_ids {
                    CollectionBlock {
                        collection_id,
                        extended_class: "rounded-[20px]"
                    }
                }
            }
        }
        div {
            Button {
                roundness: Roundness::None,
                string_placements: vec![
                    Contents::new(
                            [
                                ContentType::text("新增更多收藏").css("text-[35px]"),
                                ContentType::hint(
                                    "透過探索功能下載社群收藏或是由你開始建立",
                                ),
                            ],
                            Alignment::Left,
                        )
                        .css("flex flex-col gap-[15px]"),
                    ContentType::text("F").align_right(),
                ],
                extended_css_class: "rounded-[20px] px-[40px] py-[50px]",
                size: Size::Fat,
                clickable: false
            }
        }
    }
}
