use dioxus::prelude::*;
use rust_lib::api::shared_resources::{authentication::MINECRAFT_USER_PROFILE_URL, entry::STORAGE};

use crate::{
    BaseComponents::{Alignment, Button, ContentType, Contents, FillMode, Roundness, Size},
    MainPage::{CollectionBlock, COLLECTION_PIC},
    EXPLORE,
};

#[component]
pub fn Collections() -> Element {
    let collections =
        use_resource(
            move || async move { STORAGE.collections.clone().read_owned().await.to_owned() },
        );
    let collections_iterator = collections().into_iter().flat_map(|x| x.into_iter());
    rsx! {
        div { class: "flex flex-col space-y-[10px]",
            div { class: "flex space-x-[10px]",
                Button {
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    size: Size::Medium,
                    string_placements: vec![
                        ContentType::svg(EXPLORE)
                            .css("flex items-center justify-center w-[30px] h-[30px]")
                            .align_left(),
                        ContentType::text("全部").align_right(),
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
                for collection in collections_iterator {
                    CollectionBlock {
                        collection,
                        extended_class: "rounded-[20px]",
                        picture: COLLECTION_PIC
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
