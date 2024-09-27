#![allow(non_snake_case)]
pub mod BaseComponents;
pub mod builder;
pub mod collection_display;
pub mod collection_edit;
pub mod collections;
pub mod download_progress;
pub mod main_page;
pub mod pages;
pub mod scrollable;
pub mod side_bar;
pub mod svgs;
pub mod text_scroller;

use builder::collection_builder;
use collection_edit::CollectionEditContainer;
use dioxus::desktop::tao::dpi::PhysicalSize;
use dioxus::desktop::WindowBuilder;
use dioxus::html::input_data::MouseButton;
use dioxus_logger::tracing::Level;
use manganis::ImageAsset;
use pages::Pages;
use rand::seq::IteratorRandom;
use scrollable::Scrollable;
use snafu::ErrorCompat;
use std::collections::BTreeMap;
use svgs::{CREATE_COLLECTION, CURSEFORGE_OUTLINE, GRASS, MODRINTH_OUTLINE};
use tailwind_fuse::*;
use BaseComponents::{
    atoms::switch::{FloatingSwitch, State},
    organisms::modal::Modal,
};

use dioxus::{prelude::*, CapturedError};

use crate::collection_display::CollectionDisplay;
use crate::collections::Collections;
use crate::download_progress::DownloadProgress;
use crate::main_page::MainPage;
use crate::side_bar::SideBar;

const FIRST: ImageAsset = asset!(image("./public/first_collection_pic.png"));
const SECOND: ImageAsset = asset!(image("./public/second_collection_pic.png"));
const THIRD: ImageAsset = asset!(image("./public/third_collection_pic.png"));
const FORTH: ImageAsset = asset!(image("./public/forth_collection_pic.png"));
const FIFTH: ImageAsset = asset!(image("./public/fifth_collection_pic.png"));

pub const COLLECTION_PICS: GlobalSignal<BTreeMap<&str, &str>> = GlobalSignal::new(|| {
    BTreeMap::from([
        ("a", FIRST.path()),
        ("b", SECOND.path()),
        ("c", THIRD.path()),
        ("d", FORTH.path()),
        ("e", FIFTH.path()),
    ])
});

#[allow(clippy::unwrap_used)]
fn get_random_collection_picture() -> &'static str {
    COLLECTION_PICS
        .read()
        .values()
        .choose(&mut rand::thread_rng())
        .unwrap()
}

pub const HOME: &str = asset!("./public/home.svg");
pub const EXPLORE: &str = asset!("./public/explore.svg");
pub const SIDEBAR_COLLECTION: &str = asset!("./public/collections.svg");
pub const ARROW_RIGHT: &str = asset!("./public/keyboard_arrow_right.svg");
pub const SIM_CARD: &str = asset!("./public/sim_card_download.svg");
pub const DRAG_INDICATOR: &str = asset!("./public/drag_indicator.svg");
pub const TAILWIND_STR: &str = asset!("./public/tailwind.css");

/// `(Pages)`: Current active page
/// `Option<Pages>`: Previous page
static HISTORY: GlobalSignal<History> = GlobalSignal::new(|| History::new(Pages::MainPage));

/// `History` is used to keep track of the navigation history in the application.
/// It contains the following fields:
/// * `active`: The current active page.
/// * `history`: A vector of pages that have been visited.
/// * `prev_steps`: The number of steps taken back in the history.
///
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
    #[must_use]
    pub fn new(page: Pages) -> Self {
        Self {
            active: page.clone(),
            history: vec![page],
            prev_steps: 0,
        }
    }
    #[must_use]
    pub const fn active(&self) -> &Pages {
        &self.active
    }
    #[must_use]
    pub const fn history(&self) -> &Vec<Pages> {
        &self.history
    }
    #[must_use]
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

use rust_lib::api::shared_resources::{collection::CollectionId, entry::STORAGE};

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

#[component]
fn App() -> Element {
    let error_active = use_signal(|| true);
    rsx! {
        head::Link {
            href: TAILWIND_STR,
            rel: "stylesheet",
        }
        div {
            class: "[&_*]:transform-gpu bg-deep-background h-screen w-screen font-display leading-normal",
            ErrorBoundary {
                handle_error: move |error| { rsx! {
                    Modal {
                        active: error_active,
                        id: "error_modal",
                        div {
                            class: "flex",
                            div {
                                flex_basis: "10%",
                            }
                            div { class: "w-full bg-background overflow-x-scroll flex flex-col items-center space-y-3",
                                flex_basis: "80%",
                                div { class: "text-red text-3xl font-black",
                                    "Hmm, something went wrong. Please copy the following error to the developer."
                                }
                                pre {
                                    class: "max-w-full overflow-x-scroll font-[13px] font-bold",
                                    "{error:#?}"
                                }
                            }
                            div {
                                flex_basis: "10%",
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

pub trait ToRenderError {
    fn to_render_error(&self) -> RenderError;
}

impl<T: std::fmt::Display> ToRenderError for T {
    fn to_render_error(&self) -> RenderError {
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
            .map_err(ToRenderError::to_render_error)
            .map(|x| x.cloned())
    }
}

#[must_use]
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
            return Err(err.to_formatted().to_render_error());
        }
    }

    let mut error_handler = use_context_provider(|| Signal::new(None));
    let keys = use_keys();

    use_effect(move || {
        let binding = || {
            let history = HISTORY.read();
            Pages::DownloadProgress.apply_slide_in();
            let pages_scroller = vec![Pages::MainPage, Pages::Explore, Pages::Collections];
            Pages::scroller_applyer(pages_scroller, |x| x == &history.active)?;
            for collection_id in keys() {
                Pages::collection_display(collection_id.clone()).apply_slide_in();
                Pages::collection_edit(collection_id.clone()).apply_slide_in();
            }
            Ok::<_, anyhow::Error>(())
        };
        error_handler.set(Some(binding()));
    });

    use_memo(move || {
        if let Some(x) = error_handler.read().as_ref() {
            return x.as_ref().cloned().map_err(ToRenderError::to_render_error);
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
                    LayoutContainer {
                        MainPage {

                        }
                    }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full",
                    id: Pages::Explore.scroller_id(),
                    LayoutContainer {
                        Explore {

                        }
                    }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full",
                    id: Pages::Collections.scroller_id(),
                    Collections {

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

#[must_use]
pub fn use_keys() -> Memo<Vec<CollectionId>> {
    use_memo(|| STORAGE.collections.read().keys().cloned().collect())
}

#[component]
fn CollectionContainer() -> Element {
    let ids = use_keys();
    let should_render_ids = ids()
        .into_iter()
        .filter(|x| Pages::collection_display(x.clone()).should_render());
    rsx! {
        for collection_id in should_render_ids {
            div {
                class: "absolute inset-0 z-0 min-h-full min-w-full",
                key: "page {collection_id}",
                id: Pages::collection_display(collection_id).slide_in_id(),
                SingleCollectionContainer {
                    id: collection_id.clone(),
                }
            }
        }
    }
}

#[component]
fn SingleCollectionContainer(id: CollectionId) -> Element {
    rsx! {
        div {
            class: "bg-deep-background min-h-screen rounded-xl min-w-full",
            CollectionDisplay {
                collection_id: id
            }
        }
    }
}

/// Does dynmaic rendering
#[component]
fn LayoutContainer(children: Element, #[props(default)] extended_class: String) -> Element {
    rsx! {
        div {
            class: tw_merge!("bg-deep-background min-h-screen min-w-full", extended_class),
            div {
                class: "flex flex-col transition-all xl:items-center xl:*:justify-center xl:*:max-w-[1240px] xl:*:w-full",
                {children}
            }
        }
    }
}

#[component]
fn Explore() -> Element {
    let state = use_signal(|| State::Left);
    rsx! {
        FloatingSwitch {
            lhs_width: 80.,
            lhs: CURSEFORGE_OUTLINE("transition-all fill-background w-[40px] group-data-[selected=Right]:w-[30px] group-data-[selected=Right]:fill-secondary-surface"),
            lhs_css: "px-[20px] py-[10px]",
            rhs_width: 80.,
            rhs: MODRINTH_OUTLINE("transition-all fill-background w-[35px] group-data-[selected=Left]:w-[30px] group-data-[selected=Left]:fill-secondary-surface"),
            rhs_css: "px-[20px] py-[10px]",
            floater: "bg-orange group-data-[selected=Right]:bg-green",
            class: "h-[80px]",
            state
        }
        FloatingSwitch {
            lhs_width: 80.,
            lhs: GRASS(()),
            lhs_css: "px-[20px] py-[15px]",
            rhs_width: 120.,
            rhs: CREATE_COLLECTION(()),
            rhs_css: "px-[20px] py-[15px]",
            floater: "bg-secondary-surface",
            state
        }
    }
}
