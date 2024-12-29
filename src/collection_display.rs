pub mod mod_renderer;

use dioxus::{prelude::*, CapturedError};
use dioxus_logger::tracing::{debug, error, info, trace, warn, Level};
use mod_renderer::ModViewer;
use notify::Watcher;
use rust_lib::api::{
    backend_exclusive::vanilla::launcher::LoggerEvent,
    shared_resources::collection::{CollectionError, CollectionId, ScreenShot},
};
use strum::EnumIter;
use tokio::{fs, io::BufReader};
use tokio_stream::StreamExt;

use crate::{
    impl_context_switcher,
    main_page::STAR,
    pages::Pages,
    use_error_handler,
    BaseComponents::{
        atoms::button::{Button, FillMode, Roundness},
        molecules::{
            context_menu::{self, ContextMenu, DropDown, Menu},
            search_bar::SearchBar,
            switcher::{Comparison, StateSwitcher},
        },
        string_placements::{Alignment, ContentType, Contents},
    },
    SnafuToCapturedError,
};

pub static DISPLAY_BACKGROUND: Asset = asset!("/assets/cool_image.png");

pub static GAME_CONTROLLER: Asset = asset!("/assets/stadia_controller.svg");
pub static UNARCHIVE: Asset = asset!("/assets/unarchive.svg");
pub static CUBE: Asset = asset!("/assets/deployed_code.svg");
pub static GLOBAL_ASIA: Asset = asset!("/assets/globe_asia.svg");
pub static CIRCLE_JOIN: Asset = asset!("/assets/join.svg");
pub static MOTION_MODE: Asset = asset!("/assets/motion_mode.svg");
pub static DELETE: Asset = asset!("/assets/delete.svg");
pub static UNDO: Asset = asset!("/assets/undo.svg");
pub static HORIZ: Asset = asset!("/assets/more_horiz.svg");
pub static BRIGHT_LEFT_ARROW: Asset = asset!("/assets/bright_left_arrow.svg");
pub static MODRINTH: Asset = asset!("/assets/modrinth.svg");
pub static CURSEFORGE: Asset = asset!("/assets/curseforge.svg");

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, EnumIter)]
pub(crate) enum CollectionDisplayTopSelection {
    Mods,
    World,
    ResourcePack,
    ShaderPacks,
    ScreenShots,
}

impl_context_switcher!(CollectionDisplayTopSelection);

#[derive(Clone)]
pub enum Action {
    Start,
    Stop,
}

#[component]
pub fn ScrollableFootBar(main: Element, footer: Element, bottom: Element) -> Element {
    rsx! {
        div {
            class: "bg-deep-background relative flex flex-col h-screen gap-[20px]",
            overflow_y: "scroll",
            div {
                class: "grow h-full",
                {main}
            }
            div {
                class: "pb-[20px] flex-none sticky top-0 z-[1000]",
                {footer}
            }
            div {
                class: "flex-none min-h-[calc(100%-100px)] inset-0 w-full absolute top-full z-0",
                {bottom}
            }
        }
    }
}

#[component]
fn Footer(
    collection_id: ReadOnlySignal<CollectionId>,
    status: Signal<CollectionDisplayTopSelection>,
    search: Signal<String>,
    screenshots: Resource<Result<Vec<ScreenShot>, CollectionError>>,
    default: String,
) -> Element {
    let mut radio = collection_id().use_collection_radio();

    let mods_len = radio
        .read()
        .mod_controller()
        .map(|x| x.manager.mods.iter().filter(|x| x.enabled).count());

    let mut error_handler = use_error_handler();

    let screenshots_len = match &*screenshots.read() {
        Some(Ok(x)) => x.len(),
        Some(Err(err)) => Err(err.to_render_error())?,
        None => 0,
    };

    let logs = use_signal_sync(LoggerEvent::default);
    use_effect(move || {
        let logs = logs.read();
        let level = *logs.level();

        let msg = logs.message().unwrap_or_default();

        if msg.contains("screenshot") {
            screenshots.restart();
        }

        let output = format!("[{}] {msg}", logs.thread());

        drop(logs);

        match level {
            Level::TRACE => {
                trace!("{output}");
            }
            Level::DEBUG => {
                debug!("{output}");
            }
            Level::INFO => {
                info!("{output}");
            }
            Level::WARN => {
                warn!("{output}");
            }
            Level::ERROR => {
                error!("{output}");
            }
        }
    });
    let mut selector_visibility = use_signal(|| false);

    let base = |s, focus_right: bool| {
        use CollectionDisplayTopSelection as S;
        let len = match s {
            S::Mods => mods_len.unwrap_or_default(),
            S::World => 0,
            S::ResourcePack => 0,
            S::ShaderPacks => 0,
            S::ScreenShots => screenshots_len,
        };

        rsx! {
            div {
                class: "gap-[5px] grid grid-flow-col items-center text-hint font-medium text-[20px]",
                onclick: move |_| {
                    status.set(s);
                    selector_visibility.set(false);

                    screenshots.restart();
                },
                div {
                    aria_selected: status() == s,
                    class: "aria-selected:text-white justify-self-start",
                    match s {
                        S::Mods => "模組",
                        S::World => "地圖",
                        S::ResourcePack => "資源包",
                        S::ShaderPacks => "光影包",
                        S::ScreenShots => "螢幕捷圖"
                    }
                }
                div {
                    aria_selected: status() == s,
                    class: "aria-selected:text-white font-english text-secondary-surface",
                    class: if focus_right { "justify-self-end" } else { "justify-self-start" },
                    "({len})"
                }
            }
        }
    };

    rsx! {
        div {
            class: "bg-deep-background *:py-[15px] flex gap-[5px] h-fit w-full",
            Button {
                roundness: Roundness::Squircle,
                extended_css_class: "bg-background",
                fill_mode: FillMode::Fit,
                clickable: false,
                string_placements: vec![ContentType::svg(BRIGHT_LEFT_ARROW).css("svg-[40px]").align_center()],
            }
            DropDown {
                class: "min-w-[280px]",
                base: base(status(), false),
                onclick: move |()| {

                    screenshots.restart();
                },
                selector_visibility,
                {base(CollectionDisplayTopSelection::Mods, true)}
                {base(CollectionDisplayTopSelection::ScreenShots, true)}
                {base(CollectionDisplayTopSelection::World, true)}
                {base(CollectionDisplayTopSelection::ResourcePack, true)}
            }
            SearchBar {
                search,
                default,
            }
            Button {
                roundness: Roundness::Squircle,
                extended_css_class: "bg-background",
                fill_mode: FillMode::Fit,
                onclick: move |()| {
                    Pages::collection_edit(collection_id()).switch_active_to_self();
                },
                string_placements: vec![ContentType::svg(HORIZ).align_center()],
            }
            Button {
                roundness: Roundness::Squircle,
                extended_css_class: "bg-background",
                fill_mode: FillMode::Fit,
                string_placements: vec![ContentType::svg(STAR).css("svg-[40px]").align_center()],
            }
            Button {
                roundness: Roundness::Squircle,
                extended_css_class: "bg-white min-w-[150px]",
                fill_mode: FillMode::Fit,
                onclick: move |()| async move {
                    if let Err(err) = radio
                        .with_async_mut(move |mut collection| async move {
                            collection.launch_game(logs).await?;
                            Ok(collection)
                        })
                        .await
                    {
                        error!("collection throwed {err:?}");
                        error_handler.set(Err(err.into()));
                    }
                },
                string_placements: vec![{ ContentType::svg(GAME_CONTROLLER).align_center() }],
            }
        }
    }
}

#[component]
fn Content(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let radio = collection_id().use_collection_radio();
    let mod_loader = CopyValue::new(radio.read().mod_loader().map(ToString::to_string));
    rsx! {
        div {
            class: "rounded-[30px] w-full h-full p-[40px] grid grid-flow-col justify-stretch items-end",
            background_color: "#191919",
            background: format!(
                "linear-gradient(145deg, rgba(25, 25, 25, 0.00) 18.18%, #191919 88.98%), url(\'{}\') lightgray 50% / cover no-repeat",
                DISPLAY_BACKGROUND,
            ),
            div {
                class: "justify-self-start flex flex-col gap-[35px]",
                div {
                    class: "flex flex-col gap-[25px]",
                    div {
                        class: "text-[80px] font-black text-white trim",
                        {radio.read().display_name().clone()}
                    }
                    div {
                        class: "text-white text-[25px] font-english [&_*]:font-english font-bold leading-[1.2] capsize",
                        "Minecraft {radio.read().minecraft_version().id}"
                    }
                }
                div {
                    class: "flex gap-[10px]",
                    if let Some(loader) = &*mod_loader.read() {
                        Button {
                            fill_mode: FillMode::Fit,
                            extended_css_class: "bg-white pl-[20px] pr-[26px] py-[13px]",
                            roundness: Roundness::Pill,
                            clickable: false,
                            string_placements: vec![
                                ContentType::svg(GAME_CONTROLLER).css("svg-[30px]").align_left(),
                                ContentType::custom(
                                        rsx!(
                                            div { class :
                                            "font-english text-black font-bold trim", { loader
                                            .clone() } }
                                        ),
                                    )
                                    .align_right(),
                            ],
                        }
                    }
                    Button {
                        fill_mode: FillMode::Fit,
                        extended_css_class: "bg-white pl-[20px] pr-[26px] py-[13px]",
                        roundness: Roundness::Pill,
                        clickable: false,
                        string_placements: vec![
                            ContentType::svg(GAME_CONTROLLER).css("svg-[30px]").align_left(),
                            ContentType::text("由我建立").css("text-black").align_right(),
                        ],
                    }
                    Button {
                        fill_mode: FillMode::Fit,
                        extended_css_class: "bg-white  pl-[20px] pr-[26px] py-[13px]",
                        roundness: Roundness::Pill,
                        clickable: false,
                        string_placements: vec![
                            ContentType::svg(GAME_CONTROLLER).css("svg-[30px]").align_left(),
                            ContentType::text("我的錦集").css("text-black").align_right(),
                        ],
                    }
                }
            }
            div {
                class: "max-xl:hidden justify-self-end flex h-fit",
                img {
                    class: "rounded-l-[30px] shadow size-[280px] object-cover",
                    src: radio.read().picture_path().to_string_lossy().to_string(),
                }
                div {
                    class: "rounded-r-[30px] grid grid-flow-row justify-center items-stretch bg-deep-background pt-[25px] pb-[25px] gap-[15px]",
                    div {
                        class: "self-start justify-self-center inline-flex items-center justify-center w-[35px]",
                        {ContentType::svg(asset!("/assets/big_forge.svg"))}
                    }
                    div {
                        class: "self-end [writing-mode:vertical-rl] rotate-180 inline-flex items-center w-20 text-3xl text-hint font-english italic font-bold uppercase trim",
                        if let Some(loader) = &*mod_loader.read() {
                            {loader.clone()}
                        } else {
                            "vanilla"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Screenshots(
    collection_id: ReadOnlySignal<CollectionId>,
    screenshots: Resource<Result<Vec<ScreenShot>, CollectionError>>,
    search: ReadOnlySignal<String>,
) -> Element {
    let read = screenshots.read();
    let screenshots = match &*read {
        Some(Ok(x)) => x,
        Some(Err(err)) => Err(err.to_render_error())?,
        None => &vec![],
    };

    rsx! {
        div {
            class: "rounded-[30px] bg-background px-[30px] pb-[30px] flex flex-col gap-[20px] text-[18px] text-white",
            div {
                class: "grid grid-flow-col items-center justify-stretch gap-[10px] px-[20px] my-[10px] h-[70px]",
                div {
                    class: "justify-self-start flex items-center gap-[5px] grow w-full font-medium",
                    div {
                        class: "w-[100px]",
                        "檔案名稱",
                    }
                    div {
                        class: "w-full grow text-hint font-display",
                        "(拍攝日期/檔案大小/圖片大小)"
                    }
                }
                div {
                    class: "justify-self-end flex w-[180px] items-center self-stretch gap-[5px]",
                    "依照日期排序"
                    crate::svgs::ARROW_DOWN {}
                }
            }
            div {
                class: "grid grid-flow-row gap-[5px]",
                grid_auto_rows: "310px",
                grid_auto_columns: "390px",
                grid_template_columns: "repeat(auto-fill,390px)",
                for screenshot in screenshots {
                    div {
                        class: "flex flex-col items-start bg-deep-background rounded-[30px]",
                        div {
                            class: "text-white flex justify-start items-center p-[20px] text-[18px] font-english font-bold",
                            height: "60px",
                            div {
                                class: "trim",
                                "{screenshot.path.file_name().unwrap_or_default().to_string_lossy()}"
                            }
                        }
                        {ContentType::image(&screenshot.path).css("w-[390px] h-[200px] object-cover overflow-hidden")}
                        div {
                            class: "flex gap-[5px] items-center p-[20px] text-secondary font-english text-[15px] font-medium",
                            height: "50px",
                            div {
                                class: "trim",
                                {screenshot.get_creation_date()?.format("%Y.%m.%d").to_string()}
                            }
                            div {
                                "/"
                            }
                            div {
                                class: "trim",
                                "{screenshot.get_size()?.to_megabytes():.2} MB"
                            }
                            div {
                                "/"
                            }
                            div {
                                class: "trim",
                                {
                                    if let Some((x,y)) = image::image_dimensions(&screenshot.path).ok() {
                                        format!("{x}x{y}")
                                    } else {
                                        String::new()
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CollectionDisplay(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let status: Signal<CollectionDisplayTopSelection> =
        use_signal(|| CollectionDisplayTopSelection::Mods);

    let default = CopyValue::new(String::from("搜尋合集中的內容"));
    let search = use_signal(|| default.cloned());

    let radio = collection_id().use_collection_radio();

    let screenshots = use_resource(move || async move { radio.read().get_screenshots().await });

    rsx! {
        div {
            class: "mr-[20px] w-full h-full",
            ScrollableFootBar {
                footer: rsx! {
                    Footer {
                        collection_id,
                        status,
                        search,
                        screenshots,
                        default,
                    }
                },
                main: rsx! {
                    Content {
                        collection_id,
                    }
                },
                bottom: rsx! {
                    div {
                        class: "relative overflow-y-scroll min-w-full max-w-full flex flex-col h-full",
                        match status() {
                            CollectionDisplayTopSelection::Mods => {
                                rsx! {
                                    ModViewer {
                                        collection_id,
                                        default,
                                        search: search(),
                                    }
                                }
                            }
                            CollectionDisplayTopSelection::World => {
                                rsx! {
                                    ModViewer {
                                        collection_id,
                                        default,
                                        search: search(),
                                    }
                                }
                            }
                            CollectionDisplayTopSelection::ResourcePack => {
                                rsx! {
                                    ModViewer {
                                        collection_id,
                                        default,
                                        search: search(),
                                    }
                                }
                            }
                            CollectionDisplayTopSelection::ShaderPacks => {
                                rsx! {
                                    ModViewer {
                                        collection_id,
                                        default,
                                        search: search(),
                                    }
                                }
                            }
                            CollectionDisplayTopSelection::ScreenShots => {
                                rsx! {
                                    Screenshots {
                                        collection_id,
                                        screenshots,
                                        search: search()
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
