use dioxus::prelude::*;
use manganis::ImageAsset;
use rust_lib::api::{
    backend_exclusive::mod_management::mods::ModMetadata, shared_resources::collection::Collection,
};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    ops::Deref,
    rc::Rc,
    time::{Duration, Instant},
};
use tokio::time::interval;

use crate::BaseComponents::{SearchBar, Switch};
use crate::{
    main_page::COLLECTION_PIC,
    BaseComponents::{Button, ContentType, FillMode, Roundness, Switcher},
    Pages, EXPLORE, HISTORY,
};

pub static DISPLAY_BACKGROUND: ImageAsset = manganis::mg!(image("./public/cool_image.png")
    .format(ImageType::Avif)
    .preload());

pub static GAME_CONTROLLER: &str = manganis::mg!(file("./public/stadia_controller.svg"));
pub static UNARCHIVE: &str = manganis::mg!(file("./public/unarchive.svg"));
pub static CUBE: &str = manganis::mg!(file("./public/deployed_code.svg"));
pub static GLOBAL_ASIA: &str = manganis::mg!(file("./public/globe_asia.svg"));
pub static CIRCLE_JOIN: &str = manganis::mg!(file("./public/join.svg"));
pub static MOTION_MODE: &str = manganis::mg!(file("./public/motion_mode.svg"));
pub static DELETE: &str = manganis::mg!(file("./public/delete.svg"));
pub static UNDO: &str = manganis::mg!(file("./public/undo.svg"));
pub static HORIZ: &str = manganis::mg!(file("./public/more_horiz.svg"));

pub(crate) type Comparison<T> = (T, Option<T>);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum CollectionDisplayTopSelection {
    Mods,
    World,
    ResourcePack,
    ShaderPacks,
}

impl Switcher for CollectionDisplayTopSelection {
    fn hashed_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn compare(&self) -> bool {
        let top = use_context::<Signal<Comparison<Self>>>();
        &top().0 == self
    }

    fn switch_active_to_self(&self) {
        let mut global = use_context::<Signal<Comparison<Self>>>();
        let prev = global().0;
        if &prev != self {
            global.write().1 = Some(prev);
        }
        global.write().0 = *self;
    }
}

#[derive(Clone)]
pub enum Action {
    Start,
    Stop,
}
use futures_util::stream::StreamExt;

#[component]
pub fn CollectionDisplay(collection: ReadOnlySignal<Collection>) -> Element {
    let status: Signal<Comparison<CollectionDisplayTopSelection>> =
        use_context_provider(|| Signal::new((CollectionDisplayTopSelection::Mods, None)));
    let collection_client = use_coroutine(|mut rx: UnboundedReceiver<Action>| async move {
        while let Some(action) = rx.next().await {
            match action {
                Action::Start => {
                    collection().launch_game().await.unwrap();
                }
                Action::Stop => {}
            }
        }
    });
    rsx! {
        div { class: "relative flex flex-col",
            div { class: "sticky top-0 p-[50px] rounded-2xl grid grid-flow-col items-stretch",
                div {
                    class: "fixed inset-0 h-[900px]",
                    background: format!("radial-gradient(198.55% 100% at 50% 0%, rgba(25, 25, 25, 0.00) 0%, #191919 82.94%), url(\'{}\') lightgray 50% / cover no-repeat", DISPLAY_BACKGROUND),
                }
                div { class: "flex flex-col space-y-[35px]",
                    div { class: "text-white font-black text-[80px] leading-normal capsize",
                        {collection().display_name}
                    }
                    Button {
                        roundness: Roundness::Pill,
                        string_placements: vec![ContentType::svg(UNDO).css("svg-[30px]").align_center()],
                        onclick: move |_| {
                            if let Some(x) = HISTORY().prev_peek() {
                                x.switch_active_to_self();
                            }
                        },
                        fill_mode: FillMode::Fit,
                        extended_css_class: "w-fit shadow p-[13px]"
                    }
                }
                div { class: "flex justify-end",
                    div { class: "flex flex-col space-y-[3px] w-full max-w-[250px]",
                        img {
                            class: "w-full h-[250px] rounded-t-[20px] rounded-b-[5px] object-cover",
                            src: COLLECTION_PIC.to_string()
                        }
                        div { class: "flex space-x-[3px] min-w-full",
                            Button {
                                roundness: Roundness::None,
                                string_placements: vec![
                                    ContentType::svg(GAME_CONTROLLER)
                                        .css("svg-[30px]")
                                        .align_center()
                                ],
                                onclick: move |()| {
                                    collection_client.send(Action::Start);
                                },
                                fill_mode: FillMode::Fill,
                                extended_css_class: "px-[40px] py-[15px] rounded-[5px] rounded-bl-[20px] flex-1 min-w-0 bg-lime-300"
                            }
                            div {
                                Button {
                                    roundness: Roundness::None,
                                    string_placements: vec![ContentType::svg(HORIZ).css("svg-[25px]").align_center()],
                                    fill_mode: FillMode::Fit,
                                    background: "rgba(255,255,255,0.10)",
                                    backdrop_filter: "blur(50px)",
                                    extended_css_class: "rounded-[5px] rounded-[5px] rounded-br-[20px] flex-none"
                                }
                            }
                        }
                    }
                }
            }
            div { class: "px-[30px] bg-background rounded-2xl min-h-dvh scroll-smooth",
                div { class: "bg-background flex justify-center items-center min-h-full py-[30px]",
                    {ContentType::svg(manganis::mg!(file("public/Line 155.svg"))).get_element()}
                }
                div {
                    class: "flex flex-col gap-[15px]",
                    SelectionBar {}
                    if status().0 == CollectionDisplayTopSelection::Mods {
                        ModViewer {collection}
                    }
                }
            }
        }
    }
}

#[component]
fn ModViewer(collection: ReadOnlySignal<Collection>) -> Element {
    let collections = collection();
    if let Some(mut mods) = collections.mod_controller.map(|x| x.manager.mods) {
        mods.sort_unstable_by_key(|x| x.name.clone());
        rsx! {
            div {
                class: "grid grid-flow-row grid-cols-[repeat(auto-fill,273px)] gap-[3px]",
                for x in mods {
                    SubModViewer {collection, mods: x}
                }
            }
        }
    } else {
        None
    }
}

#[component]
fn SubModViewer(
    collection: ReadOnlySignal<Collection>,
    mods: ReadOnlySignal<ModMetadata>,
) -> Element {
    let clicked = use_signal(|| false);
    let icon = use_memo(move || mods().get_icon_path());
    rsx! {
        div {
            class: "bg-deep-background flex flex-col p-[10px] w-[273px] rounded-[5px]",
            div {
                class: "pb-[10px]",
                div {
                    class: "flex gap-[15px] items-center",
                    if let Some(icon) = icon() {
                        {ContentType::image(icon.to_string_lossy()).css("w-[50px] h-[50px] rounded-[10px]").get_element()}
                    }
                    div {
                        class: "flex flex-col gap-[10px]",
                        {ContentType::text(mods().name).css("text-xl font-bold").get_element()}
                        if let Some(version) = mods().mod_version {
                            {ContentType::hint(version).css("font-semibold text-xs italic").get_element()}
                        }
                    }
                }
            }
            div {
                class: "grid grid-flow-col items-stretch",
                div {
                    class: "justify-self-start flex gap-[5px]",
                    Button {
                        roundness: Roundness::Pill,
                        string_placements: vec![
                            ContentType::svg(UNARCHIVE)
                                .css("svg-[16px]")
                                .align_center()
                        ],
                        extended_css_class: "bg-background px-[15px] h-[30px]",
                        size: crate::BaseComponents::Size::Small,
                        fill_mode: FillMode::Fit
                    }
                    Button {
                        roundness: Roundness::Pill,
                        string_placements: vec![
                            ContentType::svg(DELETE)
                                .css("svg-[16px]")
                                .align_center()
                        ],
                        extended_css_class: "bg-background px-[15px] h-[30px]",
                        size: crate::BaseComponents::Size::Small,
                        fill_mode: FillMode::Fit
                    }
                }
                div {
                    class: "justify-self-end",
                    Switch {clicked}
                }
            }
        }
    }
}

#[component]
fn SelectionBar() -> Element {
    rsx! {
        div { class: "grid grid-flow-col items-stretch",
            div { class: "bg-deep-background rounded-full flex justify-start w-fit",
                Button {
                    extended_css_class: "pl-[20px] pr-[25px] py-[12px]",
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    signal: Rc::new(CollectionDisplayTopSelection::Mods) as Rc<dyn Switcher>,
                    focus_color_change: true,
                    string_placements: vec![
                        ContentType::svg(CUBE)
                            .css("svg-[30px]")
                            .align_left(),
                        ContentType::text("模組").align_right()
                    ]
                }
                Button {
                    extended_css_class: "pl-[20px] pr-[25px] py-[12px]",
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    focus_color_change: true,
                    signal: Rc::new(CollectionDisplayTopSelection::World) as Rc<dyn Switcher>,
                    string_placements: vec![
                        ContentType::svg(GLOBAL_ASIA)
                            .css("svg-[30px]")
                            .align_left(),
                        ContentType::text("世界").align_right()
                    ]
                }
                Button {
                    extended_css_class: "pl-[20px] pr-[25px] py-[12px]",
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    focus_color_change: true,
                    signal: Rc::new(CollectionDisplayTopSelection::ResourcePack) as Rc<dyn Switcher>,
                    string_placements: vec![
                        ContentType::svg(CIRCLE_JOIN)
                            .css("svg-[30px]")
                            .align_left(),
                        ContentType::text("資源包").align_right(),
                    ]
                }
                Button {
                    extended_css_class: "pl-[20px] pr-[25px] py-[12px]",
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    focus_color_change: true,
                    signal: Rc::new(CollectionDisplayTopSelection::ShaderPacks) as Rc<dyn Switcher>,
                    string_placements: vec![
                        ContentType::svg(MOTION_MODE)
                            .css("svg-[30px]")
                            .align_left(),
                        ContentType::text("光影包").align_right(),
                    ]
                }
            }
            div { class: "justify-end flex items-center space-x-[7px]",
                SearchBar {}
                Button {
                    roundness: Roundness::Pill,
                    string_placements: vec![
                        ContentType::svg(EXPLORE)
                            .css("svg-[25px]")
                            .align_center(),
                    ],
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
