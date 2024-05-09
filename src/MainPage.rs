use dioxus::prelude::*;

use manganis::ImageAsset;
use tailwind_fuse::*;

use crate::BaseComponents::{
    Alignment, Button, ButtonClass, ContentType, Contents, FillMode, Roundness, Size,
};

pub const COLLECTION_PIC: ImageAsset =
    manganis::mg!(image("./public/pic1.png").format(ImageType::Avif));
pub const BLOCK: &str = manganis::mg!(file("./public/block.svg"));
pub const EXPAND_CONTENT: &str = manganis::mg!(file("./public/expand_content.svg"));
pub const ICON: &str = manganis::mg!(file("./public/icon.svg"));
pub const IMG: ImageAsset = manganis::mg!(image("./public/project.png"));
pub const STAR: &str = manganis::mg!(file("./public/award_star.svg"));
pub const ARROW_LEFT: &str = manganis::mg!(file("./public/keyboard_arrow_left.svg"));
pub const ARROW_RIGHT: &str = manganis::mg!(file("./public/keyboard_arrow_right.svg"));

#[component]
pub fn MainPage() -> Element {
    rsx! {
        SuggestionPage {}
        CollectionsPage {}
    }
}

/// Creates a Collection Block with a `280px` square, with a default roundness of `5px`
#[component]
pub fn CollectionBlock(
    #[props(into)] main_text: Option<String>,
    #[props(into)] hint: Option<String>,
    picture: ImageAsset,
    #[props(default = true)] gradient: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default)] extended_class: String,
) -> Element {
    let (roundness, extended_class): (Vec<_>, Vec<_>) = extended_class
        .split_whitespace()
        .partition(|x| x.contains("rounded"));
    let extended_class = extended_class.join(" ");
    let mut img_class = String::from("h-full w-full object-cover rounded-[5px]");
    for x in roundness {
        img_class = tw_merge!(img_class, x);
    }
    rsx! {
        div {
            div {
                class: tw_merge!("relative h-[280px] w-[280px]", extended_class),
                ..attributes,
                img { class: img_class, src: picture }
                if gradient {
                    div { class: "absolute inset-0 bg-gradient-to-t from-deep-background to-23%" }
                }
                div { class: "absolute inset-0 px-5 pt-5 pb-[25px] flex flex-col gap-[15px] *:text-ellipsis overflow-hidden justify-end items-start",
                    div {
                        class: "text-3xl leading-normal capsize text-white font-bold",
                        {main_text}
                    }
                    div {
                        class: "text-[15px] leading-normal capsize text-white text-opacity-50",
                        {hint}
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
        div { class: "flex space-x-2.5 max-w-fit h-[50px]",
            Button {
                roundness: Roundness::Pill,
                size: Size::Small,
                fill_mode: FillMode::Fit,
                string_placements: vec![
                    ContentType::text("建議：快速設定").align_left(),
                    ContentType::svg(BLOCK)
                        .css(
                            "drop-shadow-lg bg-background w-10 h-10 rounded-full inline-flex justify-center items-center",
                        )
                        .align_right(),
                ]
            }
            Button {
                roundness: Roundness::Pill,
                fill_mode: FillMode::Fit,
                size: Size::Small,
                string_placements: vec![
                    ContentType::text("建議：更新提醒").align_left(),
                    ContentType::svg(BLOCK)
                        .css(
                            "drop-shadow-lg bg-background w-10 h-10 rounded-full inline-flex justify-center items-center",
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
        div { class: "flex space-x-0 lg:space-x-[20px] justify-center",
            div { class: "relative hidden shrink-0 lg:block shrink-0 h-[450px] w-[450px] shadow rounded",
                img { class: "absolute inset-0 z-0 rounded-[20px]", src: IMG }
                div { class: "absolute inset-0 z-50 flex justify-center items-center bg-gradient-to-t from-deep-background to-deep-background min-h-full max-h-full rounded-[20px]",
                    div {
                        span { class: "text-lime-300 text-6xl font-bold font-['GenSenRounded TW'] leading-[78px] tracking-[6px]",
                            "探索  創造"
                            br {}
                        }
                        span { class: "text-white text-6xl font-normal font-['GenSenRounded TW'] leading-[78px] tracking-[6px]",
                            "無窮  無限"
                            br {}
                            "創作  可能"
                        }
                    }
                }
                div { class: "absolute inset-0 z-20 self-stretch inline-flex justify-center items-center",
                    object { r#type: "image/svg+xml", data: ICON }
                }
            }
            div { class: "max-h-[450px] grid-flow-row justify-center content-evenly items-center w-full overflow-scroll space-y-1 p-0",
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
        div { class: "flex flex-col space-x-0",
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
            div { class: ButtonClass::builder().roundness(Roundness::Bottom).with_class("min-w-screen p-0"),
                div { class: "flex space-x-[3px] overflow-scroll",
                    CollectionBlock {
                        main_text: "創世幻想",
                        hint: "不久前開啟•由我建立",
                        picture: COLLECTION_PIC
                    }
                    CollectionBlock {
                        main_text: "創世幻想arstarstat",
                        hint: "不久前開啟•由我建立",
                        picture: COLLECTION_PIC
                    }
                    CollectionBlock {
                        main_text: "創世幻想",
                        hint: "不久前開啟•由我建立",
                        picture: COLLECTION_PIC
                    }
                    CollectionBlock {
                        main_text: "創世幻想",
                        hint: "不久前開啟•由我建立",
                        picture: COLLECTION_PIC
                    }
                    CollectionBlock {
                        main_text: "創世幻想",
                        hint: "不久前開啟•由我建立",
                        picture: COLLECTION_PIC
                    }
                    CollectionBlock {
                        main_text: "創世幻想",
                        hint: "不久前開啟•由我建立",
                        picture: COLLECTION_PIC
                    }
                    CollectionBlock {
                        main_text: "創世幻想",
                        hint: "不久前開啟•由我建立",
                        picture: COLLECTION_PIC
                    }
                    CollectionBlock {
                        main_text: "創世幻想",
                        hint: "不久前開啟•由我建立",
                        picture: COLLECTION_PIC
                    }
                }
            }
        }
    }
}
