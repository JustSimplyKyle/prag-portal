use std::time::Duration;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use manganis::ImageAsset;
use rust_lib::api::{
    backend_exclusive::vanilla::launcher::LoggerEvent,
    shared_resources::{collection::CollectionId, entry::STORAGE},
};
use tailwind_fuse::*;

use crate::{
    collection_display::HORIZ,
    collections::CollectionContext,
    text_scroller::use_text_scroller,
    use_error_handler,
    BaseComponents::{
        atoms::button::{Button, ButtonClass, FillMode, Roundness, Size},
        molecules::switcher::StateSwitcher,
        string_placements::{Alignment, ContentType, Contents},
    },
    Pages, ARROW_RIGHT,
};

// pub const COLLECTION_PIC: ImageAsset =
//     manganis::mg!(image("./public/pic1.png").format(ImageType::Avif).preload());
pub const BLOCK: &str = manganis::mg!("./public/block.svg");
pub const EXPAND_CONTENT: &str = manganis::mg!("./public/expand_content.svg");
pub const ICON: &str = manganis::mg!("./public/icon.svg");
pub const LIGHTNING: &str = manganis::mg!("./public/lightning.svg");
pub const IMG: ImageAsset = manganis::mg!(image("./public/project.png"));
pub const STAR: &str = manganis::mg!("./public/award_star.svg");
pub const ARROW_LEFT: &str = manganis::mg!("./public/keyboard_arrow_left.svg");

pub static START: Component = |()| {
    rsx! { svg { height: "31", xmlns: "http://www.w3.org/2000/svg", width: "31", fill: "none", "viewBox": "0 0 31 31", mask { "maskUnits": "userSpaceOnUse", y: "0", width: "31", style: "mask-type:alpha", height: "31", x: "0", id: "mask0_3548_4928", rect { height: "30", fill: "#D9D9D9", x: "0.5", y: "0.5", width: "30", } } g { mask: "url(#mask0_3548_4928)", path { d: "M10 21.7488V9.25117C10 8.86102 10.126 8.55491 10.3781 8.33282C10.6302 8.11094 10.9236 8 11.2582 8C11.3703 8 11.4832 8.01457 11.5967 8.04372C11.7106 8.07286 11.8231 8.12211 11.9344 8.19146L21.5602 14.4776C21.7452 14.598 21.8906 14.7512 21.9964 14.9371C22.1019 15.1228 22.1546 15.3122 22.1546 15.5051C22.1546 15.6981 22.1019 15.8874 21.9964 16.0731C21.8906 16.259 21.7452 16.4088 21.5602 16.5224L11.934 22.8127C11.8228 22.882 11.7088 22.9306 11.592 22.9585C11.4752 22.9862 11.3619 23 11.2521 23C10.9162 23 10.6235 22.8891 10.374 22.6672C10.1247 22.4451 10 22.139 10 21.7488Z", fill: "white", } } } }
};

pub static INVERTED_STAR: Component = |()| {
    rsx! {
         svg { height: "31", "viewBox": "0 0 31 31", xmlns: "http://www.w3.org/2000/svg", width: "31", fill: "none", mask { width: "31", style: "mask-type:alpha", x: "0", "maskUnits": "userSpaceOnUse", y: "0", height: "31", id: "mask0_3548_4919", rect { x: "0.5", width: "30", y: "0.5", fill: "#D9D9D9", height: "30", } } g { mask: "url(#mask0_3548_4919)", path { fill: "#191919", d: "M15.504 18.2343L17.5634 19.5047C17.8113 19.6392 18.0453 19.6237 18.2653 19.4581C18.4851 19.2927 18.5588 19.0779 18.4865 18.8137L17.9375 16.4703L19.8075 14.8681C20.0173 14.677 20.08 14.4543 19.9956 14.2C19.9112 13.9454 19.7412 13.806 19.4856 13.7818L17.0503 13.5759L16.0956 11.34C15.9818 11.0731 15.787 10.9397 15.5112 10.9397C15.2354 10.9397 15.0404 11.0731 14.9262 11.34L13.9659 13.584L11.5168 13.7906C11.2595 13.8148 11.0887 13.953 11.0043 14.2053C10.92 14.4576 10.9827 14.6785 11.1925 14.8681L13.0625 16.4703L12.5134 18.8137C12.4411 19.0779 12.5131 19.2927 12.7293 19.4581C12.9458 19.6237 13.1843 19.6392 13.445 19.5047L15.504 18.2343ZM11.8375 24.4415H9.00402C8.32507 24.4415 7.74767 24.2037 7.27184 23.7281C6.79621 23.2523 6.5584 22.6749 6.5584 21.9959V19.1625L4.60402 17.2234C4.33986 16.9749 4.156 16.7051 4.05246 16.414C3.94913 16.1232 3.89746 15.8185 3.89746 15.5C3.89746 15.1814 3.94913 14.8767 4.05246 14.5859C4.156 14.2949 4.33986 14.0251 4.60402 13.7765L6.5584 11.8375V9.00402C6.5584 8.32507 6.79621 7.74767 7.27184 7.27184C7.74767 6.79621 8.32507 6.5584 9.00402 6.5584H11.8375L13.7765 4.60402C14.0301 4.33986 14.3011 4.156 14.5896 4.05246C14.878 3.94913 15.1814 3.89746 15.5 3.89746C15.8185 3.89746 16.1219 3.94913 16.4103 4.05246C16.6988 4.156 16.9699 4.33986 17.2234 4.60402L19.1625 6.5584H21.9959C22.6749 6.5584 23.2523 6.79621 23.7281 7.27184C24.2037 7.74767 24.4415 8.32507 24.4415 9.00402V11.8375L26.3959 13.7765C26.6601 14.0301 26.8439 14.3011 26.9475 14.5896C27.0508 14.878 27.1025 15.1814 27.1025 15.5C27.1025 15.8185 27.0508 16.1219 26.9475 16.4103C26.8439 16.6988 26.6601 16.9699 26.3959 17.2234L24.4415 19.1625V21.9959C24.4415 22.6749 24.2037 23.2523 23.7281 23.7281C23.2523 24.2037 22.6749 24.4415 21.9959 24.4415H19.1625L17.2234 26.3959C16.9749 26.6601 16.7051 26.8439 16.414 26.9475C16.1232 27.0508 15.8185 27.1025 15.5 27.1025C15.1814 27.1025 14.8767 27.0508 14.5859 26.9475C14.2949 26.8439 14.0251 26.6601 13.7765 26.3959L11.8375 24.4415Z", } } }
    }
};

#[component]
pub fn MainPage() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-[10px] bg-background rounded-[30px] p-8",
            SuggestionPage {
            }
            CollectionsPage {

            }
        }
    }
}

#[must_use]
pub fn use_delayed_hover(onhover: Signal<bool>) -> Signal<bool> {
    let mut visibility = use_signal(|| false);
    let _ = use_resource(move || async move {
        if !onhover() {
            tokio::time::sleep(Duration::from_millis(100)).await;
            if !*onhover.peek() {
                visibility.set(false);
            }
        }
    });
    visibility
}

#[must_use]
pub fn use_delayed_signal<T: Clone>(signal: Signal<T>) -> Signal<Option<T>> {
    let mut new_signal = use_signal(|| None);
    let _ = use_resource(move || async move {
        let x = signal();
        tokio::time::sleep(Duration::from_millis(100)).await;
        new_signal.set(Some(x));
    });
    new_signal
}

/// Creates a Collection Block with a `280px` square, with a default roundness of `5px`
#[component]
pub fn CollectionBlock(
    collection_id: ReadOnlySignal<CollectionId>,
    #[props(default = false)] fat: bool,
    #[props(default = true)] gradient: bool,
    #[props(default)] z_index: String,
    #[props(default)] extended_class: String,
) -> Element {
    let collection = collection_id().get_collection();
    let picture_path = collection
        .read()
        .picture_path()
        .to_string_lossy()
        .to_string();
    let (mut onmounted, status, style) = use_text_scroller();
    let class = tw_merge!("size-[280px] max-w-[280px] min-w-[280px]", extended_class);
    let class = if fat {
        tw_merge!(class, "max-w-full min-w-full w-full col-span-2")
    } else {
        class
    };

    let mut onhover = use_signal(|| false);
    let mut menu_visibility = use_delayed_hover(onhover);

    let delayed_visibility = use_delayed_signal(menu_visibility);

    let mut launch_game_hover = use_signal(|| false);

    let mut error_handler = use_error_handler();

    let log = use_signal_sync(|| LoggerEvent::default());

    use_effect(move || {
        info!("{}", log.read());
    });

    rsx! {
        button {
            class,
            style: "z-index:{z_index}; background: radial-gradient(273.29% 100% at 0% 100%, #0E0E0E 22.75%, rgba(14, 14, 14, 0.00) 100%), url('{picture_path}') lightgray 50% / cover no-repeat;",
            aria_selected: status(),
            onclick: move |_| {
                if !launch_game_hover() {
                    Pages::collection_display(collection_id())
                        .switch_active_to_self();
                }
            },
            div {
                class: "absolute inset-0 px-5 pt-5 pb-[25px] grid grid-flow-row *:justify-self-start justify-stretch items-stretch",
                div {
                    class: "self-start w-full grid grid-flow-col z-10 justify-stretch",
                    div {
                        class: "flex justify-center items-center bg-green rounded-[15px] justify-self-start size-[45px]",
                        INVERTED_STAR {}
                    }
                    div {
                        class: "justify-self-end flex gap-[7px]",
                        button {
                            class: "flex z-[1000000] rounded-[15px] justify-center items-center bg-background size-[45px]",
                            onpointerenter: move |_| {
                                launch_game_hover.set(true);
                            },
                            onpointerleave: move |_| {
                                launch_game_hover.set(false);
                            },
                            onclick: move |x| async move {
                                x.stop_propagation();
                                let mut collection = collection_id().get_collection()();
                                let binding = async move {
                                    collection.launch_game(log).await?;
                                    collection_id().replace(collection)?;
                                    Ok(())
                                };
                                if let Err(err) = binding.await {
                                    error_handler.set(Err(err));
                                }
                            },
                            START {}
                        }
                        div {
                            class: "flex rounded-[15px] justify-center items-center group bg-background z-10 relative size-[45px]",
                            "data-visible": menu_visibility(),
                            "data-delayed-visible": delayed_visibility().unwrap_or(false),
                            onclick: move |x| {
                                x.stop_propagation();
                                menu_visibility.set(true);
                            },
                            onpointerenter: move |_| {
                                onhover.set(true);
                            },
                            onpointerleave: move |_| {
                                onhover.set(false);
                            },
                            {ContentType::svg(HORIZ).css("inline-flex justify-center items-center svg-[30px]")},
                            CollectionContext {
                                class: "transition-all group-data-[delayed-visible=false]:hidden group-data-[visible=false]:opacity-0"
                            }
                        }
                    }
                }
                div {
                    class: "self-end grid grid-flow-row items-stretch *:justify-self-start justify-start gap-[15px]",
                    div {
                        class: "group-hover:group-aria-selected:animate-scroll-left w-full text-3xl text-white text-nowrap text-left font-bold overflow-x-clip trim",
                        style: style(),
                        onmounted: move |x| {
                            onmounted.set(Some(x));
                        },
                        {collection.read().display_name().clone()}
                    }
                    div {
                        class: "text-[15px] text-hint text-ellipsis text-nowrap trim",
                        "遊玩中•由我建立"
                    }
                }
            }
        }
    }
}

#[component]
fn SuggestionPage() -> Element {
    let right_css =
        "bg-zinc-800 h-[52px] px-[25px] drop-shadow-lg rounded-full inline-flex items-center";

    let suggested_moves_placementss = [
        vec![
            Contents::new(
                [
                    ContentType::text("建議動作").css("text-3xl"),
                    ContentType::hint("你還沒完成快速設定，我們建議你盡快完成"),
                ],
                Alignment::Left,
            )
            .css("flex flex-col gap-[15px]"),
            ContentType::svg(EXPAND_CONTENT)
                .css(right_css)
                .align_right(),
        ],
        vec![
            Contents::new(
                [
                    ContentType::text("需要幫助？").css("text-3xl"),
                    ContentType::hint("查看使用手冊與教學"),
                ],
                Alignment::Left,
            )
            .css("flex flex-col gap-[15px]"),
            ContentType::svg(EXPAND_CONTENT)
                .css(right_css)
                .align_right(),
        ],
        vec![
            Contents::new(
                [
                    ContentType::text("探索內容").css("text-3xl"),
                    ContentType::hint("開始探索 Minecraft 的第三方社群內容"),
                ],
                Alignment::Left,
            )
            .css("flex flex-col gap-[15px]"),
            ContentType::text("F").align_right(),
        ],
        vec![
            Contents::new(
                [
                    ContentType::text("創造中心").css("text-3xl"),
                    ContentType::hint("建立你的個人化收藏"),
                ],
                Alignment::Left,
            )
            .css("flex flex-col gap-[15px]"),
            ContentType::text("F").align_right(),
        ],
        vec![
            Contents::new(
                [
                    ContentType::text("打造個人化收藏").css("text-3xl"),
                    ContentType::hint("你可以透過風格化功能來裝飾你的收藏"),
                ],
                Alignment::Left,
            )
            .css("flex flex-col gap-[15px]"),
            ContentType::text("F").align_right(),
        ],
        vec![
            Contents::new(
                [
                    ContentType::text("建議動作").css("text-3xl"),
                    ContentType::hint("啟動器更新已經準備就緒"),
                ],
                Alignment::Left,
            )
            .css("flex flex-col gap-[15px]"),
            ContentType::text("F").align_right(),
        ],
    ];
    let len = suggested_moves_placementss.len();
    rsx! {
        div {
            class: "flex space-x-2.5 max-w-fit h-[50px]",
            Button {
                roundness: Roundness::Pill,
                size: Size::Small,
                extended_css_class: "pr-[5px]",
                fill_mode: FillMode::Fit,
                string_placements: vec![
                    ContentType::text("建議：快速設定").align_left(),
                    ContentType::svg(BLOCK)
                        .css(
                            "drop-shadow-lg bg-background svg-[40px] inline-flex justify-center rounded-full",
                        )
                        .align_right(),
                ]
            }
            Button {
                roundness: Roundness::Pill,
                fill_mode: FillMode::Fit,
                extended_css_class: "pr-[5px]",
                size: Size::Small,
                string_placements: vec![
                    ContentType::text("建議：更新提醒").align_left(),
                    ContentType::svg(BLOCK)
                        .css(
                            "drop-shadow-lg bg-background svg-[40px] rounded-full inline-flex justify-center",
                        )
                        .align_right(),
                ]
            }
            Button {
                roundness: Roundness::Pill,
                fill_mode: FillMode::Fit,
                size: Size::Small,
                string_placements: vec![ContentType::text("使用手冊").align_center()]
            }
            Button {
                roundness: Roundness::Pill,
                fill_mode: FillMode::Fit,
                size: Size::Small,
                string_placements: vec![ContentType::text("探索內容").align_center()]
            }
            Button {
                roundness: Roundness::Pill,
                fill_mode: FillMode::Fit,
                size: Size::Small,
                string_placements: vec![ContentType::text("創造中心").align_center()]
            }
            Button {
                roundness: Roundness::Pill,
                fill_mode: FillMode::Fit,
                size: Size::Small,
                string_placements: vec![ContentType::text("個人化收藏").align_center()]
            }
        }
        div {
            class: "flex space-x-0 lg:space-x-[20px] justify-center",
            div {
                class: "relative hidden shrink-0 lg:block shrink-0 h-[450px] w-[450px] shadow rounded",
                img {
                    class: "absolute inset-0 z-0 rounded-[20px]",
                    src: IMG.to_string()
                }
                div {
                    class: "absolute inset-0 z-50 flex justify-center items-center bg-gradient-to-t from-deep-background to-deep-background min-h-full max-h-full rounded-[20px]",
                    div {
                        span {
                            class: "text-lime-300 text-6xl font-bold leading-[78px] tracking-[6px]",
                            "探索  創造"
                            br {

                            }
                        }
                        span {
                            class: "text-white text-6xl font-normal leading-[78px] tracking-[6px]",
                            "無窮  無限"
                            br {

                            }
                            "創作  可能"
                        }
                    }
                }
                div {
                    class: "absolute inset-0 z-20 self-stretch inline-flex justify-center items-center",
                    object {
                        r#type: "image/svg+xml",
                        data: ICON
                    }
                }
            }
            div {
                class: "max-h-[450px] grid-flow-row justify-center content-evenly items-center w-full overflow-scroll space-y-1 p-0",
                for (u , x) in suggested_moves_placementss.into_iter().enumerate() {
                    Button {
                        roundness: if u == 0 {
                            Roundness::Top
                        } else if u == len - 1 {
                            Roundness::Bottom
                        } else {
                            Roundness::None
                        },
                        string_placements: x,
                        extended_css_class: "bg-deep-background min-w-full px-[30px] py-[35px]",
                        clickable: false
                    }
                }
            }
        }
    }
}

#[component]
fn CollectionsPage() -> Element {
    rsx! {
        div {
            class: "flex flex-col space-x-0 overflow-y-visible",
            Button {
                roundness: Roundness::Top,
                string_placements: vec![
                    Contents::new(
                            vec![
                                ContentType::text("我的錦集").css("text-[35px]"),
                                ContentType::hint("你最愛的收藏都在這裡")
                                    .css("text-[18px]"),
                            ],
                            Alignment::Left,
                        )
                        .css("flex flex-col gap-[20px]"),
                    Contents::new(
                        vec![
                            ContentType::svg(ARROW_LEFT),
                            ContentType::svg(STAR),
                            ContentType::svg(ARROW_RIGHT),
                        ],
                        Alignment::Right,
                    ),
                ],
                extended_css_class: "p-[30px] mb-0",
                clickable: false
            }
            div {
                class: ButtonClass::builder().roundness(Roundness::Bottom).with_class("flex-0 w-full p-0 overflow-y-visible"),
                div {
                    class: "flex space-x-[3px] overflow-x-scroll overflow-y-visible",
                    for collection_id in STORAGE.collections.read().keys().cloned() {
                        CollectionBlock {
                            collection_id
                        }
                    }
                }
            }
        }
    }
}
