use dioxus::prelude::*;
use rust_lib::api::shared_resources::{collection::CollectionId, entry::STORAGE};

use crate::{
    text_scroller::use_text_scroller,
    BaseComponents::{
        atoms::button::{Button, Roundness},
        molecules::switcher::StateSwitcher,
        string_placements::{Alignment, ContentType, Contents, Image},
    },
    Pages, ARROW_RIGHT, EXPLORE, HISTORY, HOME, SIDEBAR_COLLECTION, SIM_CARD,
};

#[component]
pub fn SideBar() -> Element {
    let binding = STORAGE.collections.read();

    let collection_preview = binding.values().take(3);

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
                        for x in collection_preview {
                            div {
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
                div {
                    class: "text-lime-300 trim",
                    "我的錦集"
                }
            }
        }
    };
    let selected = use_memo(|| HISTORY.read().active.to_string());
    let mut main_page_hover = use_signal(|| false);
    let mut explore_hover = use_signal(|| false);
    let mut collections_hover = use_signal(|| false);
    rsx! {
        div {
            class: "flex flex-col place-content-start mx-5",
            div {
                class: "transition-all w-[300px] relative space-y-5 ease-linear [&_*]:ease-linear [&_*]:duration-150 group",
                // top
                div {
                    class: "h-20 relative group/main overflow-x-clip overflow-y-clip",
                    "data-main-page": main_page_hover(),
                    "data-explore": explore_hover(),
                    "data-collections": collections_hover(),
                    aria_selected: selected(),
                    div {
                        class: "h-20 w-full absolute -left-[300px] w-[900px] relative",
                        Button {
                            roundness: Roundness::Squircle,
                            string_placements: vec![
                                ContentType::svg(HOME).align_left(),
                                ContentType::text("首頁").css("text-black group-data-[main-page=false]/main:hidden").align_right(),
                            ],
                            onmouseover: move |()| {
                                main_page_hover.set(true);
                            },
                            onmouseleave: move |()| {
                                main_page_hover.set(false);
                            },
                            switcher: Pages::MainPage,
                            extended_css_class: "z-10 bg-background text-black absolute left-[300px] transition-all [&_*]:transition-all
                                min-w-[96.66666px] max-w-[96.66666px]
                                [&:not(:hover)]:justify-center
                                hover:bg-red
                                hover:min-w-[300px] 
                                hover:max-w-[300px] 

                                group-aria-[selected=main-page]/main:bg-red  
                                [&:not(:hover)]:group-hover/main:group-data-[collections=true]/main:left-[200px] 
                                [&:not(:hover)]:group-hover/main:group-data-[explore=true]/main:left-[200px] 
                            "
                        }
                        div {
                            class: "transition-all absolute z-0 flex left-[300px] gap-[5px] w-[300px]
                                [&:not(:hover)]:group-hover/main:group-data-[main-page=true]/main:left-[600px] 
                                [&:not(:hover)]:group-hover/main:group-data-[collections=true]/main:left-0 
                            ",
                            div {
                                class: "transition-all grow shrink w-[96.66666px]
                                ",
                            }
                            Button {
                                roundness: Roundness::Squircle,
                                string_placements: vec![
                                    ContentType::svg(EXPLORE).align_left(),
                                    ContentType::text("探索").css("text-black group-data-[explore=false]/main:hidden").align_right(),
                                ],
                                onmouseover: move |()| {
                                    explore_hover.set(true);
                                },
                                onmouseleave: move |()| {
                                    explore_hover.set(false);
                                },
                                switcher: Pages::Explore,
                                extended_css_class: "bg-background text-black grow transition-all [&_*]:transition-all
                                    min-w-[96.66666px] max-w-[96.66666px]
                                    [&:not(:hover)]:justify-center
                                    hover:bg-light-blue
                                    hover:min-w-[300px] 
                                    hover:max-w-[300px] 

                                    group-aria-[selected=explore]/main:bg-light-blue 
                                    group-hover/main:group-aria-[selected=explore]/main:min-w-[300px] 
                                    group-hover/main:group-aria-[selected=explore]/main:max-w-[300px] 
                                "
                            }
                            div {
                                class: "grow shrink w-[96.66666px]
                                ",
                            }
                        }
                        Button {
                            roundness: Roundness::Squircle,
                            string_placements: vec![
                                ContentType::svg(SIDEBAR_COLLECTION).align_left(),
                                ContentType::text("收藏庫").css("text-black group-data-[collections=false]/main:hidden").align_right(),
                            ],
                            onmouseover: move |()| {
                                collections_hover.set(true);
                            },
                            onmouseleave: move |()| {
                                collections_hover.set(false);
                            },
                            switcher: Pages::Collections,
                            extended_css_class: "z-10 bg-background text-black absolute -right-[300px] transition-all [&_*]:transition-all
                                min-w-[96.66666px] max-w-[96.66666px]

                                hover:bg-green
                                [&:not(:hover)]:justify-center

                                group-aria-[selected=collections]/main:bg-green 
                                hover:min-w-[300px] 
                                hover:max-w-[300px] 
                                [&:not(:hover)]:group-hover/main:group-data-[main-page=true]/main:-right-[400px] 
                                [&:not(:hover)]:group-hover/main:group-data-[explore=true]/main:-right-[400px] 
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
                    for collection_id in STORAGE.collections.read().keys().cloned() {
                        SidebarCollectionBlock {
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
