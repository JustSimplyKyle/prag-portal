#![allow(non_snake_case)]
use std::time::Duration;
use tailwind_fuse::*;

use dioxus::prelude::*;
use log::LevelFilter;
use manganis::ImageAsset;
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Layout {},
}

static ACTIVE: GlobalSignal<(String, Option<String>)> =
    GlobalSignal::new(|| (String::from("main-page"), None));

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string())
        .with_menu(None);
    // let cfg = dioxus::web::Config::new();
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
    // LaunchBuilder::new().with_cfg(cfg).launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        // link { rel: "stylesheet", href: "public/tailwind.css" }
        div { class: "bg-deep-background min-h-screen min-w-full",
            div { class: "[&_*]:transform-gpu", Router::<Route> {} }
        }
    }
}

#[component]
fn Collections() -> Element {
    rsx! {
        div { class: "bg-background min-h-screen rounded-xl p-8 w-full",
            div {
                FatButton {
                    roundness: Roundness::Top,
                    string_placements: vec![
                        ContentType::text("Collections!").align_left(),
                        ContentType::text("bruh").align_right(),
                    ],
                    extended_css_class: "bg-deep-background"
                }
            }
        }
    }
    
}

#[component]
fn Explore() -> Element {
    rsx! {
        div { class: "bg-background min-h-screen rounded-xl p-8 w-full",
            div {
                FatButton {
                    roundness: Roundness::Top,
                    string_placements: vec![
                        ContentType::text("Explore").align_left(),
                        ContentType::text("thumbsup").align_right(),
                    ],
                    extended_css_class: "bg-deep-background"
                }
            }
        }
    }
}

#[component]
fn DownloadProgress() -> Element {
    rsx! {
        div { class: "bg-background min-h-screen rounded-xl p-8 w-full",
            div {
                FatButton {
                    roundness: Roundness::Top,
                    string_placements: vec![
                        ContentType::text("Progress").align_left(),
                        ContentType::text("stop").align_right(),
                    ],
                    extended_css_class: "bg-deep-background"
                }
            }
        }
    }
}

#[component]
fn Layout() -> Element {
    let selected = ACTIVE().0;
    let prev = ACTIVE().1;
    dbg!(&prev);
    rsx! {
        div { class: "flex flex-col gap-[20px]",
            div {}
            div {
                class: "group flex overflow-hidden",
                "data-selected": &*selected,
                "data-prev": prev.unwrap_or_else(String::new),
                SideBar {}
                div { class: "w-dvw min-h-screen relative *:overflow-scroll",
                    div { class: "absolute inset-0 z-0 min-h-full animation-[main-page^slideDown^explore^slideOutUp] animation-[main-page^slideDown^collections^slideOutUp]",
                        MainPage {}
                    }
                    div { class: "absolute inset-0 z-0 min-h-full animation-[explore^slideUp^main-page^slideOutDown] animation-[explore^slideDown^collections^slideOutUp]",
                        Explore {}
                    }
                    div { class: "absolute inset-0 z-0 min-h-full animation-[collections^slideUp^explore^slideOutDown] animation-[collections^slideUp^main-page^slideOutDown]",
                        Collections {}
                    }
                    // div { class: "absolute inset-0 z-0 min-h-full min-w-full animation-[progress^slideLeft] group-data-[prev=progress]:start-[100dvw] group-data-[prev=progress]:z-[100] group-data-[prev=progress]:animate-slideRight",
                    //     DownloadProgress {}
                    // }
                    div { class: "absolute inset-0 z-0 min-h-full min-w-full flyinout-[progress]",
                        DownloadProgress {}
                    }
                }
            }
        }
    }
}
#[component]
fn SideBar() -> Element {
    let mut active_signal = use_signal(|| 0);
    let mut expanded = use_signal(|| false);
    let delayed_expanded = use_resource(move || async move {
        tokio::time::sleep(Duration::from_millis(170)).await;
        expanded()
    });
    const HOME: &str = manganis::mg!(file("./public/home.svg"));
    const EXPLORE: &str = manganis::mg!(file("./public/explore.svg"));
    const COLLECTIONS: &str = manganis::mg!(file("./public/collections.svg"));
    const SIM_CARD: &str = manganis::mg!(file("./public/sim_card_download.svg"));
    let fat_button = |roundness, svg, string: &str, active, onclick: Option<EventHandler>| {
        rsx! {
            div {
                FatButton {
                    roundness,
                    string_placements: vec![
                        ContentType::svg(svg).align_left(),
                        ContentType::text(string).css("group-aria-busy:hidden").align_right(),
                    ],
                    signal: (active),
                    onclick,
                    extended_css_class: "group-aria-expanded:pr-5"
                }
            }
        }
    };
    let onclick = move |_| {
        switch_active("collections".into())(());
        expanded.toggle()
    };
    rsx! {
        div { class: "flex flex-col place-content-start mx-5",
            div {
                class: "w-[300px] space-y-5 ease-in-out transition-all duration-500 aria-expanded:w-[80px] group",
                aria_expanded: !expanded(),
                aria_busy: !delayed_expanded().unwrap_or(false),
                div { class: "flex flex-col space-y-1",
                    {fat_button(Roundness::Top, HOME, "首頁", "main-page", None)},
                    {fat_button(Roundness::None, EXPLORE, "探索", "explore", None)},
                    {fat_button(Roundness::Bottom, COLLECTIONS, "收藏庫", "collections", Some(onclick.into()))}
                }
                div { class: "flex flex-col space-y-1",
                    FatButton {
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
                        signal: "progress",
                        extended_css_class: "group/active items-center",
                        onclick: move |_| {
                            let prev = ACTIVE().1;
                            if ACTIVE().0 == "progress" {
                                if let Some(prev) = prev {
                                    switch_active(prev)(());
                                    ACTIVE.write().1 = Some("progress".into());
                                }
                            } else {
                                switch_active("progress".into())(());
                            }
                        }
                    }
                }
            }
        }
    }
}
#[component]
fn MainPage() -> Element {
    rsx! {
        div { class: "bg-background min-h-screen rounded-xl p-8 w-full",
            div { class: "flex flex-col space-y-[20px] transition-all xl:items-center xl:*:w-[1180px]",
                SuggestionPage {}
                div { CollectionsPage {} }
            }
        }
    }
}

#[component]
pub fn CollectionsPage() -> Element {
    const STAR: &str = manganis::mg!(file("./public/award_star.svg"));
    const ARROW_LEFT: &str = manganis::mg!(file("./public/keyboard_arrow_left.svg"));
    const ARROW_RIGHT: &str = manganis::mg!(file("./public/keyboard_arrow_right.svg"));
    const COLLECTION_PIC: ImageAsset =
        manganis::mg!(image("./public/pic1.png").format(ImageType::Avif));
    let picture_builder = |main: &str, hint: &str, pic| {
        rsx! {
            div { class: "relative",
                img {
                    class: "min-h-full min-w-full object-cover rounded-[5px]",
                    src: pic
                }
                div { class: "absolute inset-0 bg-gradient-to-t from-deep-background to-23%" }
                div { class: "absolute inset-0 px-5 pt-5 pb-0 flex flex-col justify-end items-start",
                    div { class: "p-0 m-0 text-3xl font-bold", {main} }
                    div { class: "p-0 m-0 text-[15px] text-white text-opacity-50", {hint} }
                }
            }
        }
    };
    rsx! {
        div { class: "flex flex-col space-x-0",
            FatButton {
                roundness: Roundness::Top,
                string_placements: vec![
                    Contents::new(
                        vec![
                            ContentType::text("我的錦集").css("text-3xl"),
                            ContentType::hint("你最愛的收藏都在這裡"),
                        ],
                        Alignment::Left,
                    ),
                    Contents::new(
                        vec![
                            ContentType::svg(ARROW_LEFT),
                            ContentType::svg(STAR),
                            ContentType::svg(ARROW_RIGHT),
                        ],
                        Alignment::Right,
                    ),
                ],
                extended_css_class: "bg-deep-background px-[30px] pt-[30px] pb-0 mb-0",
                is_button: false
            }
            div { class: ButtonClass::builder()
                    .roundness(Roundness::Bottom)
                    .with_class("bg-deep-background min-w-screen px-0"),
                div { class: "flex space-x-0 *:h-[280px] *:w-[280px]",
                    {picture_builder("創世幻想", "不久前開啟•由我建立", COLLECTION_PIC )}
                }
            }
        }
    }
}

#[component]
pub fn SuggestionPage() -> Element {
    const BLOCK: &str = manganis::mg!(file("./public/block.svg"));
    const EXPAND_CONTENT: &str = manganis::mg!(file("./public/expand_content.svg"));
    const ICON: &str = manganis::mg!(file("./public/icon.svg"));
    const IMG: ImageAsset = manganis::mg!(image("./public/project.png"));

    let right_css =
        "bg-background px-[25px] min-h-[52px] drop-shadow-lg rounded-full inline-flex items-center";

    let string_placements = [
        vec![
            Contents::new(
                [
                    ContentType::text("建議動作"),
                    ContentType::hint("你還沒完成快速設定，我們建議你盡快完成"),
                ],
                Alignment::Left,
            ),
            ContentType::svg(EXPAND_CONTENT)
                .css(right_css)
                .align_right(),
        ],
        vec![
            Contents::new(
                [
                    ContentType::text("需要幫助？"),
                    ContentType::hint("查看使用手冊與教學"),
                ],
                Alignment::Left,
            ),
            ContentType::svg(EXPAND_CONTENT)
                .css(right_css)
                .align_right(),
        ],
        vec![
            Contents::new(
                [
                    ContentType::text("探索內容"),
                    ContentType::hint("開始探索 Minecraft 的第三方社群內容"),
                ],
                Alignment::Left,
            ),
            ContentType::text("F").align_right(),
        ],
        vec![
            Contents::new(
                [
                    ContentType::text("創造中心"),
                    ContentType::hint("建立你的個人化收藏"),
                ],
                Alignment::Left,
            ),
            ContentType::text("F").align_right(),
        ],
        vec![
            Contents::new(
                [
                    ContentType::text("打造個人化收藏"),
                    ContentType::hint("你可以透過風格化功能來裝飾你的收藏"),
                ],
                Alignment::Left,
            ),
            ContentType::text("F").align_right(),
        ],
        vec![
            Contents::new(
                [
                    ContentType::text("建議動作"),
                    ContentType::hint("啟動器更新已經準備就緒"),
                ],
                Alignment::Left,
            ),
            ContentType::text("F").align_right(),
        ],
    ];
    let len = string_placements.len();
    rsx! {
        div { class: "flex space-x-2.5 max-w-fit h-[50px]",
            SimplePillButton {
                string_placements: [
                    ContentType::text("建議：快速設定").align_left(),
                    ContentType::svg(BLOCK)
                        .css(
                            "drop-shadow-lg bg-background w-10 h-10 rounded-full inline-flex justify-center items-center",
                        )
                        .align_right(),
                ]
            }
            SimplePillButton {
                string_placements: [
                    ContentType::text("建議：更新提醒").align_left(),
                    ContentType::svg(BLOCK)
                        .css(
                            "drop-shadow-lg bg-background w-10 h-10 rounded-full inline-flex justify-center items-center",
                        )
                        .align_right(),
                ]
            }
            SimplePillButton { string_placements: [ContentType::text("使用手冊").align_center()] }
            SimplePillButton { string_placements: [ContentType::text("探索內容").align_center()] }
            SimplePillButton { string_placements: [ContentType::text("創造中心").align_center()] }
            SimplePillButton { string_placements: [ContentType::text("個人化收藏").align_center()] }
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
                        extended_css_class: "bg-deep-background text-3xl min-w-full px-[30px] py-[25px]",
                        is_button: false
                    }
                }
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct FatButtonProps {
    roundness: Roundness,
    string_placements: Vec<Contents>,
    signal: Option<(String)>,
    #[props(default = String::new())]
    extended_css_class: String,
    #[props(default = true)]
    is_button: bool,
    onclick: Option<EventHandler>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[derive(TwClass, Clone, Copy)]
#[tw(
    class = "transition-all ease-in-out drop-shadow-lg duration-300 aria-selected:bg-white aria-selected:text-black text-white min-w-full bg-background text-2xl p-5 font-bold"
)]
pub struct ButtonClass {
    pub roundness: Roundness,
}

fn switch_active(x: String) -> impl FnMut(()) -> () {
    move |_| {
        let prev = ACTIVE().0;
        if &prev != &x {
            ACTIVE.write().1 = Some(prev);
        }
        ACTIVE.write().0 = x.clone();
    }
}

#[component]
/// onclick and signal will both run if both given.
pub fn FatButton(props: FatButtonProps) -> Element {
    let FatButtonProps {
        roundness,
        string_placements,
        signal,
        extended_css_class,
        is_button,
        onclick,
        attributes,
    } = props;
    let added = [
        if string_placements.len() == 1 {
            ""
        } else {
            "pr-8"
        },
        if string_placements.len() == 1 {
            "flex justify-center"
        } else {
            "grid grid-flow-col justify-stretch"
        },
    ];
    let extended_css_class = tw_merge!(added.join(" "), extended_css_class);
    let class = ButtonClass { roundness }.with_class(extended_css_class);
    let aria_selected = Some(ACTIVE().0) == signal;
    rsx! {
        div {
            class,
            role: if is_button { "button" } else { "" },
            aria_selected,
            onclick: move |_| {
                if let Some(x) = onclick {
                    x(());
                } else if let Some(x) = &signal {
                    switch_active(x.clone())(());
                }
            },
            ..attributes,
            for x in string_placements {
                { x.get_element() }
            }
        }
    }
}
#[component]
pub fn SimplePillButton<const T: usize>(
    string_placements: [Contents; T],
    #[props(default = String::from("bg-deep-background"))] background_color_class: String,
) -> Element {
    let single = format!("px-[20px] py-[5px]");
    let dual = format!("p-[5px] pl-[20px] grid grid-flow-col justify-stretch space-x-2.5",);
    let space_class = if T == 1 { single } else { dual };
    let base_class = "rounded-full items-center text-white text-[17px]";
    rsx! {
        button { class: base_class, class: space_class, class: background_color_class,
            for x in string_placements {
                { x.get_element() }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, TwVariant)]
pub enum Roundness {
    #[tw(class = "rounded-t-3xl")]
    Top,
    #[tw(default, class = "")]
    None,
    #[tw(class = "rounded-b-3xl")]
    Bottom,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contents {
    contents: Vec<Content>,
    css: String,
    alignment: Alignment, // Positioning
}

impl Contents {
    pub fn new(contents: impl Into<Vec<Content>>, alignment: Alignment) -> Self {
        Self {
            contents: contents.into(),
            css: String::new(),
            alignment,
        }
    }
    pub fn css(mut self, css: impl Into<String>) -> Self {
        self.css = tw_merge!(self.css, css.into());
        self
    }
    pub fn get_element(self) -> Element {
        let alignment_class = self.alignment.get_alignment_class();
        rsx! {
            div { class: tw_merge!(alignment_class, self.css),
                for x in self.contents {
                    {x.get_element()}
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content {
    content: ContentType,
    css: String,
}

impl Content {
    // Constructor for ease of creation
    pub fn new(content: ContentType) -> Self {
        Self {
            content,
            css: String::new(),
        }
    }

    pub fn align_left(self) -> Contents {
        Contents {
            contents: vec![self],
            css: String::new(),
            alignment: Alignment::Left,
        }
    }

    pub fn align_right(self) -> Contents {
        Contents {
            contents: vec![self],
            css: String::new(),
            alignment: Alignment::Right,
        }
    }

    pub fn align_center(self) -> Contents {
        Contents {
            contents: vec![self],
            css: String::new(),
            alignment: Alignment::Center,
        }
    }

    pub fn align_custom(self, custom: impl Into<String>) -> Contents {
        Contents {
            contents: vec![self],
            css: String::new(),
            alignment: Alignment::Custom(custom.into()),
        }
    }

    // Method to apply additional CSS, modifying existing styling
    pub fn css(mut self, css: impl Into<String>) -> Self {
        self.css = tw_merge!(self.css, css.into());
        self
    }

    pub fn alignment(self, alignment: Alignment) -> Contents {
        Contents::new(vec![self], alignment)
    }

    pub fn get_element(self) -> Element {
        match self.content {
            ContentType::Svg(x) => {
                rsx! {
                    div { class: self.css,
                        object { r#type: "image/svg+xml", data: "{x}" }
                    }
                }
            }
            ContentType::Text(x) | ContentType::Hint(x) => {
                rsx! {
                    div { class: self.css, { x } }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    Svg(&'static str),
    Text(String),
    Hint(String),
}

impl ContentType {
    /// Constructs an SVG content.
    ///
    /// This function returns a `Content` instance rather than a `ContentType`.
    /// It is advised to use this constructor function for creating SVG content
    /// instead of directly constructing `ContentType`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let my_svg = ContentType::svg("my svg");
    /// assert_eq!(my_svg, Content::new(ContentType::Svg("my svg")));
    /// ```
    #[must_use]
    pub fn svg(string: &'static str) -> Content {
        let content_type = Self::Svg(string);
        let css = String::new();
        Content {
            content: content_type,
            css,
        }
    }

    /// Constructs a text content.
    ///
    /// This function returns a `Content` instance rather than a `ContentType`.
    /// It is advised to use this constructor function for creating text content
    /// instead of directly constructing `ContentType`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let my_text = ContentType::text("Hello, world!");
    /// assert_eq!(my_text, Content::new(ContentType::Text("Hello, world!")));
    /// ```
    #[must_use]
    pub fn text(string: impl Into<String>) -> Content {
        let content_type = Self::Text(string.into());
        let css = String::new();
        Content {
            content: content_type,
            css,
        }
    }

    /// Constructs a text content.
    ///
    /// This function returns a `Content` instance rather than a `ContentType`.
    /// It is advised to use this constructor function for creating text content
    /// instead of directly constructing `ContentType`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let my_text = ContentType::hint("Hello, world!");
    /// assert_eq!(my_text, Content::new(ContentType::Hint("Hello, world!")));
    /// ```
    #[must_use]
    pub fn hint(string: impl Into<String>) -> Content {
        let content_type = Self::Hint(string.into());
        let css = String::from("text-[17px] text-hint p-0");
        Content {
            content: content_type,
            css,
        }
    }
    /// Returns `true` if the content type is [`Svg`].
    ///
    /// [`Svg`]: ContentType::Svg
    #[must_use]
    pub fn is_svg(&self) -> bool {
        matches!(self, Self::Svg(..))
    }

    /// Returns `true` if the content type is [`Text`].
    ///
    /// [`Text`]: ContentType::Text
    #[must_use]
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text(..))
    }

    /// Returns `true` if the content type is [`Hint`].
    ///
    /// [`Hint`]: ContentType::Hint
    #[must_use]
    pub fn is_hint(&self) -> bool {
        matches!(self, Self::Hint(..))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Custom(String), // Allows for custom alignment classes
}

impl Alignment {
    pub fn get_alignment_class(&self) -> String {
        match self {
            Alignment::Left => "text-left",
            Alignment::Center => "text-center",
            Alignment::Right => "text-right flex justify-end items-center",
            Alignment::Custom(ref class) => class,
        }
        .into()
    }
}
