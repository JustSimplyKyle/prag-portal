#![allow(non_snake_case)]
use dioxus::prelude::*;
use log::LevelFilter;
use manganis::ImageAsset;
use std::{any::type_name_of_val, time::Duration};
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Layout {},
}
fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string())
        .with_menu(None);
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        div { class: "bg-deep-background min-h-screen min-w-full",
            div { class: "font-['GenSenRounded TW']", Router::<Route> {} }
        }
    }
}
#[component]
fn Layout() -> Element {
    rsx! {
        div { class: "flex flex-col gap-[20px]",
            div {}
            div { class: "flex",
                SideBar {}
                MainPage {}
            }
        }
    }
}
#[component]
fn SideBar() -> Element {
    let active_signal = use_signal(|| 0);
    let mut expanded = use_signal(|| false);
    let delayed_expanded = use_resource(move || async move {
        tokio::time::sleep(Duration::from_millis(170)).await;
        expanded()
    });
    const HOME: &str = manganis::mg!(file("./public/home.svg"));
    const EXPLORE: &str = manganis::mg!(file("./public/explore.svg"));
    const COLLECTIONS: &str = manganis::mg!(file("./public/collections.svg"));
    let fat_button = |roundness, svg, string: &str, active_signal, num, onclick| {
        if let Some(onclick) = onclick {
            rsx! {
                div {
                    FatButton {
                        roundness,
                        string_placements: [
                            PositionType::Left(ContentType::Svg(svg).into()).into(),
                            PositionType::Right(string.into()).into(),
                        ],
                        active_signal,
                        num,
                        onclick,
                        extended_css_class: "group-aria-expanded:pr-5"
                    }
                }
            }
        } else {
            rsx! {
                div {
                    FatButton {
                        roundness,
                        string_placements: [
                            PositionType::Left(ContentType::Svg(svg).into()).into(),
                            PositionType::Right(string.into()).into(),
                        ],
                        active_signal,
                        num,
                        extended_css_class: "group-aria-expanded:pr-5"
                    }
                }
            }
        }
    };
    let onclick = move |_| expanded.toggle();
    rsx! {
        div { class: "flex flex-col place-content-start mx-5",
            div {
                class: "w-[300px] space-y-5 ease-in-out transition-all duration-500 aria-expanded:w-[80px] group",
                aria_expanded: !expanded(),
                aria_busy: !delayed_expanded().unwrap_or(false),
                div { class: "flex flex-col group-aria-busy:[&_.string]:hidden space-y-1",
                    {fat_button(Roundness::Top, HOME, "首頁", active_signal, 0, None)},
                    {fat_button(Roundness::None, EXPLORE, "探索", active_signal, 1, None)},
                    {fat_button(Roundness::Bottom, COLLECTIONS, "收藏庫", active_signal, 2, Some(onclick))}
                }
            }
        }
    }
}
#[component]
fn MainPage() -> Element {
    const BLOCK: &str = manganis::mg!(file("./public/block.svg"));
    const EXPAND_CONTENT: &str = manganis::mg!(file("./public/expand_content.svg"));
    const IMG: ImageAsset = manganis::mg!(image("./public/project.png"));
    let string_placements = [
        [
            PositionType::Left(
                    ContentTypes(
                        vec![
                            "建議動作".into(),
                            ContentType::Hint(
                                "你還沒完成快速設定，我們建議你盡快完成"
                                    .into(),
                            ),
                        ],
                    ),
                )
                .into(),
            PositionType::Right(ContentType::Svg(EXPAND_CONTENT).into())
                .with_css(
                    "bg-background px-[25px] min-h-[52px] drop-shadow-lg rounded-full inline-flex justify-self-end items-center",
                ),
        ],
        [
            PositionType::Left(
                    ContentTypes(
                        vec![
                            "需要幫助？".into(),
                            ContentType::Hint("查看使用手冊與教學".into()),
                        ],
                    ),
                )
                .into(),
            PositionType::Right(ContentType::Svg(EXPAND_CONTENT).into())
                .with_css(
                    "bg-background px-[25px] min-h-[52px] drop-shadow-lg rounded-full inline-flex justify-self-end items-center",
                ),
        ],
        [
            PositionType::Left(
                    ContentTypes(
                        vec![
                            "探索內容".into(),
                            ContentType::Hint(
                                "開始探索 Minecraft 的第三方社群內容".into(),
                            ),
                        ],
                    ),
                )
                .into(),
            PositionType::Right("F".into()).into(),
        ],
        [
            PositionType::Left(
                    ContentTypes(
                        vec![
                            "創造中心".into(),
                            ContentType::Hint("建立你的個人化收藏".into()),
                        ],
                    ),
                )
                .into(),
            PositionType::Right("F".into()).into(),
        ],
        [
            PositionType::Left(
                    ContentTypes(
                        vec![
                            "打造個人化收藏".into(),
                            ContentType::Hint(
                                "你可以透過風格化功能來裝飾你的收藏".into(),
                            ),
                        ],
                    ),
                )
                .into(),
            PositionType::Right("F".into()).into(),
        ],
        [
            PositionType::Left(
                    ContentTypes(
                        vec![
                            "建議動作".into(),
                            ContentType::Hint("啟動器更新已經準備就緒".into()),
                        ],
                    ),
                )
                .into(),
            PositionType::Right("F".into()).into(),
        ]
    ];
    let len = string_placements.len();
    rsx! {
        div { class: "bg-background min-h-[95vh] rounded-xl p-8 mr-5 w-[95vw]",
            div { class: "flex flex-col space-y-[20px]",
                div { class: "flex space-x-2.5 max-w-fit h-[50px]",
                    SimplePillButton {
                        string_placements: [
                            PositionType::Left("建議：快速設定".into()).into(),
                            PositionType::Right(ContentType::Svg(BLOCK).into())
                                .with_css(
                                    "drop-shadow-lg bg-background w-10 h-10 rounded-full inline-flex justify-center items-center",
                                ),
                        ]
                    }
                    SimplePillButton {
                        string_placements: [
                            PositionType::Left("建議：更新提醒".into()).into(),
                            PositionType::Right(ContentType::Svg(BLOCK).into())
                                .with_css(
                                    "drop-shadow-lg bg-background w-10 h-10 rounded-full inline-flex justify-center items-center",
                                ),
                        ]
                    }
                    SimplePillButton { string_placements: [PositionType::Center("使用手冊".into()).into()] }
                    SimplePillButton { string_placements: [PositionType::Center("探索內容".into()).into()] }
                    SimplePillButton { string_placements: [PositionType::Center("創造中心".into()).into()] }
                    SimplePillButton { string_placements: [PositionType::Center("個人化收藏".into()).into()] }
                }
                div { class: "flex space-x-[20px] h-[450px]",
                    div { class: "grow-0 shrink-0 w-[450px] shadow rounded",
                        div {
                            class: "flex justify-center items-center bg-gradient-to-b from-stone-950 to-stone-950 min-h-full max-h-full rounded-[20px] backdrop-opacity-80",
                            background_image: "linear-gradient(to bottom, var(--tw-gradient-stops));",
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
                    }
                    div { class: "grid-flow-row justify-center content-evenly items-center w-full overflow-scroll space-y-1 p-0",
                        for (u , x) in string_placements.into_iter().enumerate() {
                            FatButton {
                                roundness: if u == 0 {
                                    Roundness::Top
                                } else if u == len - 1 {
                                    Roundness::Bottom
                                } else {
                                    Roundness::None
                                },
                                string_placements: x,
                                background_color_class: "bg-deep-background",
                                text_size_class: "text-3xl",
                                extended_css_class: "min-w-full px-[30px] py-[25px]",
                                is_button: false
                            }
                        }
                    }
                }
            }
        }
    }
}
#[derive(Clone, Props, PartialEq)]
pub struct FatButtonProps<const K: usize> {
    roundness: Roundness,
    string_placements: [Position; K],
    active_signal: Option<Signal<usize>>,
    num: Option<usize>,
    #[props(default = String::from("bg-background"))]
    background_color_class: String,
    #[props(default = String::from("text-2xl"))]
    text_size_class: String,
    #[props(default = String::new())]
    extended_css_class: String,
    #[props(default = true)]
    is_button: bool,
    onclick: Option<EventHandler>,
}
#[component]
pub fn FatButton<const K: usize>(props: FatButtonProps<{ K }>) -> Element {
    let FatButtonProps {
        roundness,
        string_placements,
        active_signal,
        num,
        background_color_class,
        text_size_class,
        extended_css_class,
        is_button,
        onclick,
    } = props;
    let rounded_state = match roundness {
        Roundness::Top => "rounded-t-3xl",
        Roundness::None => "",
        Roundness::Bottom => "rounded-b-3xl",
    };
    let aria_selected = if let (Some(x), Some(num)) = (active_signal, num) {
        x() == num
    } else {
        false
    };
    rsx! {
        div {
            class: "transition-all ease-in-out drop-shadow-lg delay-75 duration-300 aria-selected:bg-white aria-selected:text-black text-white min-w-full items-center",
            class: if K == 1 { "p-5" } else { "p-5 pr-8" },
            class: if K == 1 { "flex justify-center" } else { "grid grid-flow-col justify-stretch" },
            class: text_size_class,
            class: extended_css_class,
            class: background_color_class,
            class: rounded_state,
            class: "[&_.hint]:text-[17px] [&_.hint]:text-hint",
            role: if is_button { "button" } else { "" },
            aria_selected: aria_selected,
            onclick: move |_| {
                if let (Some(mut active_signal), Some(num)) = (active_signal, num) {
                    *active_signal.write() = num;
                }
                if let Some(x) = onclick {
                    x(());
                }
            },
            for x in string_placements {
                { x.get_object() }
            }
        }
    }
}
#[component]
pub fn SimplePillButton<const T: usize>(
    string_placements: [Position; T],
    #[props(default = String::from("bg-deep-background"))] background_color_class: String,
) -> Element {
    let single = format!("px-[20px] py-[5px]");
    let dual = format!("p-[5px] pl-[20px] grid grid-flow-col justify-stretch space-x-2.5",);
    let space_class = if T == 1 { single } else { dual };
    let base_class = "rounded-full items-center text-white text-[17px]";
    rsx! {
        button { class: base_class, class: space_class, class: background_color_class,
            for x in string_placements {
                { x.get_object() }
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Roundness {
    Top,
    None,
    Bottom,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub position: PositionType,
    css: String,
}
impl From<PositionType> for Position {
    fn from(value: PositionType) -> Self {
        Self {
            position: value,
            css: String::new(),
        }
    }
}
impl Position {
    fn get_class_alignment(&self) -> String {
        match self.position {
            PositionType::Left(_) => "text-left",
            PositionType::Right(_) => "text-right",
            PositionType::Center(_) => "text-center",
            PositionType::Custom(_) => "",
        }
        .into()
    }
    pub fn with_css(self, css: impl Into<String>) -> Self {
        let css = css.into();
        Self {
            position: self.position,
            css,
        }
    }
    pub fn get_object(self) -> Element {
        let class = format!("{} {}", self.get_class_alignment(), self.css);
        match self.position {
            PositionType::Left(x) => {
                rsx! {
                    div { class: class, { x.get_element() } }
                }
            }
            PositionType::Right(x) => {
                rsx! {
                    div { class: class, { x.get_element() } }
                }
            }
            PositionType::Center(x) => {
                rsx! {
                    div { class: class, { x.get_element() } }
                }
            }
            PositionType::Custom(x) => x,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum PositionType {
    Left(ContentTypes),
    Center(ContentTypes),
    Right(ContentTypes),
    Custom(Element),
}
impl PositionType {
    pub fn with_css(self, css: impl Into<String>) -> Position {
        Into::<Position>::into(self).with_css(css)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentTypes(Vec<ContentType>);
impl From<ContentType> for ContentTypes {
    fn from(value: ContentType) -> Self {
        Self(vec![value])
    }
}
impl From<&str> for ContentTypes {
    fn from(value: &str) -> Self {
        Into::<ContentType>::into(value).into()
    }
}
impl From<String> for ContentTypes {
    fn from(value: String) -> Self {
        Into::<ContentType>::into(value).into()
    }
}
impl ContentTypes {
    fn get_element(self) -> Element {
        let t = |x: &ContentType| if x.is_hint() { "hint" } else { "main" };
        let p = self.0.into_iter().map(|x| {
            let class = t(&x);
            rsx! {
                div { class: class, { x.get_element() } }
            }
        });
        rsx! {
            div { { p } }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    Svg(&'static str),
    String(String),
    Hint(String),
}
impl From<String> for ContentType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for ContentType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl ContentType {
    pub fn get_element(self) -> Element {
        match self {
            Self::Svg(x) => {
                rsx! {
                    div { class: "svg", object { r#type: "image/svg+xml", data: "{x}" } }
                }
            }
            Self::String(x) | Self::Hint(x) => {
                rsx! {
                    div { class: "string", { x } }
                }
            }
        }
    }
    /// Returns `true` if the content type is [`String`].
    ///
    /// [`String`]: ContentType::String
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(..))
    }
    /// Returns `true` if the content type is [`Hint`].
    ///
    /// [`Hint`]: ContentType::Hint
    #[must_use]
    pub fn is_hint(&self) -> bool {
        matches!(self, Self::Hint(..))
    }
    /// Returns `true` if the content type is [`Svg`].
    ///
    /// [`Svg`]: ContentType::Svg
    #[must_use]
    pub fn is_svg(&self) -> bool {
        matches!(self, Self::Svg(..))
    }
}
