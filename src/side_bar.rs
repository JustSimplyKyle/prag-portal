use std::time::Duration;

use dioxus::prelude::*;
use rust_lib::api::shared_resources::collection::{use_collections_radio, CollectionId};

use crate::{
    svgs,
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
    let radio = use_collections_radio();
    let binding = radio.read();

    let collection_preview = binding.0.values().take(3);

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
                                    {x.picture_path().to_string_lossy().to_string()}
                                }
                            }
                        }
                    }
                    {ContentType::svg(ARROW_RIGHT).css("svg-[25px] group-aria-expanded:hidden")}
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

    let mut onleft = use_signal(|| false);

    let mut delayed = use_signal(&*onleft);
    use_effect(move || {
        let x = onleft();

        spawn(async move {
            tokio::time::sleep(Duration::from_millis(200)).await;
            delayed.set(x);
        });
    });

    let mut onright = use_signal(|| false);
    let mut onmiddle = use_signal(|| false);

    let base = 290.0 / 3.0;

    let left_width = use_memo(move || {
        if onright() {
            format!("{}px", 300.0 - 2.0 * (base + 5.0))
        } else if onmiddle() {
            format!("{}px", 300.0 - 1.0 * (base + 5.0))
        } else {
            "300px".to_string()
        }
    });

    rsx! {
        div {
            class: "flex flex-col place-content-start px-[20px] overflow-y-auto min-w-fit",
            div {
                class: "transition-all w-[300px] relative space-y-5 group",
                // top
                div {
                    class: "h-20 relative group/main overflow-x-clip overflow-y-clip ease-slow [&_*]:ease-slow",
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
                                ContentType::text("首頁")
                                    .css("text-black group-data-[main-page=false]/main:hidden")
                                    .align_right(),
                            ],
                            onmouseover: move |()| {
                                main_page_hover.set(true);
                            },
                            onmouseleave: move |()| {
                                main_page_hover.set(false);
                            },
                            switcher: Pages::MainPage,
                            extended_css_class: "z-10 bg-background text-black absolute left-[300px] transition-all [&_*]:transition-all
                                min-w-[100px] max-w-[100px]
                                [&:not(:hover)]:justify-center
                                hover:bg-red
                                hover:min-w-[300px] 
                                hover:max-w-[300px] 

                                group-aria-[selected=main-page]/main:bg-red  
                                [&:not(:hover)]:group-hover/main:group-data-[collections=true]/main:left-[200px] 
                                [&:not(:hover)]:group-hover/main:group-data-[explore=true]/main:left-[200px] 
                            ",
                        }
                        div {
                            class: "transition-all absolute z-0 flex left-[300px] w-[300px]
                                [&:not(:hover)]:group-hover/main:group-data-[main-page=true]/main:left-[600px] 
                                [&:not(:hover)]:group-hover/main:group-data-[collections=true]/main:left-0 
                            ",
                            div {
                                class: "transition-all grow shrink w-[100px]",
                            }
                            Button {
                                roundness: Roundness::Squircle,
                                string_placements: vec![
                                    ContentType::svg(EXPLORE).align_left(),
                                    ContentType::text("探索")
                                        .css("text-black group-data-[explore=false]/main:hidden")
                                        .align_right(),
                                ],
                                onmouseover: move |()| {
                                    explore_hover.set(true);
                                },
                                onmouseleave: move |()| {
                                    explore_hover.set(false);
                                },
                                switcher: Pages::Explore,
                                extended_css_class: "bg-background text-black grow transition-all [&_*]:transition-all
                                    min-w-[100px] max-w-[100px]
                                    [&:not(:hover)]:justify-center
                                    hover:bg-light-blue
                                    hover:min-w-[300px] 
                                    hover:max-w-[300px] 

                                    group-aria-[selected=explore]/main:bg-light-blue 
                                    group-hover/main:group-aria-[selected=explore]/main:min-w-[300px] 
                                    group-hover/main:group-aria-[selected=explore]/main:max-w-[300px] 
                                ",
                            }
                            div {
                                class: "grow shrink w-[100px]",
                            }
                        }
                        Button {
                            roundness: Roundness::Squircle,
                            string_placements: vec![
                                ContentType::svg(SIDEBAR_COLLECTION).align_left(),
                                ContentType::text("收藏庫")
                                    .css("text-black group-data-[collections=false]/main:hidden")
                                    .align_right(),
                            ],
                            onmouseover: move |()| {
                                collections_hover.set(true);
                            },
                            onmouseleave: move |()| {
                                collections_hover.set(false);
                            },
                            switcher: Pages::Collections,
                            extended_css_class: "z-10 bg-background text-black absolute -right-[300px] transition-all [&_*]:transition-all
                                min-w-[100px] max-w-[100px]

                                hover:bg-green
                                [&:not(:hover)]:justify-center

                                group-aria-[selected=collections]/main:bg-green 
                                hover:min-w-[300px] 
                                hover:max-w-[300px] 
                                [&:not(:hover)]:group-hover/main:group-data-[main-page=true]/main:-right-[400px] 
                                [&:not(:hover)]:group-hover/main:group-data-[explore=true]/main:-right-[400px] 
                            ",
                        }
                    }
                }
                // middle
                div {
                    class: "relative flex flex-col flex-nowrap overflow-y-scroll space-y-1",
                    Button {
                        roundness: Roundness::Squircle,
                        string_placements: folded_images,
                        extended_css_class: "bg-background",
                    }
                    for collection_id in binding.0.keys().cloned() {
                        SidebarCollectionBlock {
                            collection_id,
                        }
                    }
                }
                // bottom
                div {
                    class: "transition-all relative overflow-x-clip ease-slow [&_*]:ease-slow [&_*]:duration-300",
                    div {
                        class: "transition-all [&_*]:transition-all absolute flex items-stretch",
                        left: "-300px",
                        width: "900px",
                        height: "90px",
                        div {
                            min_width: left_width(),
                        }
                        div {
                            class: "flex justify-stretch items-center bg-background px-[20px] relative delay-200 [&_*]:delay-200",
                            role: "button",
                            class: if onleft() { "rounded-b" } else { "rounded" },
                            onmouseover: move |_| {
                                onleft.set(true);
                            },
                            onmouseleave: move |_| {
                                onleft.set(false);
                            },
                            onclick: move |_| {
                                Pages::DownloadProgress.switch_active_to_self();
                            },

                            min_width: if onleft() { "300px" } else { "{base}px" },

                            div {
                                class: "absolute inset-0 bg-deep-background pt-[20px] overflow-hidden border-secondary-surface",
                                class: if onleft() { "border-b border-secondary-surface" },
                                transform: "translateY(-100%)",
                                max_height: if onleft() { "270px" } else { "0px" },
                                div {
                                    class: "flex flex-col gap-[5px] bg-background pt-[20px] rounded-t",
                                    div {
                                        class: "flex flex-col gap-[12px] bg-background px-[20px] pb-[20px]",
                                        div {
                                            class: "flex gap-[10px]",
                                            div {
                                                class: "flex flex-col grow gap-[10px]",
                                                div {
                                                    class: "text-[18px] font-medium trim text-white trim",
                                                    "TNVBP"
                                                }
                                                div {
                                                    class: "text-hint text-[12px] font-medium trim",
                                                    "模組 - 安裝中"
                                                }
                                            }
                                            div {
                                                class: "flex items-end gap-[3px]",
                                                div {
                                                    class: "text-green text-[20px] font-bold font-english trim",
                                                    "25"
                                                }
                                                div {
                                                    class: "text-[15px] font-medium font-english trim",
                                                    "%"
                                                }
                                            }
                                        }
                                        div {
                                            "arst"
                                        }
                                    }
                                }
                            }

                            div {
                                class: if onleft() { "basis-0" } else { "basis-1/3" },
                            }
                            div {
                                class: "basis-1/3 flex justify-start",
                                svgs::DOWNLOAD { }
                            }
                            div {
                                class: "basis-1/3",
                            }
                            div {
                                class: "text-nowrap",
                                class: if onleft() { "basis-1/3" } else { "basis-0" },
                                div {
                                    class: "flex flex-row-reverse text-[25px] font-bold text-white",
                                    class: if !delayed() { "hidden" },
                                    "下載"
                                }
                            }
                        }
                        div {
                            min_width: "5px",
                        }
                        button {
                            class: "flex justify-stretch items-center rounded bg-background px-[20px]",
                            onmouseover: move |_| {
                                onmiddle.set(true);
                            },
                            onmouseleave: move |_| {
                                onmiddle.set(false);
                            },
                            min_width: if onmiddle() { "300px" } else { "{base}px" },

                            div {
                                class: if onmiddle() { "basis-0" } else { "basis-1/3" },
                            }
                            div {
                                class: "basis-1/3 flex justify-start",
                                svgs::BOOK { }
                            }
                            div {
                                class: "basis-1/3",
                            }
                            div {
                                class: "text-nowrap",
                                class: if onmiddle() { "basis-1/3" } else { "basis-0" },
                                if onmiddle() {
                                    div {
                                        class: "flex flex-row-reverse text-[25px] font-bold text-white",
                                        "指南"
                                    }
                                }
                            }
                        }
                        div {
                            min_width: "5px",
                        }
                        button {
                            class: "flex justify-center items-center rounded bg-background px-[20px]",
                            onmouseover: move |_| {
                                onright.set(true);
                            },
                            onmouseleave: move |_| {
                                onright.set(false);
                            },
                            min_width: if onright() { "300px" } else { "{base}px" },

                            div {
                                class: if onright() { "basis-0" } else { "basis-1/3" },
                            }
                            div {
                                class: "basis-1/3 flex justify-start",
                                svgs::SMALL_TOGGLE { }
                            }
                            div {
                                class: "basis-1/3",
                            }
                            div {
                                class: "text-nowrap",
                                class: if onright() { "basis-1/3" } else { "basis-0" },
                                if onright() {
                                    div {
                                        class: "flex flex-row-reverse text-[25px] font-bold text-white",
                                        "設定"
                                    }
                                }
                            }

                        }
                        div {
                            min_width: "300px",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarCollectionBlock(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let radio = collection_id().use_collection_radio();

    let picture_path = radio.read().picture_path().to_string_lossy().to_string();
    let display_name = radio.read().display_name().clone();
    let img_block = rsx! {
        div {
            class: "relative transition-all container w-[50px] h-[50px] group-aria-expanded:w-20 group-aria-expanded:h-20 border-2 border-[#2E2E2E] rounded-[15px] group-aria-expanded:rounded-[5px]",
            {
                ContentType::image(&picture_path)
                    .css(
                        "absolute inset-0 transition-all w-full h-full object-cover inline-flex items-center rounded-[15px] group-aria-expanded:rounded-[5px]",
                    )
            }
            div {
                class: "absolute inset-x-0 bottom-0 w-3 h-3 bg-[#CCE246] rounded-full",
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
                                ContentType::text(display_name)
                                    .onmounted(element)
                                    .style(style())
                                    .css(
                                        "w-full group-hover:group-aria-selected:animate-scroll-left font-medium",
                                    ),
                                ContentType::svg(ARROW_RIGHT).css("min-w-0 z-0 svg-[30px]"),
                            ],
                            Alignment::Right,
                        )
                        .css(
                            "w-full items-center group-aria-busy:hidden text-nowrap text-ellipse overflow-x-clip",
                        ),
                ],
                switcher: Pages::collection_display(collection_id()),
                focus_color_change: false,
                // background_image: darken_sidebar_background(&picture_path),
                background_size: "cover",
                extended_css_class: "bg-background gap-[15px] p-[15px] object-cover transition-all delay-[25ms] group-aria-expanded:w-20 group-aria-expanded:min-h-20 group-aria-expanded:p-0",
            }
        }
    }
}
