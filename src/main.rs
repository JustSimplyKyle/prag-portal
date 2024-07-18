#![allow(non_snake_case)]
pub mod BaseComponents;
pub mod collection_display;
pub mod collections;
pub mod download_progress;
pub mod main_page;
pub mod side_bar;

use collection_display::{DISPLAY_BACKGROUND, GAME_CONTROLLER, UNDO};
use dioxus::desktop::tao::dpi::PhysicalSize;
use dioxus::desktop::WindowBuilder;
use dioxus::html::input_data::MouseButton;
use main_page::ARROW_LEFT;
use manganis::ImageAsset;
use rust_lib::api::backend_exclusive::vanilla::version::VersionMetadata;
use rust_lib::api::shared_resources::collection::{
    Collection, CollectionId, ModLoader, ModLoaderType,
};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;
use strum::{EnumIter, IntoEnumIterator};
use tailwind_fuse::*;
use BaseComponents::molecules::switcher::Comparison;
use BaseComponents::string_placements::{Alignment, Content, Contents};
use BaseComponents::{
    atoms::button::{Button, FillMode, Roundness},
    molecules::switcher::StateSwitcher,
    organisms::modal::{subModalProps, ComponentPointer, Modal},
    string_placements::ContentType,
};

use dioxus::prelude::*;
use log::LevelFilter;

use crate::collection_display::CollectionDisplay;
use crate::collections::Collections;
use crate::download_progress::DownloadProgress;
use crate::main_page::MainPage;
use crate::side_bar::SideBar;

pub const COLLECTION_PIC: ImageAsset = manganis::mg!(image("./public/pic1.png").preload());
pub const HOME: &str = manganis::mg!(file("./public/home.svg"));
pub const EXPLORE: &str = manganis::mg!(file("./public/explore.svg"));
pub const SIDEBAR_COLLECTION: &str = manganis::mg!(file("./public/collections.svg"));
pub const ARROW_RIGHT: &str = manganis::mg!(file("./public/keyboard_arrow_right.svg"));
pub const SIM_CARD: &str = manganis::mg!(file("./public/sim_card_download.svg"));
pub const DRAG_INDICATOR: &str = manganis::mg!(file("./public/drag_indicator.svg"));
pub const TAILWIND_STR_: &str = manganis::mg!(file("./public/tailwind.css"));

/// `(Pages)`: Current active page
/// `Option<Pages>`: Previous page
static HISTORY: GlobalSignal<History> = GlobalSignal::new(|| History::new(Pages::MainPage));
pub static TOP_LEVEL_COMPONENT: GlobalSignal<Vec<ComponentPointer<subModalProps>>> =
    GlobalSignal::new(Vec::new);

/// `History` is used to keep track of the navigation history in the application.
/// It contains the following fields:
/// * `active`: The current active page.
/// * `history`: A vector of pages that have been visited.
/// * `prev_steps`: The number of steps taken back in the history.
/// Represents a browsing history.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct History {
    /// The currently active page.
    active: Pages,
    /// The history of visited pages.
    history: Vec<Pages>,
    /// The number of previous steps taken.
    prev_steps: usize,
}

impl History {
    /// Creates a new `History` instance with the given page as the active page.
    ///
    /// # Arguments
    ///
    /// * `page` - The initial page to start the history with.
    ///
    /// # Returns
    ///
    /// A new `History` instance.
    pub fn new(page: Pages) -> Self {
        Self {
            active: page.clone(),
            history: vec![page],
            prev_steps: 0,
        }
    }
    pub const fn active(&self) -> &Pages {
        &self.active
    }
    pub const fn history(&self) -> &Vec<Pages> {
        &self.history
    }
    pub fn prev_peek(&self) -> Option<&Pages> {
        if self.prev_steps == 0 {
            self.history.iter().rev().nth(1)
        } else {
            self.history.iter().rev().nth(self.prev_steps - 1)
        }
    }

    pub fn go_prev(&mut self) {
        self.prev_steps += 1;
        if let Some(x) = self.history.iter().rev().nth(self.prev_steps) {
            self.focus_without_history(x.clone());
        } else {
            self.prev_steps = self.prev_steps.saturating_sub(1);
        }
    }

    pub fn go_next(&mut self) {
        if let Some(steps) = self.prev_steps.checked_sub(1) {
            self.prev_steps = steps;
            if let Some(x) = self.history.iter().rev().nth(steps) {
                self.focus_without_history(x.clone());
            } else {
                self.prev_steps += 1;
            }
        }
    }
    pub fn focus_with_history(&mut self, page: Pages) {
        if self.active != page {
            self.active = page.clone();
            if self.prev_steps != 0 {
                let len = self.history.len() - 1 - self.prev_steps;
                self.history = self.history[..=len].to_vec();
                self.prev_steps = 0;
            }
            self.history.push(page);
        }
    }
    pub fn focus_without_history(&mut self, page: Pages) {
        self.active = page;
    }
}

use rust_lib::api::shared_resources::entry::{self, STORAGE};

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new().with_window(
        WindowBuilder::new()
            .with_decorations(true)
            .with_inner_size(PhysicalSize::new(1600, 920)),
    );
    // .with_menu(DioxusMenu);
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Pages {
    MainPage,
    Explore,
    Collections,
    DownloadProgress,
    CollectionPage {
        id: CollectionId,
        state: CollectionPageState,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, derive_more::Display)]
pub enum CollectionPageState {
    Display,
    Edit,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, EnumIter)]
pub enum EditState {
    Personalization,
    DataLog,
    Export,
    Advanced,
}

impl std::fmt::Display for EditState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "edit-{}",
            match self {
                EditState::Personalization => "personalization",
                EditState::DataLog => "datalog",
                EditState::Export => "export",
                EditState::Advanced => "advanced",
            }
        )
    }
}

impl Scrollable for EditState {
    const GROUP_SELECTOR: &'static str = "group-edit";
}

impl_context_switcher!(EditState);

impl_optional_state_switcher!(Pages);

impl Pages {
    fn collection_display(id: CollectionId) -> Self {
        Self::CollectionPage {
            id,
            state: CollectionPageState::Display,
        }
    }
    fn collection_edit(id: CollectionId) -> Self {
        Self::CollectionPage {
            id,
            state: CollectionPageState::Edit,
        }
    }
}

impl StateSwitcher for Pages {
    fn hashed_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn compare(&self) -> bool {
        HISTORY.read().active() == self
    }

    fn switch_active_to_self(&self) {
        HISTORY.write().focus_with_history(self.clone());
    }
}

pub trait Scrollable: Sized + ToString {
    const GROUP_SELECTOR: &'static str;
    fn scroller_id(&self) -> String {
        format!("scrolling-{}", self.to_string())
    }
    fn apply_scroller_animation(&self, bottom: &[Self], top: &[Self]) -> Result<(), anyhow::Error> {
        let target = self.to_string();
        let bottom = bottom
            .iter()
            .map(|arg0| arg0.to_string())
            .collect::<Vec<_>>();
        let top = top.iter().map(|arg0| arg0.to_string()).collect::<Vec<_>>();
        let eval = eval(
            r#"
                function applyStyles(self, bottom, top, group) {
                    const groups = document.querySelectorAll('.' + group);            
                    groups.forEach(group => {
                        const prev = group.getAttribute('data-prev');
                        const target = group.querySelector('#scrolling-' + self);
                        const bottomElems = bottom.map((x) => group.querySelector('#scrolling-' + x));
                        const topElems = top.map((x) => group.querySelector('#scrolling-' + x));

                        // Reset styles first
                        bottomElems.forEach((ele) => {
                            ele.style.display = 'none';
                            ele.style.zIndex = '0';
                            ele.style.animation = '';
                        });
                        topElems.forEach((ele) => {
                            ele.style.display = 'none';
                            ele.style.zIndex = '0';
                            ele.style.animation = '';
                        });

                        target.style.display = 'block';
                        target.style.zIndex = '50';
                        const finded_bottom = bottom.find((ele) => prev === ele);
                        const finded_top = top.find((ele) => prev === ele);
                        if (finded_bottom) {
                            const bottomElem = group.querySelector('#scrolling-' + finded_bottom);
                            target.style.animation = 'slideDown 1000ms';
                            bottomElem.style.display = 'block';
                            bottomElem.style.zIndex = '10';
                            bottomElem.style.animation = 'slideOutDown 1000ms';
                        }
                        else if (finded_top) {
                            const topElem = group.querySelector('#scrolling-' + finded_top);
                            target.style.animation = 'slideUp 1000ms';
                            topElem.style.display = 'block';
                            topElem.style.zIndex = '10';
                            topElem.style.animation = 'slideOutUp 1000ms';
                        }

                    });
                }
                const [[self], [group], bottom, top] = await dioxus.recv();
                applyStyles(self, bottom, top, group);
            "#,
        );
        eval.send(
            vec![
                vec![target],
                vec![Self::GROUP_SELECTOR.to_owned()],
                bottom,
                top,
            ]
            .into(),
        )
        .map_err(|x| anyhow::anyhow!("{x:#?}"))
    }
}

impl Scrollable for Pages {
    const GROUP_SELECTOR: &'static str = "group-pages";
}

impl Pages {
    pub fn slide_in_id(&self) -> String {
        format!("flyinout-{}", self.to_string())
    }

    pub fn should_render(&self) -> bool {
        HISTORY.read().active() == self || HISTORY.read().prev_peek() == Some(self)
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
    pub fn apply_slide_in(&self) -> anyhow::Result<()> {
        eval(
            r#"
                function applyStyles(dataValue) {
                    const groups = document.querySelectorAll('.group-pages');            
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
        .map_err(|x| anyhow::anyhow!("{x:?}"))
    }
}

impl ToString for Pages {
    fn to_string(&self) -> String {
        match self {
            Self::MainPage => "main-page".into(),
            Self::Explore => "explore".into(),
            Self::Collections => "collections".into(),
            Self::DownloadProgress => "download-progress".into(),
            Self::CollectionPage { id, state } => {
                let mut hasher = DefaultHasher::new();
                id.hash(&mut hasher);
                let hash = hasher.finish();
                format!("collection-page-{}-{hash}", state.to_string())
            }
        }
    }
}

pub async fn collection_builder(
    picture_path: impl Into<Option<PathBuf>> + Send,
    version_id: impl Into<String> + Send,
) -> anyhow::Result<()> {
    let version = VersionMetadata::from_id(&version_id.into()).await?;
    let mut collection = entry::create_collection(
        "weird test",
        picture_path
            .into()
            .unwrap_or_else(|| COLLECTION_PIC.path().into()),
        version,
        ModLoader::new(ModLoaderType::Fabric, None),
        None,
    )
    .await?;
    collection
        .add_multiple_modrinth_mod(
            vec![
                "fabric-api",
                "sodium",
                "modmenu",
                "ferrite-core",
                "lazydfu",
                "iris",
                "indium",
            ],
            vec![],
            None,
        )
        .await?;
    collection.download_mods().await
}

#[component]
fn App() -> Element {
    let error_active = use_signal(|| true);
    use_future(move || async move {
        collection_builder(None, "1.20.1").await.unwrap();
    });
    rsx! {
        div {
            class: "[&_*]:transform-gpu font-['GenSenRounded TW'] bg-deep-background min-h-screen min-w-full font-display leading-normal",
            {
                TOP_LEVEL_COMPONENT().into_iter().map(|x| (x.pointer)(x.props))
            },
            ErrorBoundary {
                handle_error: move |error| { rsx! {
                    Modal { active: error_active, name: "error_modal", close_on_outer_click: false,
                        div {
                            div { class: "w-full flex flex-col items-center space-y-3",
                                div { class: "text-red text-2xl font-bold",
                                    "Hmm, something went wrong. Please copy the following error to the developer."
                                }
                                Button {
                                    roundness: Roundness::Pill,
                                    extended_css_class: "text-[13px] font-bold",
                                    string_placements: rsx! {
                                        pre {
                                            "{error:#?}"
                                        }
                                    },
                                    fill_mode: FillMode::Fit,
                                    clickable: false
                                }
                            }
                        }
                    }
                } },
                Layout {
                }
            }
        }
    }
}

fn scroller_applyer<T: Scrollable + std::fmt::Debug>(
    pages_scroller: Vec<T>,
    filterer: impl Fn(&T) -> bool,
) -> anyhow::Result<()> {
    let iter = pages_scroller
        .iter()
        .enumerate()
        .filter(|(_, x)| filterer(&*x));
    for (u, x) in iter {
        let (left, right) = pages_scroller.split_at(u);
        x.apply_scroller_animation(&right[1..], left)?;
    }
    Ok(())
}

#[component]
fn Layout() -> Element {
    use_effect(move || {
        let history = HISTORY.read();
        Pages::DownloadProgress.apply_slide_in().unwrap();
        let pages_scroller = vec![Pages::MainPage, Pages::Explore, Pages::Collections];
        scroller_applyer(pages_scroller, |x| x == &history.active).unwrap();
        for collection in &*STORAGE.collections.read() {
            Pages::collection_display(collection.get_collection_id())
                .apply_slide_in()
                .unwrap();
            Pages::collection_edit(collection.get_collection_id())
                .apply_slide_in()
                .unwrap();
        }
    });

    let history = HISTORY.read();
    rsx! {
        div {
            class: "w-screen group-pages flex",
            "data-selected": history.active.to_string(),
            "data-prev": history.prev_peek().map_or_else(String::new, ToString::to_string),
            onmousedown: move |x| {
                if let Some(x) = x.data().trigger_button() {
                    if x == MouseButton::Fourth {
                        HISTORY.write().go_prev();
                    }
                    if x == MouseButton::Fifth {
                        HISTORY.write().go_next();
                    }
                }
            },
            SideBar {}
            div {
                class: "w-full min-h-screen relative *:overflow-scroll",
                if Pages::MainPage.should_render() {
                    div {
                        class: "absolute inset-0 z-0 min-h-full",
                        id: Pages::MainPage.scroller_id(),
                        LayoutContainer {
                            MainPage {
                            }
                        }
                    }
                }
                if Pages::Explore.should_render() {
                    div {
                        class: "absolute inset-0 z-0 min-h-full",
                        id: Pages::Explore.scroller_id(),
                        LayoutContainer {
                            Explore {
                            }
                        }
                    }
                }
                if Pages::Collections.should_render() {
                    div {
                        class: "absolute inset-0 z-0 min-h-full",
                        id: Pages::Collections.scroller_id(),
                        LayoutContainer {
                            Collections {
                            }
                        }
                    }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: Pages::DownloadProgress.slide_in_id(),
                    LayoutContainer {
                        DownloadProgress {
                        }
                    }
                }
                CollectionContainer {  }
            }
            CollectionEditContainer { }
        }
    }
}

#[component]
fn CollectionEditContainer() -> Element {
    let binding = STORAGE.collections.read();
    let collection_ids = binding.iter().map(Collection::get_collection_id);
    rsx! {
        for collection_id in collection_ids {
            div {
                class: "absolute inset-0 z-0 min-w-full min-h-full",
                id: Pages::collection_edit(collection_id).slide_in_id(),
                    if Pages::collection_edit(collection_id.clone()).should_render() {
                    CollectionEdit {
                        collection_id: collection_id.clone()
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarDisplay(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let read = collection_id.read();
    let collection = read.get_collection();
    rsx! {
        div {
            class: "flex flex-col w-full",
            div {
                class: "flex flex-col p-5 justify-end rounded-t-[50px] w-full h-[250px]",
                background: format!("radial-gradient(171.48% 102.52% at 0% 100%, #000 0%, rgba(0, 0, 0, 0.00) 100%), url(\"{}\") lightgray 50% / cover no-repeat", DISPLAY_BACKGROUND),
                {
                    ContentType::image(collection.picture_path.to_string_lossy().to_string()).css("w-[100px] h-[100px] bg-cover rounded-t-[50px] rounded-bl-[15px] rounded-br-[50px] p-[5px]")
                }
            }
            Button {
                roundness: Roundness::Bottom,
                extended_css_class: "bg-background justify-start px-5 pt-[22px]",
                string_placements: vec![
                    Contents::new(
                        vec![
                            ContentType::text(&collection.display_name).css("text-3xl font-balck"),
                            ContentType::hint("由我建立•18 分鐘•不久前開啟").css("font-medium text-[15px]")
                        ],
                        Alignment::Left
                    ).css("flex flex-col gap-[15px]")
                ]
            }
        }
    }
}

#[component]
fn EditSidebar(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        div {
            class: "flex flex-col min-w-[400px] gap-[20px]",
            SidebarDisplay {
                collection_id
            }
            div {
                class: "flex flex-col",
                Button {
                    roundness: Roundness::Top,
                    fill_mode: FillMode::Fit,
                    extended_css_class: "bg-background",
                    focus_color_change: true,
                    switcher: EditState::Personalization,
                    string_placements: vec![
                        ContentType::text("風格化").align_left(),
                        ContentType::svg(ARROW_RIGHT).css("svg-[30px]").align_right(),
                    ]
                }
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "bg-background",
                    fill_mode: FillMode::Fit,
                    focus_color_change: true,
                    switcher: EditState::DataLog,
                    string_placements: vec![
                        ContentType::text("收藏紀錄").align_left(),
                        ContentType::svg(ARROW_RIGHT).css("svg-[30px]").align_right(),
                    ]
                }
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "bg-background",
                    fill_mode: FillMode::Fit,
                    focus_color_change: true,
                    switcher: EditState::Export,
                    string_placements: vec![
                        ContentType::text("分享&匯出").align_left(),
                        ContentType::svg(ARROW_RIGHT).css("svg-[30px]").align_right(),
                    ]
                }
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "bg-background",
                    fill_mode: FillMode::Fit,
                    focus_color_change: true,
                    switcher: EditState::Advanced,
                    string_placements: vec![
                        ContentType::text("進階選項").align_left(),
                        ContentType::svg(ARROW_RIGHT).css("svg-[30px]").align_right(),
                    ]
                }
            }
            div {
                class: "flex justify-stretch w-full gap-[10px]",
                Button {
                    roundness: Roundness::Pill,
                    onclick: move |_| {
                        Pages::collection_display(collection_id()).switch_active_to_self();
                    },
                    extended_css_class: "flex w-auto min-w-auto justify-center items-center bg-background gap-[15px] pl-[20px] pr-[30px]",
                    string_placements: vec![
                        ContentType::svg(UNDO).css("svg-[35px]").align_center(),
                        ContentType::text("返回頁面").align_center()
                    ]
                }
                Button {
                    roundness: Roundness::Pill,
                    extended_css_class: "flex w-auto min-w-auto items-center bg-background gap-[15px] pl-[20px] pr-[30px]",
                    string_placements: vec![
                        ContentType::svg(ARROW_LEFT).align_center(),
                        ContentType::text("返回頁面").align_center()
                    ]
                }
            }

        }
    }
}

#[component]
fn EditTemplate(children: Element, title: Element) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full min-h-screen relative bg-background px-[30px] pb-[30px] rounded-[30px]",
            div {
                class: "bg-background sticky top-0",
                div {
                    class: "flex flex-col z-10 bg-background container pt-[30px] rounded-b-[30px]",
                    {title}
                    div {
                        class: "bg-background py-[10px] rounded-t-[30px]",
                    }
                }
            }
            div {
                class: "flex flex-col overflow-scroll z-0 gap-[20px]",
                {children}
            }
        }
    }
}

#[component]
fn Personalization(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        EditTemplate {
            title: rsx! {
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "rounded-[20px] p-[40px] gap-[20px]",
                    string_placements: vec![
                        Contents::new(
                            vec![
                                ContentType::text("風格化").css("font-black text-white text-[40px]"),
                                ContentType::hint("自訂你的收藏樣式")
                            ],
                            Alignment::Left
                        ).css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right()
                    ]
                }
            },
            for _ in 0..10 {
                Button {
                    roundness: Roundness::Pill,
                    extended_css_class: "rounded-[20px] p-[40px]",
                    string_placements: vec![
                        Contents::new(
                            vec![
                                ContentType::text("not").css("font-black text-white text-[40px]"),
                                ContentType::hint("自訂你的收藏樣式")
                            ],
                            Alignment::Left
                        ).css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right()
                    ]
                }
            }
        }
    }
}

#[component]
fn DataLog(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        EditTemplate {
            title: rsx! {
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "rounded-[20px] p-[40px]",
                    string_placements: vec![
                        Contents::new(
                            vec![
                                ContentType::text("收藏紀錄").css("font-black text-white text-[40px]"),
                                ContentType::hint("查看這個收藏的資訊")
                            ],
                            Alignment::Left
                        ).css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right()
                    ]
                }
            },
        }
    }
}

#[component]
fn Export(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        EditTemplate {
            title: rsx! {
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "rounded-[20px] p-[40px]",
                    string_placements: vec![
                        Contents::new(
                            vec![
                                ContentType::text("分享").css("font-black text-white text-[40px]"),
                                ContentType::hint("分享你的收藏或是將它匯出至電腦")
                            ],
                            Alignment::Left
                        ).css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right()
                    ]
                }
            },
        }
    }
}

#[component]
fn Advanced(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        EditTemplate {
            title: rsx! {
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "rounded-[20px] p-[40px]",
                    string_placements: vec![
                        Contents::new(
                            vec![
                                ContentType::text("進階選項").css("font-black text-white text-[40px]"),
                                ContentType::hint("單獨修改此收藏的進階選項")
                            ],
                            Alignment::Left
                        ).css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right()
                    ]
                }
            }
        }
    }
}

#[component]
fn CollectionEdit(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let edit_state: Signal<Comparison<EditState>> =
        use_context_provider(|| Signal::new((EditState::Personalization, None)));
    use_effect(move || {
        let vec = EditState::iter().collect::<Vec<_>>();
        scroller_applyer(vec, |x| &edit_state.read().0 == x).unwrap();
    });
    rsx! {
        div {
            class: "flex w-full bg-deep-background group-edit min-h-screen gap-[20px] rounded-[5px] px-[20px] pb-[20px]",
            "data-prev": edit_state().1.map_or_else(String::new, |x| x.to_string()),
            EditSidebar { collection_id }
            div {
                class: "w-full min-h-screen relative *:overflow-scroll",
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::Personalization.scroller_id(),
                    Personalization { collection_id }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::DataLog.scroller_id(),
                    DataLog { collection_id  }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::Export.scroller_id(),
                    Export { collection_id  }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::Advanced.scroller_id(),
                    Advanced { collection_id  }
                }
            }

        }
    }
}

#[component]
fn CollectionContainer() -> Element {
    rsx! {
        for collection_id in STORAGE
            .collections
            .read()
            .iter()
            .map(Collection::get_collection_id)
        {
            div {
                class: "absolute inset-0 z-0 min-h-full min-w-full",
                id: Pages::collection_display(collection_id.clone()).slide_in_id(),
                if Pages::collection_display(collection_id.clone()).should_render() {
                    LayoutContainer {
                        extended_class: "p-0",
                        CollectionDisplay {
                            collection_id: collection_id.clone()
                        }
                    }
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
        div {
            class: tw_merge!("bg-background min-h-screen rounded-xl p-8 min-w-full", extended_class),
            div {
                class: "flex flex-col space-y-[20px] transition-all xl:items-center xl:*:justify-center xl:*:max-w-[1180px] xl:*:w-full",
                {children}
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
