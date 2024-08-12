use std::ops::Deref;

use dioxus::prelude::*;
use dioxus_elements::geometry::PixelsRect;
use manganis::ImageAsset;
use rust_lib::api::{
    backend_exclusive::mod_management::mods::ModMetadata,
    shared_resources::collection::CollectionId,
};
use strum::EnumIter;
use tailwind_fuse::tw_merge;
use tokio_stream::StreamExt;

use crate::{
    collections::{ARROW_DOWN, SEARCH},
    impl_context_switcher,
    main_page::{ARROW_LEFT, STAR},
    pages::Pages,
    text_scroller::use_text_scroller,
    BaseComponents::{
        atoms::{
            button::{Button, FillMode, Roundness, Size},
            switch::Switch,
        },
        molecules::{
            search_bar::{FilterSearch, SearchBar},
            switcher::{Comparison, StateSwitcher, StateSwitcherSelectionBar, ToClass},
        },
        string_placements::{Alignment, ContentType, Contents, Hint, StringPlacements, Text},
    },
    EXPLORE,
};

pub static DISPLAY_BACKGROUND: ImageAsset = asset!(image("./public/cool_image.png").preload());

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
fn CollectionBackground(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let launch_game = use_coroutine(move |mut rx| async move {
        while let Some(action) = rx.next().await {
            match action {
                Action::Start => {
                    let mut collection = collection_id().get_collection_owned();
                    collection.launch_game().await.unwrap();
                    let collection_to_replace =
                        &mut *collection_id().try_get_raw_mut_collection().unwrap();
                    *collection_to_replace = collection;
                }
                Action::Stop => {}
            }
        }
    });
    let (onmounted, status, style) = use_text_scroller();
    let collection = collection_id().get_collection();
    let len = collection.with(|x| x.mod_controller().map(|x| x.manager.mods.len()));
    let mod_loader = collection.read().mod_loader().map(ToString::to_string);
    let status: Signal<Comparison<CollectionDisplayTopSelection>> =
        use_signal(|| (CollectionDisplayTopSelection::Mods, None));
    let default = CopyValue::new(String::from("搜尋合集中的內容"));
    let mut value = use_signal(|| default.cloned());
    rsx! {
        ScrollableFootBar {
            footer: rsx!(
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
                        extended_css_class: "bg-background min-w-[545px] w-full",
                        fill_mode: FillMode::Fit,
                        clickable: false,
                        string_placements: vec![
                            ContentType::custom(rsx!(
                                input {
                                    class: "w-full text-hint font-medium text-xl leading-[1.2] capsize",
                                    onfocusin: move |_| {
                                        if &*value.read() == &default.cloned() {
                                            value.set(String::new());
                                        }
                                    },
                                    onfocusout: move |_| {
                                        value.set(default.cloned());
                                    },
                                    oninput: move |x| {
                                        value.set(x.value());
                                    },
                                    value: value(),
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
                        extended_css_class: "bg-background min-w-[150px]",
                        fill_mode: FillMode::Fit,
                        string_placements: rsx![
                            div {
                                background: "var(--unnamed, linear-gradient(90deg, #C92B460%, #C92B4620%, #9747FF20%, #9747FF40%, #7CAED340%, #7CAED360%, #14AE5C60%, #14AE5C80%, #CDE34780%, #CDE347100%));",
                                {ContentType::svg(GAME_CONTROLLER).align_center()}
                            }
                        ],
                    }
                }
            ),
            main: rsx!(
                div {
                    class: "rounded-[30px] w-full h-full p-[40px] grid grid-flow-col justify-stretch items-end",
                    background: format!(
                        "radial-gradient(198.55% 100% at 50% 0%, rgba(25, 25, 25, 0.00) 0%, #191919 82.94%), url(\'{}\') lightgray 50% / cover no-repeat",
                        DISPLAY_BACKGROUND,
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
                            if let Some(loader) = mod_loader {
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
                    img {
                        class: "justify-self-end rounded-[30px] size-[280px] object-cover",
                        src: collection.read().picture_path().to_string_lossy().to_string()
                    }
                }
            ),
            bottom: rsx!(
                div {
                    class: "relative overflow-y-scroll min-w-full max-w-full flex flex-col h-full",
                    if status().0 == CollectionDisplayTopSelection::Mods {
                        ModViewer {
                            collection_id,
                            default,
                            search: value()
                        }
                    }
                }
            ),
        }
    }
}

#[component]
pub fn GridRow(
    items: [Element; 6],
    #[props(default)] class: String,
    #[props(extends = div, extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let class = tw_merge!("flex items-center gap-[10px]", class);
    rsx! {
        div {
            class,
            ..attributes,
            div {
                class: "grow flex items-center w-full gap-[20px]",
                div {
                    class: "flex-none inline-flex justify-center w-[80px]",
                    {&items[0]}
                }
                div {
                    class: "grow w-full py-[10px]",
                    {&items[1]}
                }
            }
            div {
                class: "flex-none w-[80px] py-[10px] inline-flex justify-center",
                {&items[2]}
            }
            div {
                class: "flex-none w-[80px] py-[10px] inline-flex justify-center",
                {&items[3]}
            }
            div {
                class: "flex-none w-[80px] py-[10px] inline-flex justify-center",
                {&items[4]}
            }
            div {
                class: "flex-none w-[80px] py-[10px] inline-flex justify-center",
                {&items[5]}
            }
        }
    }
}

#[component]
pub fn CollectionDisplay(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        div {
            class: "mr-[20px] relative w-full h-full flex flex-col",
            CollectionBackground {
                collection_id,
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
        collection().mod_controller().cloned().map(move |mut x| {
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
                class: "w-full border-b-[3px] border-b-secondary-surface rounded-t-[30px] px-[50px] py-[10px] backdrop-blur-[7.5px] sticky top-0 z-[2000]",
                background: "rgba(25, 25, 25, 0.90)",
                items: [
                    rsx!(
                        Text {
                            css: "flex-none inline-flex justify-center w-[80px] text-white text-lg",
                            "圖示"
                        }
                    ),
                    rsx!(
                        Text {
                            css: "grow w-full py-[10px] text-white text-lg",
                            "名稱（來源／文件名稱）"
                        }
                    ),
                    rsx!(
                        Text {
                            css: "flex-none w-[75px] py-[10px] text-white text-lg",
                            "更新"
                        }
                    ),
                    rsx!(
                        Text {
                            css: "flex-none w-[75px] py-[10px] text-white text-lg",
                            "刪除"
                        }
                    ),
                    rsx!(
                        Text {
                            css: "flex-none w-[75px] py-[10px] text-white text-lg",
                            "更多"
                        }
                    ),
                    rsx!(
                        Text {
                            css: "flex-none w-[75px] py-[10px] text-white text-lg",
                            "狀態"
                        }
                    ),
                ]
            }

            div {
                class: "bg-background w-full h-full flex flex-col px-[30px]",
                div {
                    class: "flex flex-col gap-[5px]",
                    FilterSearch {
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
fn SubModViewer(collection_id: ReadOnlySignal<CollectionId>, mods: ModMetadata) -> Element {
    let clicked = use_signal(|| false);
    let icon = rsx!(if let Some(icon) = mods.icon_url.as_ref() {
        {
            ContentType::image(icon.to_string()).css("size-[80px] rounded-[15px]")
        }
    });
    let name = rsx!(
        Text {
            css: "text-white text-[28px] font-bold font-english",
            {mods.name.clone()}
        }
    );
    let file_name = rsx!(
        if let Some(version) = &mods.mod_version {
            Hint {
                css: "font-medium text-hint text-[15px] font-english",
                {version.clone()}
            }
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
                rsx!(
                    {icon}
                ),
                rsx!(
                    div {
                        class: "flex flex-col justify-center gap-[15px]",
                        {name}
                        {file_name}
                    }
                ),
                rsx!(
                    {upgrade}
                ),
                rsx!(
                    {delete}
                ),
                rsx!(
                    {more}
                ),
                rsx!(
                    {status}
                ),
            ]
        }
    }
}

impl From<CollectionDisplayTopSelection> for StringPlacements {
    fn from(value: CollectionDisplayTopSelection) -> Self {
        use CollectionDisplayTopSelection as T;
        match value {
            T::Mods => vec![
                ContentType::svg(CUBE).css("svg-[30px]").align_left(),
                ContentType::text("模組").align_right(),
            ],
            T::World => vec![
                ContentType::svg(GLOBAL_ASIA).css("svg-[30px]").align_left(),
                ContentType::text("世界").align_right(),
            ],
            T::ResourcePack => vec![
                ContentType::svg(CIRCLE_JOIN).css("svg-[30px]").align_left(),
                ContentType::text("資源包").align_right(),
            ],
            T::ShaderPacks => vec![
                ContentType::svg(MOTION_MODE).css("svg-[30px]").align_left(),
                ContentType::text("光影包").align_right(),
            ],
        }
        .into()
    }
}

impl ToClass for CollectionDisplayTopSelection {
    fn to_class(&self) -> String {
        String::from("pl-[20px] pr-[25px] py-[12px]")
    }
}

#[component]
fn SelectionBar(
    sender: Signal<String>,
    status: Signal<Comparison<CollectionDisplayTopSelection>>,
) -> Element {
    rsx! {
        div {
            class: "grid grid-flow-col items-stretch",
            StateSwitcherSelectionBar {
                class: "justify-start",
                signal: status,
                default_state: CollectionDisplayTopSelection::Mods
            }
            div {
                class: "justify-end flex items-center space-x-[7px]",
                SearchBar {
                    sender
                }
                Button {
                    roundness: Roundness::Pill,
                    string_placements: vec![ContentType::svg(EXPLORE).css("svg-[25px]").align_center()],
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
