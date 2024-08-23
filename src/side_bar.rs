use std::time::Duration;

use dioxus::prelude::*;
use rust_lib::api::shared_resources::{collection::CollectionId, entry::STORAGE};

use crate::{
    text_scroller::use_text_scroller,
    BaseComponents::{
        atoms::button::{Button, Roundness},
        molecules::switcher::StateSwitcher,
        string_placements::{Alignment, ContentType, Contents, Image, Text},
    },
    Pages, ARROW_RIGHT, EXPLORE, HISTORY, HOME, SIDEBAR_COLLECTION, SIM_CARD,
};

#[component]
pub fn SideBar() -> Element {
    let keys = use_context::<Memo<Vec<CollectionId>>>();

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
                                    {x.read().picture_path().to_string_lossy().to_string()}
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
                class: "flex justify-self-end group-aria-busy:hidden",
                Text {
                    css: "text-lime-300",
                    "我的錦集"
                }
            }
        }
    };
    #[derive(derive_more::Display, PartialEq, Eq, Clone, Copy)]
    enum Selection {
        MainPage,
        Explore,
        Collections,
    }
    let mut clicked = use_signal(|| false);
    let delayed = use_resource(move || async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        clicked()
    });
    let mut selected = use_signal(|| Selection::MainPage);
    rsx! {
        div {
            class: "flex flex-col place-content-start mx-5",
            div {
                class: "transition-all w-[300px] relative space-y-5 ease-linear duration-500 group",
                // top
                div {
                    class: "h-20 relative group/main overflow-x-clip overflow-y-clip",
                    onmouseover: move |_| {
                        clicked.set(true);
                    },
                    onmouseleave: move |_| {
                        clicked.set(false);
                    },
                    aria_busy: delayed().unwrap_or(false),
                    aria_selected: selected().to_string(),
                    div {
                        class: "h-20 w-full absolute -left-[300px] w-[900px] relative",
                        Button {
                            roundness: Roundness::Squircle,
                            string_placements: vec![
                                ContentType::svg(HOME).css("[&_#mysvg_path]:!fill-current [&_#mysvg_path]:!text-red [&_#mysvg_path]:!fill-red").align_left(),
                                ContentType::text("首頁").css("group-aria-[busy=true]/main:hidden").align_right(),
                            ],
                            onclick: move |_| {
                                selected.set(Selection::MainPage);
                            },
                            switcher: Pages::MainPage,
                            extended_css_class: "z-10 bg-background text-black absolute left-[200px] transition-all [&_*]:transition-all
                                min-w-[96.66666px] max-w-[96.66666px]
                                group-hover/main:left-[300px]
                                group-hover/main:justify-center

                                group-[&:not(:hover)]/main:group-aria-[selected=MainPage]/main:bg-red  
                                group-[&:not(:hover)]/main:group-aria-[selected=MainPage]/main:min-w-[300px] 
                                group-[&:not(:hover)]/main:group-aria-[selected=MainPage]/main:max-w-[300px] 
                                group-[&:not(:hover)]/main:group-aria-[selected=MainPage]/main:left-[300px] 
                            "
                        }
                        div {
                            class: "transition-all absolute z-0 flex left-[300px] gap-[5px] w-[300px]
                                group-[&:not(:hover)]/main:group-aria-[selected=MainPage]/main:left-[600px]
                                group-[&:not(:hover)]/main:group-aria-[selected=Collections]/main:left-[100px]
                            ",
                            div {
                                class: "transition-all grow shrink w-[96.66666px]
                                group-[&:not(:hover)]/main:group-aria-[selected=MainPage]/main:w-0
                                group-[&:not(:hover)]/main:group-aria-[selected=Explore]/main:w-0
                                ",
                            }
                            Button {
                                roundness: Roundness::Squircle,
                                onclick: move |_| {
                                    selected.set(Selection::Explore);
                                },
                                string_placements: vec![
                                    ContentType::svg(EXPLORE).align_left(),
                                    ContentType::text("探索").css("group-aria-[busy=true]/main:hidden").align_right(),
                                ],
                                switcher: Pages::Explore,
                                extended_css_class: "bg-background text-black grow transition-all [&_*]:transition-all
                                    min-w-[96.66666px] max-w-[96.66666px]
                                    group-hover/main:justify-center
                                    group-[&:not(:hover)]/main:group-aria-[selected=Explore]/main:bg-light-blue 
                                    group-[&:not(:hover)]/main:group-aria-[selected=Explore]/main:min-w-[300px] 
                                    group-[&:not(:hover)]/main:group-aria-[selected=Explore]/main:max-w-[300px] 
                                "
                            }
                            div {
                                class: "grow shrink w-[96.66666px]
                                group-[&:not(:hover)]/main:group-aria-[selected=Explore]/main:w-0
                                ",
                            }
                        }
                        Button {
                            roundness: Roundness::Squircle,
                            onclick: move |_| {
                                selected.set(Selection::Collections);
                                // expanded.toggle();
                            },
                            string_placements: vec![
                                ContentType::svg(SIDEBAR_COLLECTION).align_left(),
                                ContentType::text("收藏庫").css("group-aria-[busy=true]/main:hidden").align_right(),
                            ],
                            switcher: Pages::Collections,
                            extended_css_class: "z-10 bg-background text-black absolute -right-[400px] transition-all [&_*]:transition-all
                                min-w-[96.66666px] max-w-[96.66666px]
                                group-hover/main:-right-[300px]
                                group-hover/main:justify-center
                                group-[&:not(:hover)]/main:group-aria-[selected=Collections]/main:min-w-[300px] 
                                group-[&:not(:hover)]/main:group-aria-[selected=Collections]/main:bg-green 
                                group-[&:not(:hover)]/main:group-aria-[selected=Collections]/main:max-w-[300px] 
                                group-[&:not(:hover)]/main:group-aria-[selected=Collections]/main:-right-[300px] 
                            "
                        }
                    }
                }
                // middle
                div {
                    class: "flex flex-col flex-nowrap overflow-scroll max-h-[451px] space-y-1",
                    Button {
                        roundness: Roundness::Squircle,
                        string_placements: folded_images,
                        extended_css_class: "bg-background"
                    }
                    for collection_id in keys() {
                        SidebarCollectionBlock {
                            key: "{collection_id}",
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
    let collection = collection_id().get_collection();
    let picture_path = collection
        .read()
        .picture_path()
        .to_string_lossy()
        .to_string();
    let display_name = collection.read().display_name().clone();
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

    let (element, status, style) = use_text_scroller();

    rsx! {
        div {
            class: "group",
            aria_selected: status(),
            Button {
                roundness: Roundness::Squircle,
                string_placements: vec![
                    ContentType::custom(img_block).align_left(),
                    Contents::new(
                        vec![
                            ContentType::text(display_name).onmounted(element).style(style()).css("w-full group-hover:group-aria-selected:animate-scroll-left font-medium"),
                            ContentType::svg(ARROW_RIGHT).css("min-w-0 z-0 svg-[30px]")
                        ],
                        Alignment::Right
                    )
                    .css("w-full items-center group-aria-busy:hidden text-nowrap text-ellipse overflow-x-clip"),
                ],
                switcher: Pages::collection_display(collection_id()),
                focus_color_change: false,
                // background_image: darken_sidebar_background(&picture_path),
                background_size: "cover",
                extended_css_class: "bg-background gap-[15px] p-[15px] object-cover transition-all delay-[25ms] group-aria-expanded:w-20 group-aria-expanded:min-h-20 group-aria-expanded:p-0"
            }
        }
    }
}

fn darken_sidebar_background(s: &impl ToString) -> String {
    format!("linear-gradient(to right, rgba(25, 25, 25, 0.8) 0%, rgba(25, 25, 25, 1) 68%, rgba(25, 25, 25, 1) 100%),url(\"{}\")", s.to_string())
}
