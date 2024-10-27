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
use dioxus_logger::tracing::{info, Level};
use dioxus_radio::hooks::use_init_radio_station;
use manganis::ImageAsset;
use pages::Pages;
use rand::seq::IteratorRandom;
use scrollable::Scrollable;
use snafu::ErrorCompat;
use std::{collections::BTreeMap, path::PathBuf, time::Duration};
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

const FIRST: ImageAsset = asset!("./public/first_collection_pic.png").image();
const SECOND: ImageAsset = asset!("./public/second_collection_pic.png").image();
const THIRD: ImageAsset = asset!("./public/third_collection_pic.png").image();
const FORTH: ImageAsset = asset!("./public/forth_collection_pic.png").image();
const FIFTH: ImageAsset = asset!("./public/fifth_collection_pic.png").image();

pub const COLLECTION_PICS: GlobalSignal<BTreeMap<&str, PathBuf>> = GlobalSignal::new(|| {
    BTreeMap::from([
        ("a", FIRST.resolve()),
        ("b", SECOND.resolve()),
        ("c", THIRD.resolve()),
        ("d", FORTH.resolve()),
        ("e", FIFTH.resolve()),
    ])
});

#[allow(clippy::unwrap_used)]
fn get_random_collection_picture() -> PathBuf {
    COLLECTION_PICS
        .read()
        .values()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone()
}

pub const HOME: Asset = asset!("./public/home.svg");
pub const EXPLORE: Asset = asset!("./public/explore.svg");
pub const SIDEBAR_COLLECTION: Asset = asset!("./public/collections.svg");
pub const ARROW_RIGHT: Asset = asset!("./public/keyboard_arrow_right.svg");
pub const SIM_CARD: Asset = asset!("./public/sim_card_download.svg");
pub const DRAG_INDICATOR: Asset = asset!("./public/drag_indicator.svg");
pub const TAILWIND_STR: Asset = asset!("./public/tailwind.css");

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

use rust_lib::api::shared_resources::collection::{
    use_collections_radio, Collection, CollectionId, FetchCollectionChannel,
};

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new().with_window(
        WindowBuilder::new()
            .with_decorations(true)
            .with_title("Prag Portal")
            .with_inner_size(PhysicalSize::new(1600, 920)),
    );
    LaunchBuilder::desktop().with_cfg(cfg).launch(|| {
        use rust_lib::api::shared_resources::collection::Collections as C;
        use_init_radio_station::<C, FetchCollectionChannel>(|| {
            C(Collection::scan()
                .unwrap()
                .into_iter()
                .flatten()
                .map(|x| (x.get_collection_id(), x))
                .collect())
        });
        App()
    });
}

#[cfg(debug_assertions)]
#[component]
fn TailwindSetup() -> Element {
    let tailwind_config = include_str!("../tailwind.config.js");
    let input_css = include_str!("../input.css");
    rsx! {

        document::Script { src: "https://cdn.tailwindcss.com" }

        document::Style {
            r#type: "text/tailwindcss",
            {input_css}
        }
        document::Link {
            href: TAILWIND_STR,
            rel: "stylesheet",
        }

        document::Script {
            {tailwind_config}
        }
    }
}

#[cfg(not(debug_assertions))]
#[component]
fn TailwindSetup() -> Element {
    rsx! {
        document::Link {
            href: TAILWIND_STR,
            rel: "stylesheet",
        }
    }
}

#[component]
fn App() -> Element {
    let error_active = use_signal(|| true);
    rsx! {
        TailwindSetup {}
        div {
            class: "[&_*]:transform-gpu bg-deep-background h-screen w-screen font-display leading-normal",
            ErrorBoundary {
                handle_error: move |error| { rsx! {
                    Modal {
                        active: error_active,
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
    fn throw(&mut self);
}

impl<P, K: Into<anyhow::Error> + Send + Sync + 'static, T: FnMut() -> Result<P, K>> ThrowResource<T>
    for T
{
    fn throw(&mut self) {
        if let Err(x) = self() {
            use std::error::Error;
            let boxed_error: Box<dyn Error + Send + Sync> = Box::from(x.into());
            let leaked_error: &'static (dyn Error + Send + Sync) = Box::leak(boxed_error);
            ScopeId::APP.throw_error(leaked_error);
        }
    }
}

#[must_use]
pub fn use_error_handler() -> Signal<Result<(), anyhow::Error>> {
    use_context()
}

#[must_use]
pub fn use_mounted() -> Signal<Option<std::rc::Rc<MountedData>>> {
    use_signal(|| None)
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Size2D {
    pub width: f64,
    pub height: f64,
}

#[must_use]
pub fn use_visible_size(
    mounted: Signal<Option<std::rc::Rc<MountedData>>>,
) -> Signal<Option<Size2D>> {
    let mut signal = use_signal(|| None);
    use_future(move || async move {
        loop {
            let current_size = if let Some(x) = &*mounted.read() {
                x.get_client_rect()
                    .await
                    .map(|x| Size2D {
                        width: x.width(),
                        height: x.height(),
                    })
                    .unwrap_or_default()
            } else {
                Size2D::default()
            };
            if *signal.peek() != Some(current_size) {
                signal.set(Some(current_size));
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    });
    signal
}

#[must_use]
pub fn use_scroll_size(
    mounted: Signal<Option<std::rc::Rc<MountedData>>>,
) -> Signal<Option<Size2D>> {
    let mut signal = use_signal(|| None);
    use_future(move || async move {
        loop {
            let current_size = if let Some(x) = &*mounted.read() {
                x.get_scroll_size()
                    .await
                    .map(|x| Size2D {
                        width: x.width,
                        height: x.height,
                    })
                    .unwrap_or_default()
            } else {
                Size2D::default()
            };
            if *signal.peek() != Some(current_size) {
                signal.set(Some(current_size));
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    });
    signal
}

pub trait ErrorFormatted {
    fn to_formatted(&self) -> String;
}

impl<T: std::fmt::Debug + ErrorCompat + 'static + snafu::Error> ErrorFormatted for T {
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

fn t_create_collection() -> Result<(), RenderError> {
    let mut error_handler = use_error_handler();
    let collections_radio = use_collections_radio();

    let err = use_resource(move || collection_builder(None, "1.20.1", collections_radio));

    let read = err.read();
    let Some(id) = read
        .as_ref()
        .map(|x| x.as_ref().cloned())
        .transpose()
        .map_err(|x| x.to_formatted().to_render_error())?
    else {
        return Ok(());
    };
    let radio = id.use_collection_radio();
    let mut write_radio = id.use_collection_radio();
    spawn(async move {
        info!("Adding mods...");
        let binding = move || async move {
            let mut collection = radio.read_owned();
            collection
                .add_multiple_modrinth_mod(
                    vec![
                        "fabric-api",
                        "sodium",
                        "modmenu",
                        "ferrite-core",
                        "lazydfu",
                        "create-fabric",
                        "iris",
                        "indium",
                    ],
                    vec![],
                    None,
                )
                .await?;
            collection.download_mods().await?;
            write_radio.replace(collection)?;
            Ok(())
        };
        error_handler.set(binding().await);
        info!("Finished downloading mods");
    });

    Ok(())
}

#[component]
fn Layout() -> Element {
    let error_handler: Signal<Result<(), anyhow::Error>> =
        use_context_provider(|| Signal::new(Ok(())));

    // t_create_collection()?;

    let keys = use_keys();

    use_effect(move || {
        let mut binding = || {
            let history = HISTORY.read();
            Pages::DownloadProgress.apply_slide_in();
            let pages_scroller = vec![Pages::MainPage, Pages::Explore, Pages::Collections];
            Pages::scroller_applyer(pages_scroller, |x| x == &history.active)?;
            for collection_id in keys() {
                Pages::collection_display(collection_id).apply_slide_in();
                Pages::collection_edit(collection_id).apply_slide_in();
            }
            Ok::<_, anyhow::Error>(())
        };

        binding.throw();
    });

    use_memo(move || {
        if let Err(x) = error_handler.read().as_ref() {
            return Err(x.to_render_error());
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
                class: "bg-deep-background w-screen h-screen relative *:overflow-scroll",
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
pub fn use_keys() -> ReadOnlySignal<Vec<CollectionId>> {
    ReadOnlySignal::new(Signal::new(
        use_collections_radio().read().0.keys().cloned().collect(),
    ))
}

#[component]
fn CollectionContainer() -> Element {
    let should_render_ids = use_keys()()
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
    let enabled = use_signal(|| false);
    let enabled2 = use_signal(|| false);
    rsx! {
        FloatingSwitch {
            lhs_width: 80.,
            lhs: rsx! {
                CURSEFORGE_OUTLINE {
                    class: "transition-all fill-background w-[40px] group-data-[selected=Right]:w-[30px] group-data-[selected=Right]:fill-secondary-surface"
                }
            },
            lhs_css: "px-[20px] py-[10px]",
            rhs_width: 80.,
            rhs: rsx! {
                MODRINTH_OUTLINE {
                    class: "transition-all fill-background w-[35px] group-data-[selected=Left]:w-[30px] group-data-[selected=Left]:fill-secondary-surface"
                }
            },
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
            rhs: rsx! { CREATE_COLLECTION {} },
            rhs_css: "px-[20px] py-[15px]",
            floater: "bg-secondary-surface",
            state
        }
        div {
            class: "flex flex-col bg-background",
            // Foldable {
            //     enabled,
            //     title: rsx! {
            //         div {
            //             class: "size-[100px]",
            //         }
            //     },
            //     div {
            //         class: "flex flex-col gap-[20px]",
            //         div {
            //             class: "text-[80px] bg-deep-background",
            //             "ABCDEFG"
            //         }
            //         div {
            //             class: "text-[80px] bg-deep-background",
            //             "HIJKLMNOP"
            //         }
            //         div {
            //             class: "text-[80px] bg-deep-background",
            //             "QRSTUV"
            //         }
            //     }
            // }
            // Foldable {
            //     enabled: enabled2,
            //     title: rsx! {
            //         div {
            //             class: "size-[100px]",
            //         }
            //     },
            //     div {
            //         class: "flex flex-col gap-[20px]",
            //         div {
            //             class: "text-[80px] bg-deep-background",
            //             "ABCDEFG"
            //         }
            //         div {
            //             class: "text-[80px] bg-deep-background",
            //             "HIJKLMNOP"
            //         }
            //         div {
            //             class: "text-[80px] bg-deep-background",
            //             "QRSTUV"
            //         }
            //     }
            // }
        }
    }
}
