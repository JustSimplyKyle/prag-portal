#![allow(non_snake_case)]
pub mod BaseComponents;
pub mod collection_display;
pub mod collection_edit;
pub mod collections;
pub mod download_progress;
pub mod main_page;
pub mod pages;
pub mod scrollable;
pub mod side_bar;
pub mod text_scroller;

use builder::collection_builder;
use collection_edit::CollectionEditContainer;
use dioxus::desktop::tao::dpi::PhysicalSize;
use dioxus::desktop::WindowBuilder;
use dioxus::html::input_data::MouseButton;
use dioxus_logger::tracing::{info, Level};
use itertools::Itertools;
use manganis::ImageAsset;
use pages::Pages;
use rand::Rng;
use rust_lib::api::backend_exclusive::download::DownloadError;
use rust_lib::api::backend_exclusive::errors::ManifestProcessingError;
use rust_lib::api::backend_exclusive::storage::storage_loader::StorageError;
use rust_lib::api::backend_exclusive::vanilla::version::VersionMetadata;
use rust_lib::api::shared_resources::collection::{
    CollectionError, CollectionId, ModLoader, ModLoaderType,
};
use scrollable::Scrollable;
use snafu::{AsErrorSource, ErrorCompat};
use std::collections::BTreeMap;
use std::path::PathBuf;
use tailwind_fuse::*;
use BaseComponents::{
    atoms::button::{Button, FillMode, Roundness},
    organisms::modal::{ComponentPointer, Modal},
    string_placements::ContentType,
};

use dioxus::{prelude::*, CapturedError};

use crate::collection_display::CollectionDisplay;
use crate::collections::Collections;
use crate::download_progress::DownloadProgress;
use crate::main_page::MainPage;
use crate::side_bar::SideBar;

pub const COLLECTION_PICS: GlobalSignal<BTreeMap<&str, ImageAsset>> = GlobalSignal::new(|| {
    BTreeMap::from([
        (
            "a",
            manganis::mg!(image("./public/first_collection_pic.png").preload()),
        ),
        (
            "b",
            manganis::mg!(image("./public/second_collection_pic.png").preload()),
        ),
        (
            "c",
            manganis::mg!(image("./public/third_collection_pic.png").preload()),
        ),
        (
            "d",
            manganis::mg!(image("./public/forth_collection_pic.png").preload()),
        ),
        (
            "e",
            manganis::mg!(image("./public/fifth_collection_pic.png").preload()),
        ),
    ])
});

fn get_random_collection_picture() -> ImageAsset {
    let index = rand::thread_rng().gen_range(0..=4);
    COLLECTION_PICS.read().values().nth(index).unwrap().clone()
}

pub const HOME: &str = manganis::mg!("public/home.svg");
pub const EXPLORE: &str = manganis::mg!("public/explore.svg");
pub const SIDEBAR_COLLECTION: &str = manganis::mg!("public/collections.svg");
pub const ARROW_RIGHT: &str = manganis::mg!("public/keyboard_arrow_right.svg");
pub const SIM_CARD: &str = manganis::mg!("public/sim_card_download.svg");
pub const DRAG_INDICATOR: &str = manganis::mg!("public/drag_indicator.svg");
pub const TAILWIND_STR: &str = manganis::mg!("public/tailwind.css");

/// `(Pages)`: Current active page
/// `Option<Pages>`: Previous page
static HISTORY: GlobalSignal<History> = GlobalSignal::new(|| History::new(Pages::MainPage));
pub static TOP_LEVEL_COMPONENT: GlobalSignal<
    Vec<ComponentPointer<crate::BaseComponents::organisms::modal::__sub_modalProps>>,
> = GlobalSignal::new(Vec::new);

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
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new().with_window(
        WindowBuilder::new()
            .with_decorations(true)
            .with_title("Prag Portal")
            .with_inner_size(PhysicalSize::new(1600, 920)),
    );
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

mod builder {
    use std::path::PathBuf;

    use dioxus_logger::tracing::info;
    use rust_lib::api::{
        backend_exclusive::{errors::ManifestProcessingError, vanilla::version::VersionMetadata},
        shared_resources::{
            collection::{CollectionError, ModLoader, ModLoaderType},
            entry,
        },
    };
    use snafu::prelude::*;

    use crate::get_random_collection_picture;

    #[derive(Snafu, Debug)]
    pub enum CollectionBuilderError {
        #[snafu(display("Invalid version id {id}"))]
        InvalidVersionId { id: String },
        #[snafu(display("Failed to parse version id {id}"))]
        VersionIdParsing {
            id: String,
            source: ManifestProcessingError,
        },
        #[snafu(transparent)]
        CollectionError { source: CollectionError },
    }

    pub async fn collection_builder(
        picture_path: impl Into<Option<PathBuf>> + Send,
        version_id: impl Into<String> + Send,
    ) -> Result<(), CollectionBuilderError> {
        let version_id = version_id.into();
        let version = VersionMetadata::from_id(&version_id)
            .await
            .context(VersionIdParsingSnafu { id: &version_id })?
            .context(InvalidVersionIdSnafu { id: &version_id })?;
        let mut collection = entry::create_collection(
            "新的收藏",
            picture_path
                .into()
                .unwrap_or_else(|| get_random_collection_picture().path().into()),
            version,
            ModLoader::new(ModLoaderType::Fabric, None),
            None,
        )
        .await?;
        info!("Adding mods...");
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
        collection.download_mods().await?;
        info!("Finished downloading mods");
        Ok(())
    }
}

#[component]
fn App() -> Element {
    let error_active = use_signal(|| true);
    rsx! {
        head::Link {
            href: TAILWIND_STR,
            rel: "stylesheet",
        }
        div {
            class: "[&_*]:transform-gpu font-['GenSenRounded TW'] bg-deep-background h-screen w-screen font-display leading-normal",
            {
                TOP_LEVEL_COMPONENT().into_iter().map(|x| (x.pointer)(x.props))
            }
            ErrorBoundary {
                handle_error: move |error| { rsx! {
                    Modal {
                        active: error_active,
                        name: "error_modal",
                        close_on_outer_click: false,
                        div {
                            div { class: "min-w-full flex flex-col items-center space-y-3",
                                div { class: "text-red text-3xl font-black",
                                    "Hmm, something went wrong. Please copy the following error to the developer."
                                }
                                Button {
                                    roundness: Roundness::Pill,
                                    extended_css_class: "text-[13px] font-bold",
                                    string_placements: rsx! {
                                        pre { "{error:#?}" }
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

pub trait IntoRenderError {
    fn into_render_error(self) -> RenderError;
}

impl IntoRenderError for anyhow::Error {
    fn into_render_error(self) -> RenderError {
        use std::error::Error;
        let boxed_error: Box<dyn Error + Send + Sync> = Box::from(self);
        let leaked_error: &'static (dyn Error + Send + Sync) = Box::leak(boxed_error);
        RenderError::Aborted(CapturedError::from(leaked_error))
    }
}

pub trait RefIntoRenderError {
    fn into_render_error(&self) -> RenderError;
}

impl<T: std::fmt::Display> RefIntoRenderError for T {
    fn into_render_error(&self) -> RenderError {
        RenderError::Aborted(CapturedError::from_display(self.to_string()))
    }
}

pub trait ThrowResource<T> {
    fn throw(&self) -> Result<Option<T>, RenderError>;
}

impl<T: Clone> ThrowResource<T> for Resource<Result<T, anyhow::Error>> {
    fn throw(&self) -> Result<Option<T>, RenderError> {
        let binding = self.read();
        let transpose = binding.as_ref().map(|x| x.as_ref()).transpose();
        transpose
            .map_err(RefIntoRenderError::into_render_error)
            .map(|x| x.cloned())
    }
}

pub fn use_error_handler() -> Signal<Option<Result<(), anyhow::Error>>> {
    use_context()
}

pub trait ErrorFormatted {
    fn to_formatted(&self) -> String;
}

impl<T: std::error::Error + std::fmt::Debug + ErrorCompat + 'static> ErrorFormatted for T {
    fn to_formatted(&self) -> String {
        let chain = self
            .iter_chain()
            .enumerate()
            .fold(String::new(), |mut acc, (u, x)| {
                acc.push_str(&format!("{u}: {x}"));
                acc.push('\n');
                acc
            });
        format!("display: \n{chain}debug:\n{self:#?}")
    }
}
#[component]
fn Layout() -> Element {
    {
        let err = use_resource(|| collection_builder(None, "1.20.1"));
        let read = err.read();
        let transpose = read.as_ref().map(|x| x.as_ref().cloned()).transpose();
        if let Err(err) = transpose {
            return Err(err.to_formatted().into_render_error());
        }
    }

    let keys = use_context_provider(move || {
        Signal::memo(move || (STORAGE.collections)().into_keys().collect::<Vec<_>>())
    });

    let mut error_handler = use_context_provider(|| Signal::new(None));

    use_effect(move || {
        let binding = || {
            let history = HISTORY.read();
            Pages::DownloadProgress.apply_slide_in()?;
            let pages_scroller = vec![Pages::MainPage, Pages::Explore, Pages::Collections];
            Pages::scroller_applyer(pages_scroller, |x| x == &history.active)?;
            for collection_id in keys() {
                Pages::collection_display(collection_id.clone()).apply_slide_in()?;
                Pages::collection_edit(collection_id).apply_slide_in()?;
            }
            Ok::<_, anyhow::Error>(())
        };
        error_handler.set(Some(binding()));
    });

    use_memo(move || {
        if let Some(x) = error_handler.read().as_ref() {
            return x
                .as_ref()
                .cloned()
                .map_err(RefIntoRenderError::into_render_error);
        }
        Ok(())
    })()?;

    let history = HISTORY.read();
    rsx! {
        div {
            class: "max-w-screen max-h-screen overflow-clip group-pages flex",
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
            SideBar {

            }
            div {
                class: "bg-background w-screen h-screen relative *:overflow-scroll",
                div {
                    class: "absolute inset-0 z-0 min-h-full",
                    id: Pages::MainPage.scroller_id(),
                    if Pages::MainPage.should_render() {
                        LayoutContainer {
                            MainPage {

                            }
                        }
                    }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full",
                    id: Pages::Explore.scroller_id(),
                    if Pages::Explore.should_render() {
                        LayoutContainer {
                            Explore {

                            }
                        }
                    }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full",
                    id: Pages::Collections.scroller_id(),
                    if Pages::Collections.should_render() {
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
                CollectionContainer {

                }
            }
            CollectionEditContainer {

            }
        }
    }
}

#[component]
fn CollectionContainer() -> Element {
    let keys = use_context::<Memo<Vec<CollectionId>>>();
    rsx! {
        for collection_id in keys() {
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
