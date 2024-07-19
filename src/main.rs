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

use collection_edit::CollectionEditContainer;
use dioxus::desktop::tao::dpi::PhysicalSize;
use dioxus::desktop::WindowBuilder;
use dioxus::html::input_data::MouseButton;
use manganis::ImageAsset;
use pages::Pages;
use rust_lib::api::backend_exclusive::vanilla::version::VersionMetadata;
use rust_lib::api::shared_resources::collection::{ModLoader, ModLoaderType};
use scrollable::Scrollable;
use std::path::PathBuf;
use tailwind_fuse::*;
use BaseComponents::{
    atoms::button::{Button, FillMode, Roundness},
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

pub async fn collection_builder(
    picture_path: impl Into<Option<PathBuf>> + Send,
    version_id: impl Into<String> + Send,
) -> anyhow::Result<()> {
    let version = VersionMetadata::from_id(&version_id.into()).await?;
    let mut collection = entry::create_collection(
        "新的收藏",
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
    rsx! {
        div {
            class: "[&_*]:transform-gpu font-['GenSenRounded TW'] bg-deep-background min-h-screen min-w-full font-display leading-normal",
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
                            div { class: "w-full flex flex-col items-center space-y-3",
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

#[component]
fn Layout() -> Element {
    use_future(|| collection_builder(None, "1.20.1"));

    use_effect(move || {
        let history = HISTORY.read();
        Pages::DownloadProgress.apply_slide_in().unwrap();
        let pages_scroller = vec![Pages::MainPage, Pages::Explore, Pages::Collections];
        Pages::scroller_applyer(pages_scroller, |x| x == &history.active).unwrap();
        for collection_id in STORAGE.collections.read().keys().cloned() {
            Pages::collection_display(collection_id.clone())
                .apply_slide_in()
                .unwrap();
            Pages::collection_edit(collection_id)
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
            SideBar {

            }
            div {
                class: "bg-background w-full min-h-screen relative *:overflow-scroll",
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
    rsx! {
        for collection_id in STORAGE.collections.read().keys().cloned() {
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
