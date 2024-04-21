#![allow(non_snake_case)]
pub mod BaseComponents;
pub mod Collections;
pub mod MainPage;

use dioxus::desktop::tao::dpi::PhysicalSize;
use dioxus::desktop::WindowBuilder;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use tailwind_fuse::*;

use dioxus::prelude::*;
use log::LevelFilter;
use BaseComponents::ActiveCompare;

use crate::BaseComponents::{Alignment, Button, ContentType, FillMode, Roundness};
use crate::Collections::Collections;
use crate::MainPage::{CollectionBlock, MainPage, COLLECTION_PIC};

pub const HOME: &str = manganis::mg!(file("./public/home.svg"));
pub const EXPLORE: &str = manganis::mg!(file("./public/explore.svg"));
pub const SIDEBAR_COLLECTION: &str = manganis::mg!(file("./public/collections.svg"));
pub const ARROW_RIGHT: &str = manganis::mg!(file("./public/keyboard_arrow_right.svg"));
pub const SIM_CARD: &str = manganis::mg!(file("./public/sim_card_download.svg"));
pub const TAILWIND_STR_: &str = manganis::mg!(file("./public/tailwind.css"));

/// `(Pages)`: Current active page
/// `Option<Pages>`: Previous page
static ACTIVE_PAGE: GlobalSignal<(Pages, Option<Pages>)> =
    GlobalSignal::new(|| (Pages::MainPage, None));

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_window(
            WindowBuilder::new()
                .with_decorations(true)
                .with_inner_size(PhysicalSize::new(1600, 920)),
        )
        .with_menu(None);
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Pages {
    MainPage,
    Explore,
    Collections,
    DownloadProgress,
    CollectionPage(Arc<str>),
}

impl Pages {
    fn new_collection_page(s: impl Into<Arc<str>>) -> Pages {
        Pages::CollectionPage(s.into())
    }
}

impl ActiveCompare for Pages {
    fn hashed_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn compare(&self) -> bool {
        &ACTIVE_PAGE().0 == self
    }

    fn switch_active(&self) {
        let prev = ACTIVE_PAGE().0;
        if &prev != self {
            ACTIVE_PAGE.write().1 = Some(prev);
        }
        ACTIVE_PAGE.write().0 = self.clone();
    }
}

impl Pages {
    pub fn slide_in_id(&self) -> String {
        format!("flyinout-{}", self.to_string())
    }

    /// Applies slide-in animations to HTML elements based on data attributes.
    ///
    /// This function dynamically applies CSS animations to elements within a webpage
    /// using Tailwind CSS-defined animations. It targets elements with a specific class ('.group')
    /// and adjusts their styles according to their data attributes.
    ///
    /// ## Attributes
    /// The function expects HTML elements to have certain attributes and structure:
    /// * Top level element should have the class `group`.
    /// * Each `group` element should contain at least one child element with an `id` that is acquired by `self.slide_in_id()`
    ///
    /// ## Data Attributes
    /// * `data-prev`: This attribute specifies whether the element was the previous element in a
    ///   sequence. If `true`, the `slideRight` animation is applied.
    /// * `data-selected`: This attribute indicates if the element is the currently selected one.
    ///   If `true`, the `slideLeft` animation is applied.
    ///
    /// ## Usage
    /// To use this function, ensure that your HTML elements are set up correctly with the
    /// required `id` and data attributes. Additionally, for most use cases involving animations or transitions,
    /// it's essential to manage the positioning context correctly:
    ///
    /// - The parent container should have a **relative** positioning to serve as the positioning context for its children.
    /// - Child elements, which are the targets of the animations, should be styled with **absolute** positioning to overlay
    ///   within the relative container seamlessly.
    ///
    /// It is crucial to call this function at the start of each component's lifecycle to properly initialize
    /// the animations.
    ///
    /// Here is an example element setup:
    ///
    /// ```rust
    /// fn Component() {
    ///     Pages::DownloadProgress.apply_slide_in();
    ///     rsx! {
    ///         div {
    ///             "data-selected": selected.to_string(),
    ///             "data-prev": prev.map_or_else(String::new, |x| x.to_string()),
    ///             div { class: "w-full min-h-screen relative",
    ///                 div { class: "absolute inset-0 z-0 min-h-full min-w-full", id: Pages::DownloadProgress.slide_in_id(), LayoutContainer { DownloadProgress {} } }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn apply_slide_in(&self) {
        eval(
            r#"
                function applyStyles(dataValue) {
                    const groups = document.querySelectorAll('.group');            
                    groups.forEach(group => {
                        const prev = group.getAttribute('data-prev') === dataValue;
                        const selected = group.getAttribute('data-selected') === dataValue;
                        const target = group.querySelector('#flyinout-' + dataValue);

                        // Reset styles first
                        target.style.insetInlineStart = '';
                        target.style.zIndex = '0';
                        target.style.display = 'none';
                        target.style.animation = '';

                        if (prev) {
                            target.style.insetInlineStart = '100dvw';
                            target.style.zIndex = '100';
                            target.style.display = 'block';                        
                            target.style.animation = 'slideRight 500ms';
                        } else if (selected) {
                            target.style.zIndex = '50';
                            target.style.display = 'block';                        
                            target.style.animation = 'slideLeft 500ms';
                        }
                    });
                }
                applyStyles(await dioxus.recv());
            "#,
        )
        .send(self.to_string().into())
        .unwrap();
    }
}

impl ToString for Pages {
    fn to_string(&self) -> String {
        match self {
            Self::MainPage => "main-page".into(),
            Self::Explore => "explore".into(),
            Self::Collections => "collections".into(),
            Self::DownloadProgress => "progress".into(),
            Self::CollectionPage(x) => dbg!({
                let mut hasher = DefaultHasher::new();
                x.hash(&mut hasher);
                let hash = hasher.finish();
                format!("collection-page-{}", hash)
            }),
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        div { class: "bg-deep-background min-h-screen min-w-full font-display leading-normal",
            div { class: "[&_*]:transform-gpu", Layout {} }
        }
    }
}

#[component]
fn Layout() -> Element {
    let selected = ACTIVE_PAGE().0;
    let prev = ACTIVE_PAGE().1;
    Pages::new_collection_page("新的收藏").apply_slide_in();
    Pages::DownloadProgress.apply_slide_in();
    rsx! {
        div {
            class: "w-screen inline-flex self-stretch group flex overflow-hidden",
            "data-selected": selected.to_string(),
            "data-prev": prev.map_or_else(String::new, |x| x.to_string()),
            SideBar {}
            div { class: "w-full min-h-screen relative *:overflow-scroll",
                div { class: "absolute inset-0 z-0 min-h-full animation-[main-page^slideDown^explore^slideOutUp] animation-[main-page^slideDown^collections^slideOutUp]",
                    LayoutContainer { MainPage {} }
                }
                div { class: "absolute inset-0 z-0 min-h-full animation-[explore^slideUp^main-page^slideOutDown] animation-[explore^slideDown^collections^slideOutUp]",
                    LayoutContainer { Explore {} }
                }
                div { class: "absolute inset-0 z-0 min-h-full animation-[collections^slideUp^explore^slideOutDown] animation-[collections^slideUp^main-page^slideOutDown]",
                    LayoutContainer { Collections {} }
                }
                div { class: "absolute inset-0 z-0 min-h-full min-w-full", id: Pages::DownloadProgress.slide_in_id(), LayoutContainer { DownloadProgress {} } }
                div { class: "absolute inset-0 z-0 min-h-full min-w-full", id: Pages::new_collection_page("新的收藏").slide_in_id(),
                    LayoutContainer { extended_class: "p-0", CollectionPage {} }
                }
            }
        }
    }
}

/// Does dynmaic rendering
/// do not wrap the children in another div
#[component]
fn LayoutContainer(children: Element, #[props(default)] extended_class: String) -> Element {
    rsx! {
        div { class: tw_merge!("bg-background min-h-screen rounded-xl p-8 min-w-full", extended_class),
            div { class: "flex flex-col space-y-[20px] transition-all xl:items-center xl:*:justify-center xl:*:max-w-[1180px] xl:*:w-full",
                {children}
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CollectionPageTopSelection {
    Mods,
    World,
    ResourcePack,
    ShaderPacks,
}

const A: GlobalSignal<(
    CollectionPageTopSelection,
    Option<CollectionPageTopSelection>,
)> = GlobalSignal::new(|| (CollectionPageTopSelection::Mods, None));

impl ActiveCompare for CollectionPageTopSelection {
    fn hashed_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn compare(&self) -> bool {
        &A().0 == self
    }

    fn switch_active(&self) {
        let prev = A().0;
        if &prev != self {
            A.write().1 = Some(prev);
        }
        A.write().0 = *self;
    }
}

#[component]
fn CollectionPage() -> Element {
    rsx! {
        div { class: "flex flex-col",
            div { class: "sticky top-0 p-[50px] rounded-2xl bg-slate-800 grid grid-flow-col items-stretch",
                div { class: "flex flex-col space-y-[35px]",
                    div { class: "text-white font-black text-[80px] leading-normal capsize",
                        "新的收藏"
                    }
                    Button {
                        roundness: Roundness::Pill,
                        string_placements: vec![ContentType::text("F").css("w-[30px] h-[30px]").align_center()],
                        fill_mode: FillMode::Fit,
                        extended_css_class: "w-fit shadow p-[13px]"
                    }
                }
                div { class: "flex justify-end",
                    div { class: "flex flex-col space-y-[3px] w-full max-w-[250px]",
                        CollectionBlock {
                            extended_class: "rounded-[20px] w-full h-[250px]",
                            picture: COLLECTION_PIC,
                            gradient: false
                        }
                        div { class: "flex space-x-[3px] min-w-full",
                            Button {
                                roundness: Roundness::None,
                                string_placements: vec![ContentType::text("s").align_center()],
                                fill_mode: FillMode::Fill,
                                extended_css_class: "rounded-[5px] rounded-bl-[20px] flex-1 min-w-0 bg-lime-300"
                            }
                            Button {
                                roundness: Roundness::None,
                                string_placements: vec![ContentType::text("...").align_center()],
                                fill_mode: FillMode::Fit,
                                extended_css_class: "rounded-[5px] rounded-br-[20px] bg-white/10 backdrop-blur-[100px] flex-none"
                            }
                        }
                    }
                }
            }
            div { class: "px-[30px] bg-background rounded-2xl min-h-dvh scroll-smooth",
                div { class: "bg-background flex justify-center items-center min-h-full py-[30px]",
                    {ContentType::svg(manganis::mg!(file("public/Line 155.svg"))).get_element()}
                }
                div { class: "grid grid-flow-col items-stretch",
                    div { class: "bg-deep-background rounded-full flex justify-start w-fit",
                        Button {
                            roundness: Roundness::Pill,
                            fill_mode: FillMode::Fit,
                            signal: Rc::new(CollectionPageTopSelection::Mods) as Rc<dyn ActiveCompare>,
                            string_placements: vec![ContentType::text("A").align_left(), ContentType::text("模組").align_right()]
                        }
                        Button {
                            roundness: Roundness::Pill,
                            fill_mode: FillMode::Fit,
                            signal: Rc::new(CollectionPageTopSelection::World) as Rc<dyn ActiveCompare>,
                            string_placements: vec![ContentType::text("B").align_left(), ContentType::text("世界").align_right()]
                        }
                        Button {
                            roundness: Roundness::Pill,
                            fill_mode: FillMode::Fit,
                            signal: Rc::new(CollectionPageTopSelection::ResourcePack) as Rc<dyn ActiveCompare>,
                            string_placements: vec![
                                ContentType::text("C").align_left(),
                                ContentType::text("資源包").align_right(),
                            ]
                        }
                        Button {
                            roundness: Roundness::Pill,
                            fill_mode: FillMode::Fit,
                            signal: Rc::new(CollectionPageTopSelection::ShaderPacks) as Rc<dyn ActiveCompare>,
                            string_placements: vec![
                                ContentType::text("D").align_left(),
                                ContentType::text("光影包").align_right(),
                            ]
                        }
                    }
                    div { class: "flex space-x-[7px] justify-end",
                        Button {
                            roundness: Roundness::Pill,
                            string_placements: vec![
                                ContentType::svg(EXPLORE)
                                    .css("flex justify-center items-center w-[25px] h-[25px] overflow-none")
                                    .align_center(),
                            ],
                            fill_mode: FillMode::Fit,
                            extended_css_class: "px-[25px]"
                        }
                        Button {
                            roundness: Roundness::Pill,
                            string_placements: vec![ContentType::text("F").css("w-[25px] h-[25px]").align_center()],
                            fill_mode: FillMode::Fit,
                            extended_css_class: "px-[25px]"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Explore() -> Element {
    rsx! {
        div {
            Button {
                roundness: Roundness::Top,
                string_placements: vec![
                    ContentType::text("Explore").align_left(),
                    ContentType::text("thumbsup").align_right(),
                ]
            }
        }
    }
}

#[component]
fn DownloadProgress() -> Element {
    rsx! {
        div {
            Button {
                roundness: Roundness::Top,
                string_placements: vec![
                    ContentType::text("Progress").align_left(),
                    ContentType::text("stop").align_right(),
                ]
            }
        }
    }
}

pub static EXPANDED: GlobalSignal<bool> = GlobalSignal::new(|| false);

#[component]
fn SideBar() -> Element {
    let delayed_expanded = use_resource(move || async move {
        if EXPANDED() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        } else {
            // tokio::time::sleep(Duration::from_millis(100)).await;
        }
        EXPANDED()
    });
    let onclick = move |()| {
        Pages::Collections.switch_active();
        if EXPANDED() {
            *EXPANDED.write() = false;
        } else {
            *EXPANDED.write() = true;
        }
    };
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
                    ContentType::svg(ARROW_RIGHT).css("flex items-center w-[25px] h-[25px] *:w-[25px] *:h-[25px] block group-aria-expanded:hidden").get_element()
                }
            }
        }
        div { class: tw_merge!(Alignment::Right.get_alignment_class(), "group-aria-busy:hidden"), {ContentType::text("我的錦集").css("text-lime-300").get_element()} }
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
                        signal: Rc::new(Pages::MainPage) as Rc<dyn ActiveCompare>,
                        extended_css_class: "bg-background group-aria-expanded:pr-5"
                    }
                    Button {
                        roundness: Roundness::None,
                        string_placements: vec![
                            ContentType::svg(EXPLORE).align_left(),
                            ContentType::text("探索").css("group-aria-busy:hidden").align_right(),
                        ],
                        signal: Rc::new(Pages::Explore) as Rc<dyn ActiveCompare>,
                        extended_css_class: "bg-background group-aria-expanded:pr-5"
                    }
                    Button {
                        roundness: Roundness::Bottom,
                        string_placements: vec![
                            ContentType::svg(SIDEBAR_COLLECTION).align_left(),
                            ContentType::text("收藏庫").css("group-aria-busy:hidden").align_right(),
                        ],
                        signal: Rc::new(Pages::Collections) as Rc<dyn ActiveCompare>,
                        onclick,
                        extended_css_class: "bg-background group-aria-expanded:pr-5"
                    }
                }
                // middle
                div { class: "flex flex-col space-y-1",
                    Button { roundness: Roundness::Top, string_placements: folded_images, extended_css_class: "bg-background" }
                    Button {
                        roundness: Roundness::None,
                        string_placements: vec![
                            ContentType::image(COLLECTION_PIC.to_string())
                                .css(
                                    "transition-all w-[50px] h-[50px] object-cover inline-flex items-center rounded-[15px] border-2 border-zinc-900 group-aria-expanded:w-20 group-aria-expanded:h-20",
                                )
                                .align_left(),
                            ContentType::text("新的收藏").align_right().css("group-aria-busy:hidden"),
                        ],
                        signal: Rc::new(Pages::new_collection_page("新的收藏")) as Rc<dyn ActiveCompare>,
                        focus_color_change: false,
                        extended_css_class: "bg-background transition-all delay-[25ms] group-aria-expanded:w-20 group-aria-expanded:min-h-20 group-aria-expanded:p-0"
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
                        signal: Rc::new(Pages::DownloadProgress) as Rc<dyn ActiveCompare>,
                        extended_css_class: "bg-background group/active items-center",
                        onclick: move |()| {
                            let prev = ACTIVE_PAGE().1;
                            if ACTIVE_PAGE().0 == Pages::DownloadProgress {
                                if let Some(prev) = prev {
                                    prev.switch_active();
                                    ACTIVE_PAGE.write().1 = Some(Pages::DownloadProgress);
                                }
                            } else {
                                Pages::DownloadProgress.switch_active();
                            }
                        }
                    }
                }
            }
        }
    }
}
