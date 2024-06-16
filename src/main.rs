#![allow(non_snake_case)]
pub mod BaseComponents;
pub mod collection_display;
pub mod collections;
pub mod main_page;
pub mod side_bar;

use dioxus::desktop::tao::dpi::PhysicalSize;
use dioxus::desktop::WindowBuilder;
use dioxus::html::input_data::MouseButton;
use manganis::ImageAsset;
use rust_lib::api::backend_exclusive::vanilla::version::VersionMetadata;
use rust_lib::api::shared_resources::collection::{CollectionId, ModLoader, ModLoaderType};
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;
use std::sync::Arc;
use tailwind_fuse::*;

use dioxus::prelude::*;
use log::LevelFilter;
use BaseComponents::{subModalProps, ComponentPointer, Switcher};

use crate::collection_display::CollectionDisplay;
use crate::collections::Collections;
use crate::main_page::MainPage;
use crate::side_bar::SideBar;
use crate::BaseComponents::{Alignment, Button, ContentType, Contents, FillMode, Modal, Roundness};

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
    pub fn active(&self) -> &Pages {
        &self.active
    }
    pub fn history(&self) -> &Vec<Pages> {
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

use rust_lib::api::shared_resources::entry::{self, DOWNLOAD_PROGRESS, STORAGE};

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
    CollectionPage(Arc<str>),
}

impl Pages {
    fn new_collection_page(s: CollectionId) -> Self {
        Self::CollectionPage(s.0.into())
    }
}

impl Switcher for Pages {
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
            Self::CollectionPage(x) => {
                let mut hasher = DefaultHasher::new();
                x.hash(&mut hasher);
                let hash = hasher.finish();
                format!("collection-page-{hash}")
            }
        }
    }
}

trait ErrorToString<T> {
    fn error_to_string(&self) -> Result<&T, String>;
}

impl<T> ErrorToString<T> for Result<T, anyhow::Error> {
    fn error_to_string(&self) -> Result<&T, String> {
        self.as_ref().map_err(|x| format!("{x:#?}"))
    }
}
pub async fn collection_builder(
    picture_path: impl Into<Option<PathBuf>>,
    version_id: impl Into<String>,
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
        div { class: "[&_*]:transform-gpu font-['GenSenRounded TW'] bg-deep-background min-h-screen min-w-full font-display leading-normal",
            {
                TOP_LEVEL_COMPONENT().into_iter().map(|x| (x.pointer)(x.props))
            },
            ErrorBoundary {
                handle_error: move |error| { rsx! {
                    Modal { active: error_active, name: "error_modal", close_on_outer_click: false,
                        div {
                            div { class: "flex flex-col space-y-3",
                                div { class: "text-red text-2xl font-bold",
                                    "Hmm, something went wrong. Please copy the following error to the developer."
                                }
                                Button {
                                    roundness: Roundness::Pill,
                                    extended_css_class: "text-[13px] font-bold",
                                    string_placements: rsx! { "{error} " },
                                    fill_mode: FillMode::Fit,
                                    clickable: false
                                }
                            }
                        }
                    }
                } },
                Layout {}
            }
        }
    }
}

#[component]
fn Layout() -> Element {
    use_effect(move || {
        let _ = HISTORY.read();
        Pages::DownloadProgress.apply_slide_in().unwrap();
        for collection in &STORAGE.read().collections {
            Pages::new_collection_page(collection.get_collection_id())
                .apply_slide_in()
                .unwrap();
        }
    });

    let history = HISTORY.read();

    rsx! {
        div {
            class: "w-screen inline-flex self-stretch group flex overflow-hidden",
            "data-selected": history.active().to_string(),
            "data-prev": history.prev_peek().map_or_else(String::new, |x| x.to_string()),
            onmousedown: move |x| {
                if let Some(x) = x.data().trigger_button() {
                    if x == MouseButton::Fourth {
                        HISTORY.write().go_prev()
                    }
                    if x == MouseButton::Fifth {
                        HISTORY.write().go_next()
                    }
                }
            },
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
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: Pages::DownloadProgress.slide_in_id(),
                    LayoutContainer { DownloadProgress {} }
                }
                CollectionContainer {}
            }
        }
    }
}

#[component]
fn CollectionContainer() -> Element {
    rsx! {
        for (name , collection) in STORAGE().collections.into_iter().map(|x| (x.get_collection_id(), x)) {
            if Pages::new_collection_page(collection.get_collection_id()).should_render() {
                div {
                    key: "{name.0}",
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: Pages::new_collection_page(name).slide_in_id(),
                    LayoutContainer { extended_class: "p-0",
                        CollectionDisplay { collection }
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
        div { class: tw_merge!("bg-background min-h-screen rounded-xl p-8 min-w-full", extended_class),
            div { class: "flex flex-col space-y-[20px] transition-all xl:items-center xl:*:justify-center xl:*:max-w-[1180px] xl:*:w-full",
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

#[component]
fn DownloadProgress() -> Element {
    let download_progress = DOWNLOAD_PROGRESS.read();
    let stoarge = STORAGE.read();
    let collections = &stoarge.collections;
    let mut progress = download_progress
        .iter()
        .filter(|(_, x)| x.percentages < 100.)
        .filter_map(|(id, progress)| {
            collections
                .iter()
                .find(|c| c.get_collection_id() == id.collection_id)
                .map(|c| (c, progress))
        })
        .peekable();
    rsx! {
        div {
            if let Some((collection, progress)) = progress.peek() {
                div {
                    class: "w-full h-[350px] p-[30px] rounded-[20px]",
                    background: format!(
                        "linear-gradient(88deg, #0E0E0E 14.88%, rgba(14, 14, 14, 0.70) 100%), url('{}') lightgray 50% / cover no-repeat",
                        collection.picture_path.to_string_lossy().to_string(),
                    ),
                    div { class: "w-full grid grid-flow-col justify-stretch",
                        div { class: "justify-self-start flex flex-col gap-[20px]",
                            {ContentType::text(&collection.display_name).css("justify-self-start text-[60px] font-black text-white")},
                            {
                                ContentType::hint(format!("總計 {}/已下載 {}", progress.total_size.unwrap_or_default().display_size_from_megabytes(), progress.current_size.unwrap_or_default().display_size_from_megabytes()))
                                    .css("font-medium")
                            }
                        }
                        div { class: "justify-self-end flex",
                            {ContentType::text(format!("{}",progress.speed.unwrap_or_default().display_size_from_megabytes())).css("text-[50px] font-bold text-white")},
                            {ContentType::hint("/s").css("text-[50px] font-bold")}
                        }
                    }
                }
            }
            for (collection , progress) in progress {
                Button {key: "{collection.get_collection_id().0}",
                    roundness: Roundness::Pill,
                    string_placements: rsx! {
                        div { class: "w-full grid grid-flow-col justify-stretch items-stretch",
                            div { class: "container justify-self-start",
                                div { class: "w-full flex gap-[15px]",
                                    {ContentType::svg(DRAG_INDICATOR).css("self-center svg-[30px]")},
                                    div { class: "w-full flex gap-[20px]",
                                        {ContentType::image(collection.picture_path.to_string_lossy().to_string()).css("bg-cover bg-white w-[80px] h-[80px] rounded-[10px]")},
                                        div { class: "w-full flex flex-col justify-start gap-[10px]",
                                            {ContentType::text(&collection.display_name).css("text-[25px] font-bold")},
                                            div { class: "flex gap-[4px]",
                                                {ContentType::hint(format!("{} / {} |", progress.current_size.unwrap_or_default().display_size_from_megabytes(), progress.total_size.unwrap_or_default().display_size_from_megabytes())).css("text-base font-semibold")},
                                                {ContentType::text(format!("{}", progress.speed.unwrap_or_default().display_size_from_megabytes())).css("text-base font-semibold")}
                                            }
                                            div { class: "w-full h-full flex items-end",
                                                div { class: "rounded-[50px] w-full h-[7px] bg-zinc-800",
                                                    div {
                                                        class: "transition-all rounded-[50px] bg-white h-[7px]",
                                                        width: format!("{}%", progress.percentages)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    extended_css_class: "rounded-[5px]",
                    fill_mode: FillMode::Fill
                }
            }
        }
    }
}

trait SizeFromMegaBytes {
    fn display_size_from_megabytes(&self) -> String;
}

impl SizeFromMegaBytes for f64 {
    fn display_size_from_megabytes(&self) -> String {
        match *self {
            f if f < 1_000. => format!("{:.2} bytes", f),
            f if f < 1_000_000. => format!("{:.2} KB", f / 1_000.),
            f if f < 1_000_000_000. => format!("{:.2} MB", f / 1_000_000.),
            f => format!("{:.2} GB", f / 1_000_000_000.),
        }
    }
}
