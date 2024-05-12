use dioxus::prelude::*;
use manganis::ImageAsset;
use rust_lib::api::shared_resources::entry::STORAGE;

use crate::{
    BaseComponents::{
        Alignment, Button, ContentType, Contents, FillMode, Roundness, Size, SVG_CSS,
    },
    MainPage::{CollectionBlock, COLLECTION_PIC, STAR},
    EXPLORE,
};

pub static NOTE: &str = manganis::mg!(file("./public/note_stack_add.svg"));
pub static BOOKMARK: &str = manganis::mg!(file("./public/bookmark.svg"));
pub static BOOKMARK_ADD: &str = manganis::mg!(file("./public/bookmark_add.svg"));

#[component]
pub fn Collections() -> Element {
    let collections =
        use_resource(
            move || async move { STORAGE.collections.clone().read_owned().await.to_owned() },
        );
    let collections_iterator = collections().into_iter().flat_map(|x| x.into_iter());

    rsx! {
        div { class: "flex flex-col space-y-[10px]",
            div { class: "grid grid-flow-col justify-stretch items-center h-[55px]",
                div { class: "flex h-full items-center space-x-[10px]",
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![
                            ContentType::svg(EXPLORE)
                                .css(SVG_CSS)
                                .align_left(),
                            ContentType::text("全部").align_right(),
                        ],
                        extended_css_class: "pl-[20px] pr-[25px] h-full"
                    }
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![
                            ContentType::svg(STAR).css(SVG_CSS).align_center(),
                        ],
                        extended_css_class: "px-[20px] h-full"
                    }
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![
                            ContentType::svg(BOOKMARK)
                                .css(SVG_CSS)
                                .align_left(),
                            ContentType::text("分類夾").align_right(),
                        ],
                        extended_css_class: "px-[20px] h-full my-0"
                    }
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![
                            ContentType::svg(BOOKMARK_ADD).css(SVG_CSS).align_center(),
                        ],
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
                            ContentType::text("Condition").align_left(),
                            ContentType::text("DOWN").align_right(),
                        ],
                        extended_css_class: "pl-[20px] pr-[10px] h-full"
                    }
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![
                            ContentType::text("Fullscreen").align_center(),
                            ContentType::text("DOWN").align_right(),
                        ],
                        extended_css_class: "pl-[20px] pr-[10px] h-full"
                    }
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![
                            ContentType::svg(NOTE).css(SVG_CSS).align_center(),
                        ],
                        extended_css_class: "hover:bg-green px-[20px] h-full"
                    }
                    Button {
                        roundness: Roundness::Pill,
                        fill_mode: FillMode::Fit,
                        size: Size::Medium,
                        string_placements: vec![
                            ContentType::text("Search").align_center(),
                            ContentType::text("DOWN").align_right(),
                        ],
                        extended_css_class: "pl-[20px] pr-[10px] h-full"
                    }
                }
            }
            div { class: "grid grid-flow-row grid-cols-[repeat(auto-fill,280px)] gap-[20px]",
                for collection in collections_iterator {
                    CollectionBlock { collection, extended_class: "rounded-[20px]", picture: COLLECTION_PIC }
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
