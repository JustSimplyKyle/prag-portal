pub mod mod_renderer;

use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error, info, trace, warn, Level};
use manganis::ImageAsset;
use mod_renderer::ModViewer;
use rust_lib::api::{
    backend_exclusive::vanilla::launcher::LoggerEvent, shared_resources::collection::CollectionId,
};
use strum::EnumIter;

use crate::{
    impl_context_switcher,
    main_page::STAR,
    pages::Pages,
    use_error_handler,
    BaseComponents::{
        atoms::button::{Button, FillMode, Roundness},
        molecules::{
            search_bar::SearchBar,
            switcher::{Comparison, StateSwitcher},
        },
        string_placements::{Alignment, ContentType, Contents},
    },
};

pub static DISPLAY_BACKGROUND: ImageAsset = asset!("./public/cool_image.png").image();

pub static GAME_CONTROLLER: Asset = asset!("./public/stadia_controller.svg");
pub static UNARCHIVE: Asset = asset!("./public/unarchive.svg");
pub static CUBE: Asset = asset!("./public/deployed_code.svg");
pub static GLOBAL_ASIA: Asset = asset!("./public/globe_asia.svg");
pub static CIRCLE_JOIN: Asset = asset!("./public/join.svg");
pub static MOTION_MODE: Asset = asset!("./public/motion_mode.svg");
pub static DELETE: Asset = asset!("./public/delete.svg");
pub static UNDO: Asset = asset!("./public/undo.svg");
pub static HORIZ: Asset = asset!("./public/more_horiz.svg");
pub static BRIGHT_LEFT_ARROW: Asset = asset!("./public/bright_left_arrow.svg");
pub static MODRINTH: Asset = asset!("./public/modrinth.svg");
pub static CURSEFORGE: Asset = asset!("./public/curseforge.svg");

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, EnumIter)]
pub(crate) enum CollectionDisplayTopSelection {
    Mods,
    World,
    ResourcePack,
    ShaderPacks,
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
    search: Signal<String>,
    default: String,
) -> Element {
    let mut radio = collection_id().use_collection_radio();
    let len = radio
        .read()
        .mod_controller()
        .map(|x| x.manager.mods.iter().filter(|x| x.enabled).count());
    let mut error_handler = use_error_handler();
    let logs = use_signal_sync(LoggerEvent::default);
    use_effect(move || {
        let logs = logs.read();
        let level = *logs.level();
        let output = format!("[{}] {}", logs.thread(), logs.message().unwrap_or_default());

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

    let launch_game = move || {
        spawn(async move {
            let mut collection = radio.read_owned();
            let err = move || async move {
                collection.launch_game(logs).await?;
                radio.replace(collection)?;
                Ok(())
            };
            if let Err(err) = err().await {
                error_handler.set(Err(err));
            }
        })
    };

    rsx! {
        div {
            class: "bg-deep-background *:py-[15px] flex gap-[5px] h-fit w-full",
            Button {
                roundness: Roundness::Squircle,
                extended_css_class: "bg-background",
                fill_mode: FillMode::Fit,
                clickable: false,
                string_placements: vec![
                    ContentType::svg(BRIGHT_LEFT_ARROW).css("svg-[40px]").align_center(),
                ],
            }
            Button {
                roundness: Roundness::Squircle,
                extended_css_class: "bg-background pl-[25px] pr-[20px] min-w-[280px] max-w-[280px]",
                fill_mode: FillMode::Fit,
                clickable: false,
                string_placements: vec![
                    Contents::new(
                        vec![
                            ContentType::text("模組").css("font-medium"),
                            ContentType::text(format!("({})", len.unwrap_or_default())).css("text-hint font-english font-medium"),
                        ],
                        Alignment::Left,
                    ).css("gap-[5px] align-center"),
                    ContentType::svg(asset!("public/arrow_drop_down_40.svg")).css("svg-[40px]").align_right(),
                ],
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
                string_placements: vec![
                    ContentType::svg(HORIZ).align_center(),
                ],
            }
            Button {
                roundness: Roundness::Squircle,
                extended_css_class: "bg-background",
                fill_mode: FillMode::Fit,
                string_placements: vec![
                    ContentType::svg(STAR).css("svg-[40px]").align_center(),
                ],
            }
            Button {
                roundness: Roundness::Squircle,
                extended_css_class: "bg-white min-w-[150px]",
                fill_mode: FillMode::Fit,
                onclick: move |()| {
                    launch_game();
                },
                string_placements: vec![
                    {ContentType::svg(GAME_CONTROLLER).align_center()}
                ],
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
                DISPLAY_BACKGROUND
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
                                ContentType::custom(rsx!(
                                    div {
                                        class: "font-english text-black font-bold leading-[1.2] capsize",
                                        {loader.clone()}
                                    }
                                )).align_right(),
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
                            ContentType::text("由我建立").css("text-black").align_right()
                        ],
                    }
                    Button {
                        fill_mode: FillMode::Fit,
                        extended_css_class: "bg-white  pl-[20px] pr-[26px] py-[13px]",
                        roundness: Roundness::Pill,
                        clickable: false,
                        string_placements: vec![
                            ContentType::svg(GAME_CONTROLLER).css("svg-[30px]").align_left(),
                            ContentType::text("我的錦集").css("text-black").align_right()
                        ],
                    }
                }
            }
            div {
                class: "max-xl:hidden justify-self-end flex h-fit",
                img {
                    class: "rounded-l-[30px] shadow size-[280px] object-cover",
                    src: radio.read().picture_path().to_string_lossy().to_string()
                }
                div {
                    class: "rounded-r-[30px] grid grid-flow-row justify-center items-stretch bg-deep-background pt-[25px] pb-[25px] gap-[15px]",
                    div {
                        class: "self-start justify-self-center inline-flex items-center justify-center w-[35px]",
                        {ContentType::svg(asset!("./public/big_forge.svg"))}
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
pub fn CollectionDisplay(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let status: Signal<Comparison<CollectionDisplayTopSelection>> =
        use_signal(|| (CollectionDisplayTopSelection::Mods, None));

    let default = CopyValue::new(String::from("搜尋合集中的內容"));
    let search = use_signal(|| default.cloned());
    rsx! {
        div {
            class: "mr-[20px] w-full h-full",
            ScrollableFootBar {
                footer: rsx!(Footer { collection_id, search, default }),
                main: rsx!(Content { collection_id, }),
                bottom: rsx!(
                    div {
                        class: "relative overflow-y-scroll min-w-full max-w-full flex flex-col h-full",
                        match status().0 {
                            CollectionDisplayTopSelection::Mods => {
                                rsx!(ModViewer { collection_id, default, search: search() })
                            },
                            CollectionDisplayTopSelection::World => {
                                rsx!(ModViewer { collection_id, default, search: search() })
                            }
                            CollectionDisplayTopSelection::ResourcePack => {
                                rsx!(ModViewer { collection_id, default, search: search() })
                            },
                            CollectionDisplayTopSelection::ShaderPacks => {
                                rsx!(ModViewer { collection_id, default, search: search() })
                            },
                        }
                    }
                ),
            }
        }
    }
}
