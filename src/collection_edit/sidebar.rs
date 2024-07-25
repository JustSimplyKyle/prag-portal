use dioxus::prelude::*;
use rust_lib::api::shared_resources::collection::CollectionId;

use crate::{
    collection_display::{DISPLAY_BACKGROUND, UNDO},
    collection_edit::EditState,
    main_page::ARROW_LEFT,
    pages::Pages,
    text_scroller::use_text_scroller,
    BaseComponents::{
        atoms::button::{Button, FillMode, Roundness},
        molecules::switcher::StateSwitcher,
        string_placements::{Alignment, ContentType, Contents},
    },
    ARROW_RIGHT,
};
#[component]
pub fn EditSidebar(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        div {
            class: "flex flex-col min-w-[400px] max-w-[400px] gap-[20px]",
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
                        ContentType::text("返回頁面").align_center(),
                    ]
                }
                Button {
                    roundness: Roundness::Pill,
                    extended_css_class: "flex w-auto min-w-auto items-center bg-background gap-[15px] pl-[20px] pr-[30px]",
                    string_placements: vec![
                        ContentType::svg(ARROW_LEFT).align_center(),
                        ContentType::text("返回頁面").align_center(),
                    ]
                }
            }
        }
    }
}

#[component]
fn EditSidebarInfographic(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let (onmounted, status, style) = use_text_scroller();
    let collection = collection_id().get_collection();
    rsx! {
        div {
            class: "group flex flex-col w-full",
            aria_selected: status(),
            div {
                class: "flex flex-col p-5 justify-end rounded-t-[50px] w-full min-h-[250px]",
                background: format!(
                    "radial-gradient(171.48% 102.52% at 0% 100%, #000 0%, rgba(0, 0, 0, 0.00) 100%), url(\"{}\") lightgray 50% / cover no-repeat",
                    DISPLAY_BACKGROUND,
                ),
                {
                    ContentType::image(collection.read().picture_path().to_string_lossy().to_string()).css("w-[100px] h-[100px] bg-cover rounded-t-[50px] rounded-bl-[15px] rounded-br-[50px] p-[5px]")
                }
            }
            Button {
                roundness: Roundness::Bottom,
                clickable: false,
                extended_css_class: "bg-background overflow-x-clip justify-start px-5 pt-[22px]",
                string_placements: vec![
                    Contents::new(
                            vec![
                                ContentType::text(collection.read().display_name())
                                    .onmounted(onmounted)
                                    .style(dbg!(style()))
                                    .css("text-3xl font-black w-full group-hover:group-aria-selected:animate-scroll-left overflow-x-clip text-nowrap"),
                                ContentType::hint("由我建立•18 分鐘•不久前開啟")
                                    .css("font-medium text-[15px]"),
                            ],
                            Alignment::Left,
                        )
                        .css("flex flex-col gap-[15px]"),
                ]
            }
        }
    }
}
