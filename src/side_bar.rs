use std::{rc::Rc, time::Duration};

use dioxus::prelude::*;
use rust_lib::api::shared_resources::{collection::Collection, entry::STORAGE};
use tailwind_fuse::tw_merge;

use crate::{
    main_page::COLLECTION_PIC,
    BaseComponents::{Alignment, Button, ContentType, Roundness, Switcher},
    Pages, ARROW_RIGHT, EXPLORE, HISTORY, HOME, SIDEBAR_COLLECTION, SIM_CARD,
};

static EXPANDED: GlobalSignal<bool> = GlobalSignal::new(|| false);

#[component]
pub fn SideBar() -> Element {
    let delayed_expanded = use_resource(move || async move {
        if EXPANDED() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        } else {
            // tokio::time::sleep(Duration::from_millis(100)).await;
        }
        EXPANDED()
    });
    let onclick = move |()| {
        Pages::Collections.switch_active_to_self();
        *EXPANDED.write() = !EXPANDED();
    };

    let collections = STORAGE().collections;

    let folded_images = rsx! {
        div { class: "transition-all",
            {ContentType::svg(HOME).css("hidden group-aria-expanded:block").get_element()},
            div { class: "flex items-center space-x-0",
                div { class: "flex space-x-[-20px]",
                    {ContentType::image(COLLECTION_PIC.to_string())
                        .css(
                            "z-50 w-10 h-10 object-cover shrink-0 inline-flex justify-center items-center rounded-full border-2 border-zinc-900 group-aria-expanded:hidden"
                        ).get_element()},
                    {ContentType::image(COLLECTION_PIC.to_string())
                        .css(
                            "z-40 w-10 h-10 object-cover shrink-0 inline-flex justify-center items-center rounded-full border-2 border-zinc-900 group-aria-expanded:hidden"
                        ).get_element()},
                    {ContentType::image(COLLECTION_PIC.to_string())
                        .css(
                            "z-30 w-10 h-10 object-cover shrink-0 inline-flex justify-center items-center rounded-full border-2 border-zinc-900 group-aria-expanded:hidden"
                        ).get_element()}
                }
                {
                    ContentType::svg(ARROW_RIGHT).css("svg-[25px] group-aria-expanded:hidden").get_element()
                }
            }
        }
        div { class: tw_merge!(Alignment::Right.get_alignment_class(), "group-aria-busy:hidden"),
            {ContentType::text("我的錦集").css("text-lime-300").get_element()}
        }
    };
    rsx! {
        div { class: "flex flex-col place-content-start mx-5",
            div {
                class: "w-[300px] space-y-5 transition-all ease-linear duration-500 aria-expanded:w-[80px] group",
                aria_expanded: !EXPANDED(),
                aria_busy: !delayed_expanded().unwrap_or(false),
                // top
                div { class: "flex flex-col space-y-1",
                    Button {
                        roundness: Roundness::Top,
                        string_placements: vec![
                            ContentType::svg(HOME).align_left(),
                            ContentType::text("首頁").css("group-aria-busy:hidden").align_right(),
                        ],
                        focus_color_change: true,
                        signal: Rc::new(Pages::MainPage) as Rc<dyn Switcher>,
                        extended_css_class: "bg-background group-aria-expanded:pr-5"
                    }
                    Button {
                        roundness: Roundness::None,
                        string_placements: vec![
                            ContentType::svg(EXPLORE).align_left(),
                            ContentType::text("探索").css("group-aria-busy:hidden").align_right(),
                        ],
                        focus_color_change: true,
                        signal: Rc::new(Pages::Explore) as Rc<dyn Switcher>,
                        extended_css_class: "bg-background group-aria-expanded:pr-5"
                    }
                    Button {
                        roundness: Roundness::Bottom,
                        string_placements: vec![
                            ContentType::svg(SIDEBAR_COLLECTION).align_left(),
                            ContentType::text("收藏庫").css("group-aria-busy:hidden").align_right(),
                        ],
                        signal: Rc::new(Pages::Collections) as Rc<dyn Switcher>,
                        onclick,
                        focus_color_change: true,
                        extended_css_class: "bg-background group-aria-expanded:pr-5"
                    }
                }
                // middle
                div { class: "flex flex-col flex-nowrap overflow-scroll max-h-[451px] space-y-1",
                    Button { roundness: Roundness::Top, string_placements: folded_images, extended_css_class: "bg-background" }
                    for collection in collections {
                        SidebarCollectionBlock { collection: collection }
                    }
                }
                // bottom
                div { class: "flex flex-col space-y-1",
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
                        signal: Rc::new(Pages::DownloadProgress) as Rc<dyn Switcher>,
                        extended_css_class: "bg-background group/active items-center",
                        onclick: move |()| {
                            let history = HISTORY();
                            let prev = history.prev_peek();
                            if HISTORY().active() == &Pages::DownloadProgress {
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
fn SidebarCollectionBlock(collection: Collection) -> Element {
    let img_block = rsx! {
        div { class: "relative transition-all container w-[50px] h-[50px] group-aria-expanded:w-20 group-aria-expanded:h-20 border-2 border-[#2E2E2E] rounded-[15px] group-aria-expanded:rounded-[5px]",
            { ContentType::image(COLLECTION_PIC.to_string())
            .css("absolute inset-0 transition-all w-full h-full object-cover inline-flex items-center rounded-[15px] group-aria-expanded:rounded-[5px]",)
            .get_element() },
            div { class: "absolute inset-x-0 bottom-0 w-3 h-3 bg-[#CCE246] rounded-full" }
        }
    };
    let display = &collection.display_name;
    let signal_check = collection.get_collection_id();
    rsx! {
        Button {
            roundness: Roundness::None,
            string_placements: vec![
                ContentType::custom(img_block).align_left(),
                ContentType::text(display).align_right().css("group-aria-busy:hidden"),
            ],
            signal: Rc::new(Pages::new_collection_page(signal_check)) as Rc<dyn Switcher>,
            focus_color_change: false,
            background_image: darken_sidebar_background(COLLECTION_PIC),
            background_size: "cover",
            extended_css_class: "bg-background object-cover transition-all delay-[25ms] group-aria-expanded:w-20 group-aria-expanded:min-h-20 group-aria-expanded:p-0"
        }
    }
}

fn darken_sidebar_background(s: impl ToString) -> String {
    format!("linear-gradient(to right, rgba(25, 25, 25, 0.8) 0%, rgba(25, 25, 25, 1) 68%, rgba(25, 25, 25, 1) 100%),url(\"{}\")", s.to_string())
}
