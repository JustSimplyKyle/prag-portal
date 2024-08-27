use dioxus::prelude::*;
use rust_lib::api::shared_resources::entry::STORAGE;
use tailwind_fuse::tw_merge;

use crate::{
    main_page::{use_delayed_hover, CollectionBlock, STAR},
    pages::Pages,
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
fn TopBar() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-[5px]",
        }
    }
}

#[component]
pub fn ALPHABETICAL() -> Element {
    rsx! {
         svg { "viewBox": "0 0 40 40", height: "40", fill: "none", width: "40", xmlns: "http://www.w3.org/2000/svg", mask { x: "0", y: "0", width: "40", height: "40", style: "mask-type:alpha", "maskUnits": "userSpaceOnUse", id: "mask0_3518_49973", rect { height: "40", width: "40", fill: "#D9D9D9" } } g { mask: "url(#mask0_3518_49973)", path { fill: "white", d: "M8.82165 24.1213L7.81665 26.9163C7.69499 27.2202 7.51277 27.4553 7.26999 27.6217C7.02749 27.7881 6.73999 27.8713 6.40749 27.8713C5.84665 27.8713 5.42235 27.6393 5.13457 27.1755C4.84679 26.7113 4.79652 26.24 4.98374 25.7617L9.81707 13.1367C9.93874 12.8436 10.1296 12.6024 10.3896 12.4129C10.6496 12.2235 10.9446 12.1288 11.2746 12.1288H12.4296C12.7596 12.1288 13.0565 12.2235 13.3204 12.4129C13.5843 12.6024 13.7789 12.85 13.9042 13.1559L18.695 25.7467C18.8814 26.2561 18.8343 26.7377 18.5537 27.1913C18.2735 27.6446 17.8489 27.8713 17.28 27.8713C16.9361 27.8713 16.6221 27.7766 16.3379 27.5871C16.0535 27.3977 15.8533 27.1438 15.7375 26.8255L14.8271 24.1213H8.82165ZM9.7879 21.3788H13.8187L11.8912 15.7175H11.72L9.7879 21.3788ZM25.9158 25.0455H32.4258C32.8103 25.0455 33.1385 25.181 33.4104 25.4521C33.6821 25.7232 33.8179 26.0504 33.8179 26.4338C33.8179 26.8174 33.6808 27.153 33.4067 27.4405C33.1325 27.7277 32.7993 27.8713 32.4071 27.8713H23.1517C22.9005 27.8713 22.6861 27.7829 22.5083 27.6063C22.3305 27.4293 22.2417 27.2161 22.2417 26.9667V25.513C22.2417 25.3221 22.2693 25.1407 22.3246 24.9688C22.3798 24.7971 22.4628 24.6318 22.5733 24.4729L30.1542 14.9546H24.0254C23.6346 14.9546 23.3021 14.8106 23.0279 14.5225C22.7537 14.2342 22.6167 13.8982 22.6167 13.5146C22.6167 13.1313 22.7607 12.8045 23.0487 12.5342C23.3368 12.2639 23.6751 12.1288 24.0637 12.1288H32.8442C33.0925 12.1288 33.303 12.2155 33.4758 12.3888C33.6483 12.5624 33.7346 12.7741 33.7346 13.0238V14.4971C33.7346 14.6879 33.7069 14.8642 33.6517 15.0259C33.5964 15.1873 33.5135 15.3474 33.4029 15.5063L25.9158 25.0455ZM15.9808 9.20462C15.7958 9.20462 15.658 9.13059 15.5675 8.98253C15.4772 8.8342 15.5122 8.67989 15.6725 8.51962L19.4204 4.7717C19.5679 4.6267 19.7618 4.5542 20.0021 4.5542C20.2423 4.5542 20.4348 4.6267 20.5796 4.7717L24.3275 8.51962C24.4878 8.67989 24.5222 8.8342 24.4308 8.98253C24.3394 9.13059 24.1997 9.20462 24.0117 9.20462H15.9808ZM19.4204 35.2284L15.6725 31.4804C15.5122 31.3202 15.4778 31.1659 15.5692 31.0175C15.6605 30.8695 15.8003 30.7955 15.9883 30.7955H24.0192C24.2042 30.7955 24.3419 30.8695 24.4325 31.0175C24.5228 31.1659 24.4878 31.3202 24.3275 31.4804L20.5796 35.2284C20.4321 35.3734 20.2382 35.4459 19.9979 35.4459C19.7576 35.4459 19.5651 35.3734 19.4204 35.2284Z", } } }
    }
}

#[component]
pub fn CollectionContext(#[props(default)] class: String) -> Element {
    let extended_css_class = "bg-inherit pl-[15px] py-[15px] pr-[20px] gap-[10px]";
    let text = "text-[16px] font-medium";

    rsx! {
        ContextMenu {
            class,
            direction: context_menu::Direction::DownRightEdge,
            div {
                class: "flex flex-col items-center bg-background group/cool",
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
                        ContentType::hint("更改資料夾").css(text).align_left(),
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
    let read = STORAGE.collections.read();
    let keys = read.keys();
    let keys_iter = keys.clone().zip((0..keys.len()).rev());
    let search = use_signal(String::new);
    rsx! {
        div {
            class: "bg-deep-background flex z-10 flex-col gap-[10px]",
            div {
                class: "flex gap-[5px]",
                div {
                    class: "min-w-[280px] max-w-[280px] flex gap-[5px] rounded-[30px] items-center",
                    div {
                        class: "flex-0 size-[40px]",
                        "F"
                    }
                    Hint {
                        css: "grow",
                        "選擇資料夾"
                    }
                    {ContentType::svg(ARROW_DOWN).css("svg-[40px] flex-0")}
                }
                SearchBar {
                    search,
                    default: "搜尋合集"
                }
                Button {
                    roundness: Roundness::Squircle,
                    fill_mode: FillMode::Fit,
                    string_placements: vec![
                        ContentType::custom(rsx!(ALPHABETICAL {})).align_left(),
                        ContentType::svg(ARROW_DOWN).css("svg-[40px]").align_right(),
                    ]
                }
            }
            div {
                class: "justify-center bg-background p-[30px] rounded-[30px] grid grid-flow-row gap-[20px] overflow-y-visible",
                grid_auto_rows: "280px",
                grid_auto_columns: "280px",
                grid_template_columns: "repeat(auto-fill,280px)",
                for (collection_id, i) in keys_iter {
                    CollectionBlock {
                        fat: rand::Rng::gen::<bool>(&mut rand::thread_rng()),
                        collection_id: collection_id.clone(),
                        z_index: "{i}",
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
