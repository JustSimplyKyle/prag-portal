use std::time::Duration;

use dioxus::prelude::*;
use rust_lib::api::shared_resources::{collection::CollectionId, entry::STORAGE};

use crate::{
    BaseComponents::{
        atoms::button::{Button, Roundness},
        molecules::switcher::StateSwitcher,
        string_placements::{ContentType, Image, Text},
    },
    Pages, ARROW_RIGHT, EXPLORE, HISTORY, HOME, SIDEBAR_COLLECTION, SIM_CARD,
};

#[component]
pub fn SideBar() -> Element {
    let mut expanded = use_signal(|| false);
    let delayed_expanded = use_resource(move || async move {
        if expanded() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        } else {
            // tokio::time::sleep(Duration::from_millis(100)).await;
        }
        expanded()
    });
    let onclick = move |()| {
        Pages::Collections.switch_active_to_self();
        expanded.toggle();
    };

    let binding = STORAGE.collections.read();

    let collection_preview = binding.iter().take(3);

    let folded_images = rsx! {
        div {
            class: "grid grid-flow-col justify-stretch items-center",
            div {
                class: "justify-self-start transition-all",
                {ContentType::svg(HOME).css("hidden group-aria-expanded:block")}
                div {
                    class: "flex items-center space-x-0",
                    div {
                        class: "flex space-x-[-20px]",
                        for (id , x) in collection_preview {
                            div {
                                key: "{id}",
                                Image {
                                    css: "z-50 w-10 h-10 object-cover shrink-0 inline-flex justify-center items-center rounded-full border-2 border-zinc-900 group-aria-expanded:hidden",
                                    {x.picture_path().to_string_lossy().to_string()}
                                }
                            }
                        }
                    }
                    {
                        ContentType::svg(ARROW_RIGHT).css("svg-[25px] group-aria-expanded:hidden")
                    }
                }
            }
            div {
                class: "justify-self-end group-aria-busy:hidden",
                Text {
                    css: "text-lime-300",
                    "我的錦集"
                }
            }
        }
    };
    rsx! {
        div {
            class: "flex flex-col place-content-start mx-5",
            div {
                class: "w-[300px] space-y-5 transition-all ease-linear duration-500 aria-expanded:w-[80px] group",
                aria_expanded: !expanded(),
                aria_busy: !delayed_expanded().unwrap_or(false),
                // top
                div {
                    class: "flex flex-col space-y-1",
                    Button {
                        roundness: Roundness::Top,
                        string_placements: vec![
                            ContentType::svg(HOME).align_left(),
                            ContentType::text("首頁").css("group-aria-busy:hidden").align_right(),
                        ],
                        focus_color_change: true,
                        switcher: Pages::MainPage,
                        extended_css_class: "bg-background group-aria-expanded:pr-5"
                    }
                    Button {
                        roundness: Roundness::None,
                        string_placements: vec![
                            ContentType::svg(EXPLORE).align_left(),
                            ContentType::text("探索").css("group-aria-busy:hidden").align_right(),
                        ],
                        focus_color_change: true,
                        switcher: Pages::Explore,
                        extended_css_class: "bg-background group-aria-expanded:pr-5"
                    }
                    Button {
                        roundness: Roundness::Bottom,
                        string_placements: vec![
                            ContentType::svg(SIDEBAR_COLLECTION).align_left(),
                            ContentType::text("收藏庫").css("group-aria-busy:hidden").align_right(),
                        ],
                        switcher: Pages::Collections,
                        onclick,
                        focus_color_change: true,
                        extended_css_class: "bg-background group-aria-expanded:pr-5"
                    }
                }
                // middle
                div {
                    class: "flex flex-col flex-nowrap overflow-scroll max-h-[451px] space-y-1",
                    Button {
                        roundness: Roundness::Top,
                        string_placements: folded_images,
                        extended_css_class: "bg-background"
                    }
                    for collection_id in STORAGE.collections.read().keys().cloned() {
                        SidebarCollectionBlock {
                            key: "{collection_id.clone()}",
                            collection_id
                        }
                    }
                }
                // bottom
                div {
                    class: "flex flex-col space-y-1",
                    Button {
                        roundness: Roundness::Top,
                        string_placements: vec![
                            ContentType::svg(SIM_CARD).align_left(),
                            ContentType::text("返回")
                                .align_right()
                                .css(
                                    "hidden group-aria-[busy=false]:group-aria-selected/active:block group-aria-busy:hidden",
                                ),
                            ContentType::text("無下載佇列")
                                .align_right()
                                .css("group-aria-selected/active:hidden group-aria-busy:hidden text-hint"),
                        ],
                        focus_color_change: true,
                        switcher: Pages::DownloadProgress,
                        extended_css_class: "bg-background group/active items-center",
                        onclick: move |()| {
                            let history = HISTORY();
                            let prev = history.prev_peek();
                            if history.active() == &Pages::DownloadProgress {
                                if let Some(prev) = prev {
                                    prev.switch_active_to_self();
                                }
                            } else {
                                Pages::DownloadProgress.switch_active_to_self();
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarCollectionBlock(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let binding = collection_id.read();
    let collection = binding.get_collection();
    let picture_path = collection.picture_path().to_string_lossy().to_string();
    let img_block = rsx! {
        div {
            class: "relative transition-all container w-[50px] h-[50px] group-aria-expanded:w-20 group-aria-expanded:h-20 border-2 border-[#2E2E2E] rounded-[15px] group-aria-expanded:rounded-[5px]",
            { ContentType::image(&picture_path)
            .css("absolute inset-0 transition-all w-full h-full object-cover inline-flex items-center rounded-[15px] group-aria-expanded:rounded-[5px]")
            }
            div {
                class: "absolute inset-x-0 bottom-0 w-3 h-3 bg-[#CCE246] rounded-full"
            }
        }
    };
    rsx! {
        Button {
            roundness: Roundness::None,
            string_placements: vec![
                ContentType::custom(img_block).align_left(),
                ContentType::text(collection.display_name().clone())
                    .align_right()
                    .css("group-aria-busy:hidden"),
            ],
            switcher: Pages::collection_display(collection.get_collection_id()),
            focus_color_change: false,
            background_image: darken_sidebar_background(&picture_path),
            background_size: "cover",
            extended_css_class: "bg-background object-cover transition-all delay-[25ms] group-aria-expanded:w-20 group-aria-expanded:min-h-20 group-aria-expanded:p-0"
        }
    }
}

fn darken_sidebar_background(s: &impl ToString) -> String {
    format!("linear-gradient(to right, rgba(25, 25, 25, 0.8) 0%, rgba(25, 25, 25, 1) 68%, rgba(25, 25, 25, 1) 100%),url(\"{}\")", s.to_string())
}
