use dioxus::{prelude::*, CapturedError};
use manganis::ImageAsset;
use rust_lib::api::{
    backend_exclusive::mod_management::mods::{ModMetadata, Platform},
    shared_resources::collection::CollectionId,
};
use strum::EnumIter;
use tailwind_fuse::tw_merge;

use crate::{
    collections::SEARCH,
    impl_context_switcher,
    main_page::STAR,
    pages::Pages,
    BaseComponents::{
        atoms::{
            button::{Button, FillMode, Roundness},
            switch::Switch,
        },
        molecules::{
            search_bar::FuzzyFilterer,
            switcher::{Comparison, StateSwitcher},
        },
        string_placements::{Alignment, ContentType, Contents, Hint, Text},
    },
};

pub static DISPLAY_BACKGROUND: ImageAsset = asset!(image("./public/cool_image.png"));

pub static GAME_CONTROLLER: &str = asset!("./public/stadia_controller.svg");
pub static UNARCHIVE: &str = asset!("./public/unarchive.svg");
pub static CUBE: &str = asset!("./public/deployed_code.svg");
pub static GLOBAL_ASIA: &str = asset!("./public/globe_asia.svg");
pub static CIRCLE_JOIN: &str = asset!("./public/join.svg");
pub static MOTION_MODE: &str = asset!("./public/motion_mode.svg");
pub static DELETE: &str = asset!("./public/delete.svg");
pub static UNDO: &str = asset!("./public/undo.svg");
pub static HORIZ: &str = asset!("./public/more_horiz.svg");
pub static BRIGHT_LEFT_ARROW: &str = asset!("./public/bright_left_arrow.svg");
pub static MODRINTH: &str = asset!("./public/modrinth.svg");
pub static CURSEFORGE: &str = asset!("./public/curseforge.svg");

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
    default: CopyValue<String>,
) -> Element {
    let collection = collection_id().get_collection();
    let len = collection.with(|x| {
        x.mod_controller()
            .map(|x| x.manager.mods.iter().filter(|x| x.enabled).count())
    });
    let launch_game = move || {
        spawn(async move {
            let mut collection = collection_id().get_collection()();
            collection.launch_game().await.unwrap();
            collection_id().replace(collection).unwrap();
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
            Button {
                roundness: Roundness::Squircle,
                extended_css_class: "bg-background min-w-fit w-full",
                fill_mode: FillMode::Fit,
                clickable: false,
                string_placements: vec![
                    ContentType::custom(rsx!(
                        input {
                            class: "w-full text-hint font-medium text-xl leading-[1.2] capsize",
                            onfocusin: move |_| {
                                if &*search.read() == &default.cloned() {
                                    search.set(String::new());
                                }
                            },
                            onfocusout: move |_| {
                                search.set(default.cloned());
                            },
                            oninput: move |x| {
                                search.set(x.value());
                            },
                            value: search(),
                        }
                    ))
                    .align_left()
                    .css("grow min-w-full justify-self-stretch"),
                    ContentType::svg(SEARCH).css("shrink-0").align_right(),
                ],
            }
            Button {
                roundness: Roundness::Squircle,
                extended_css_class: "bg-background",
                fill_mode: FillMode::Fit,
                onclick: move |_| {
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
                onclick: move |_| {
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
    let collection = collection_id().get_collection();
    let mod_loader = CopyValue::new(collection.read().mod_loader().map(ToString::to_string));
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
                    Text {
                        css: "text-[80px] font-black text-white",
                        {collection.read().display_name().clone()}
                    }
                    div {
                        class: "text-white text-[25px] font-english [&_*]:font-english font-bold leading-[1.2] capsize",
                        "Minecraft {collection.read().minecraft_version().id}"
                    }
                }
                div {
                    class: "flex gap-[10px]",
                    if let Some(loader) = &*mod_loader.read() {
                        Button {
                            fill_mode: FillMode::Fit,
                            extended_css_class: "bg-white text-black pl-[20px] pr-[26px] py-[13px]",
                            roundness: Roundness::Pill,
                            clickable: false,
                            string_placements: vec![
                                ContentType::svg(GAME_CONTROLLER).css("svg-[30px]").align_left(),
                                ContentType::text(loader).css("loader font-english font-bold").align_right()
                            ],
                        }
                    }
                    Button {
                        fill_mode: FillMode::Fit,
                        extended_css_class: "bg-white text-black pl-[20px] pr-[26px] py-[13px]",
                        roundness: Roundness::Pill,
                        clickable: false,
                        string_placements: vec![
                            ContentType::svg(GAME_CONTROLLER).css("svg-[30px]").align_left(),
                            ContentType::text("由我建立").align_right()
                        ],
                    }
                    Button {
                        fill_mode: FillMode::Fit,
                        extended_css_class: "bg-white text-black pl-[20px] pr-[26px] py-[13px]",
                        roundness: Roundness::Pill,
                        clickable: false,
                        string_placements: vec![
                            ContentType::svg(GAME_CONTROLLER).css("svg-[30px]").align_left(),
                            ContentType::text("我的錦集").align_right()
                        ],
                    }
                }
            }
            div {
                class: "max-xl:hidden justify-self-end flex h-fit",
                img {
                    class: "rounded-l-[30px] shadow size-[280px] object-cover",
                    src: collection.read().picture_path().to_string_lossy().to_string()
                }
                div {
                    class: "rounded-r-[30px] grid grid-flow-row justify-center items-stretch bg-deep-background pt-[25px] pb-[25px] gap-[15px]",
                    div {
                        class: "self-start justify-self-center inline-flex items-center justify-center w-[35px]",
                        {ContentType::svg(asset!("./public/big_forge.svg"))}
                    }
                    Text {
                        css: "self-end [writing-mode:vertical-rl] rotate-180 inline-flex items-center w-20 text-3xl text-hint font-english italic font-bold uppercase",
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

#[component]
pub fn GridRow<const T: usize>(
    items: [Element; T],
    #[props(default)] class: String,
    #[props(extends = div, extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    if T < 3 {
        let err = RenderError::Aborted(CapturedError::from_display("T should be greater than 2"));
        return Err(err);
    }
    let class = tw_merge!("flex items-center gap-[20px]", class);
    rsx! {
        div {
            class,
            ..attributes,
            div {
                class: "grow flex items-center h-full w-full gap-[20px]",
                div {
                    class: "flex-none inline-flex justify-center w-[75px]",
                    {&items[0]}
                }
                div {
                    class: "grow w-full flex items-center gap-[10px]",
                    div {
                        class: "grow w-full py-[10px]",
                        {&items[1]}
                    }
                    div {
                        class: "min-w-[150px] max-w-[150px]",
                        {&items[2]}
                    }
                }
            }
            div {
                class: "flex items-center h-full gap-[10px]",
                for i in 3..T {
                    div {
                        class: "flex-none w-[75px] py-[10px] inline-flex justify-center items-center",
                        {&items[i]}
                    }
                }
            }
        }
    }
}

#[component]
fn ModViewer(
    collection_id: ReadOnlySignal<CollectionId>,
    search: ReadOnlySignal<String>,
    default: String,
) -> Element {
    let mods = use_memo(move || {
        let collection = collection_id().get_collection();
        let binding = collection.read();
        binding.mod_controller().cloned().map(move |mut x| {
            x.manager.mods.sort_by_key(|x| x.name.clone());
            x.manager.mods
        })
    });
    let mods = mods()
        .into_iter()
        .flatten()
        .map(|x| {
            (
                x.name.clone(),
                rsx! {
                    SubModViewer {
                        collection_id,
                        mods: x
                    }
                },
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        div {
            class: "bg-background flex flex-col gap-[20px] rounded-t-[30px] pb-[30px] h-full overflow-x-hidden",
            GridRow {
                class: "w-full border-b-[3px] border-b-secondary-surface rounded-t-[30px] h-[70px] px-[50px] py-[10px] backdrop-blur-[7.5px] sticky top-0 z-[2000]",
                background: "rgba(25, 25, 25, 0.90)",
                items: [
                    rsx!(
                        Text {
                            css: "flex-none inline-flex justify-center w-[80px] text-white text-lg h-full",
                            "圖示"
                        }
                    ),
                    rsx!(
                        Text {
                            css: "text-white text-lg h-full",
                            "名稱（來源／文件名稱）"
                        }
                    ),
                    rsx!(
                        Text {
                            css: "text-white text-lg h-full",
                            "作者"
                        }
                    ),
                    rsx!(
                        Text {
                            css: "text-white text-lg h-full",
                            "更新"
                        }
                    ),
                    rsx!(
                        Text {
                            css: "text-white text-lg h-full",
                            "刪除"
                        }
                    ),
                    rsx!(
                        Text {
                            css: "text-white text-lg h-full",
                            "更多"
                        }
                    ),
                    rsx!(
                        Text {
                            css: "text-white text-lg h-full",
                            "狀態"
                        }
                    ),
                ]
            }

            div {
                class: "bg-background w-full h-full flex flex-col px-[30px]",
                div {
                    class: "flex flex-col gap-[5px]",
                    FuzzyFilterer {
                        search,
                        default,
                        childrens: mods,
                    }
                }
            }
        }
    }
}

#[component]
fn SubModViewer(
    collection_id: ReadOnlySignal<CollectionId>,
    mods: ReadOnlySignal<ModMetadata>,
) -> Element {
    let clicked = use_signal(|| mods.read().enabled);
    use_effect(move || {
        let clicked = clicked();
        let id = collection_id();
        spawn(async move {
            let mut collection = id.get_collection()();
            let manager = &mut collection.mod_controller.as_mut().unwrap().manager;
            let modify = manager
                .mods
                .iter_mut()
                .find(|x| &**x == &*mods.read())
                .unwrap();

            if clicked {
                modify.enable().await.unwrap();
            } else {
                modify.disable().await.unwrap();
            }

            id.replace(collection).unwrap();
        });
    });
    let icon = rsx!(if let Some(icon) = mods.read().icon_url.as_ref() {
        {
            ContentType::image(icon.to_string()).css("size-[80px] rounded-[15px]")
        }
    });
    let name = rsx!(
        div {
            class: "flex gap-[7px]",
            Text {
                css: "text-white text-[28px] font-bold font-english",
                {mods.read().name.clone()}
            }
            div {
                class: "w-[40px] bg-background inline-flex items-center justify-center h-[30px] px-[10px] rounded-[30px]",
                {
                    ContentType::svg(
                        match mods.read().platform() {
                            Platform::Modrinth => MODRINTH,
                            Platform::Curseforge => CURSEFORGE
                        })
                    .get_element()
                }
            }
        }
    );
    let file_name = rsx!(
        if let Some(version) = &mods.read().mod_version {
            Hint {
                css: "font-medium text-hint text-[15px] font-english",
                {version.clone()}
            }
        }
    );
    let author = rsx!(
        Hint {
            css: "text-[15px] font-english",
            {mods.read().authors.join(", ")}
        }
    );
    let upgrade = rsx!(Button {
        roundness: Roundness::None,
        extended_css_class:
            "flex items-center justify-center bg-background rounded-[15px] h-[60px]",
        string_placements: vec![ContentType::svg(UNARCHIVE).align_center()],
        fill_mode: FillMode::Fill
    });
    let delete = rsx!(Button {
        roundness: Roundness::None,
        extended_css_class:
            "flex items-center justify-center bg-background rounded-[15px] h-[60px]",
        string_placements: vec![ContentType::svg(DELETE).align_center()],
        fill_mode: FillMode::Fill
    });
    let more = rsx!(Button {
        roundness: Roundness::None,
        extended_css_class:
            "flex items-center justify-center bg-background rounded-[15px] h-[60px]",
        string_placements: vec![ContentType::svg(HORIZ).align_center()],
        fill_mode: FillMode::Fill
    });
    let status = rsx!(Switch { clicked });
    rsx! {
        GridRow {
            class: "bg-deep-background items-center rounded-[20px] p-[20px]",
            items: [
                icon,
                rsx!(
                    div {
                        class: "flex flex-col justify-center gap-[15px]",
                        {name}
                        {file_name}
                    }
                ),
                author,
                upgrade,
                delete,
                more,
                status
            ]
        }
    }
}
