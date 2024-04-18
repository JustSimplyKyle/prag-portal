use dioxus::prelude::*;

use crate::{
    BaseComponents::{Alignment, Button, ContentType, Contents, FillMode, Roundness, Size},
    MainPage::{CollectionBlock, COLLECTION_PIC},
};

#[component]
pub fn Collections() -> Element {
    rsx! {
        div { class: "flex flex-col space-y-[10px]",
            div { class: "flex space-x-[10px]",
                Button {
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    size: Size::Medium,
                    string_placements: vec![ContentType::text("H").align_left(), ContentType::text("全部").align_right()],
                    extended_css_class: "pl-[20px] pr-[25px] py-[12px]"
                }
                Button {
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    size: Size::Medium,
                    string_placements: vec![ContentType::text("H").align_center()],
                    extended_css_class: "px-[20px] py-[12px]"
                }
                Button {
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    size: Size::Medium,
                    string_placements: vec![
                        ContentType::text("H").align_left(),
                        ContentType::text("分類夾").align_right(),
                    ],
                    extended_css_class: "pl-[20px] pr-[25px] py-[12px]"
                }
                Button {
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    size: Size::Medium,
                    string_placements: vec![ContentType::text("H").align_center()],
                    extended_css_class: "px-[20px] py-[12px]"
                }
            }
            div { class: "grid grid-flow-row grid-cols-[repeat(auto-fill,280px)] gap-[20px]",
                CollectionBlock {
                    extended_class: "rounded-[20px]",
                    main_text: "創世幻想",
                    hint: "不久前開啟•由我建立",
                    picture: COLLECTION_PIC
                }
                CollectionBlock {
                    extended_class: "rounded-[20px]",
                    main_text: "text",
                    hint: "arst",
                    picture: COLLECTION_PIC
                }
                CollectionBlock {
                    extended_class: "rounded-[20px]",
                    main_text: "text",
                    hint: "arst",
                    picture: COLLECTION_PIC
                }
                CollectionBlock {
                    extended_class: "rounded-[20px]",
                    main_text: "text",
                    hint: "arst",
                    picture: COLLECTION_PIC
                }
                CollectionBlock {
                    extended_class: "rounded-[20px]",
                    main_text: "text",
                    hint: "arst",
                    picture: COLLECTION_PIC
                }
                CollectionBlock {
                    extended_class: "rounded-[20px]",
                    main_text: "text",
                    hint: "arst",
                    picture: COLLECTION_PIC
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
                is_button: false
            }
        }
    }
}
