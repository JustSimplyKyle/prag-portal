use dioxus::prelude::*;
use rust_lib::api::shared_resources::collection::CollectionId;

use crate::{
    main_page::{CollectionBlock, STAR},
    BaseComponents::{
        atoms::button::{Button, FillMode, Roundness, Size},
        molecules::{
            context_menu::{self, ContextMenu},
            search_bar::SearchBar,
        },
        string_placements::{Alignment, ContentType, Contents, Hint, Text},
    },
    EXPLORE,
};

pub static NOTE: &str = manganis::mg!("./public/note_stack_add.svg");
pub static CROP_FREE: &str = manganis::mg!("./public/crop_free.svg");
pub static FILTER_LIST: &str = manganis::mg!("./public/filter_list.svg");
pub static SEARCH: &str = manganis::mg!("./public/search.svg");
pub static ARROW_DOWN: &str = manganis::mg!("./public/arrow_drop_down.svg");
pub static BOOKMARK: &str = manganis::mg!("./public/bookmark.svg");
pub static BOOKMARK_ADD: &str = manganis::mg!("./public/bookmark_add.svg");

#[component]
pub fn CollectionContext() -> Element {
    use context_menu::Direction::DownRightEdge;
    let extended_css_class = "bg-background pl-[15px] py-[15px] pr-[20px] gap-[10px]";
    let text = "text-[16px] font-medium";
    rsx! {
        ContextMenu {
            direction: DownRightEdge,
            div {
                class: "relative overflow-hidden flex bg-background flex-col items-center",
                Button {
                    extended_css_class,
                    string_placements: vec![
                        ContentType::text("H").align_left(),
                        ContentType::hint("開始遊戲").css(text).align_left()
                    ]
                }
                Button {
                    extended_css_class,
                    string_placements: vec![
                        ContentType::text("H").align_left(),
                        ContentType::hint("更改資料夾").css(text).align_left()
                    ]
                }
                Button {
                    extended_css_class,
                    string_placements: vec![
                        ContentType::text("H").align_left(),
                        ContentType::hint("檢視詳情").css(text).align_left()
                    ]
                }
                Button {
                    extended_css_class,
                    string_placements: vec![
                        ContentType::text("H").align_left(),
                        ContentType::hint("風格化選項").css(text).align_left()
                    ]
                }
                Button {
                    extended_css_class,
                    string_placements: vec![
                        ContentType::text("H").align_left(),
                        ContentType::hint("瀏覽本機資料夾").css(text).align_left()
                    ]
                }
                Button {
                    extended_css_class,
                    string_placements: vec![
                        ContentType::text("H").align_left(),
                        ContentType::hint("匯出/分享合集").css(text).align_left()
                    ]
                }
                Button {
                    extended_css_class,
                    string_placements: vec![
                        ContentType::text("H").align_left(),
                        ContentType::hint("複製合集").css(text).align_left()
                    ]
                }
                Button {
                    extended_css_class,
                    string_placements: vec![
                        ContentType::text("H").align_left(),
                        ContentType::hint("刪除合集").css(text).align_left()
                    ]
                }
            }
        }
    }
}

#[component]
pub fn Collections() -> Element {
    let keys = use_context::<Memo<Vec<CollectionId>>>();
    let binding = keys.read();
    let keys_iter = binding.iter().zip((0..keys.len()).rev());
    let sender = use_signal(String::new);
    rsx! {
        div {
            class: "flex z-10 flex-col space-y-[10px]",
            div {
                class: "grid grid-flow-col justify-stretch items-center h-[55px]",
                div {
                    class: "flex h-full items-center space-x-[10px]",
                    div {
                        class: "relative z-10 py-[10px] pl-[20px] pr-[25px] bg-deep-background p-0 rounded-full w-fit grid grid-flow-col items-center justify-stretch",
                        {ContentType::svg(EXPLORE).css("justify-self-start svg-[30px]")}
                        Text {
                            css: "justify-self-end text-lg",
                            "全部"
                        }
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
                        extended_css_class: "px-[25px] h-full"
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
                class: "grid grid-flow-row grid-cols-[repeat(auto-fill,280px)] gap-[20px] overflow-y-visible",
                for (collection_id, i) in keys_iter {
                    CollectionBlock {
                        collection_id: collection_id.clone(),
                        style: "z-index:{i}",
                        extended_class: "rounded-[20px] overflow-y-visible",
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
                extended_css_class: "z-0 rounded-[20px] px-[40px] py-[50px]",
                size: Size::Fat,
                clickable: false
            }
        }
    }
}
