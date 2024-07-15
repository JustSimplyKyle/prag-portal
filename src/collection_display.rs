use dioxus::prelude::*;
use manganis::ImageAsset;
use rust_lib::api::{
    backend_exclusive::mod_management::mods::ModMetadata,
    shared_resources::collection::CollectionId,
};
use std::rc::Rc;
use strum::EnumIter;
use tokio_stream::StreamExt;

use crate::{
    impl_context_switcher,
    BaseComponents::{
        atoms::{
            button::{Button, FillMode, Roundness, Size},
            switch::Switch,
        },
        molecules::{
            search_bar::{SearchBar, SearchContainer},
            switcher::{Comparison, StateSwitcher, StateSwitcherSelectionBar, ToClass},
        },
        string_placements::{ContentType, Hint, StringPlacements, Text},
    },
    EXPLORE, HISTORY,
};

pub static DISPLAY_BACKGROUND: ImageAsset =
    manganis::mg!(image("./public/cool_image.png").preload());

pub static GAME_CONTROLLER: &str = manganis::mg!(file("./public/stadia_controller.svg"));
pub static UNARCHIVE: &str = manganis::mg!(file("./public/unarchive.svg"));
pub static CUBE: &str = manganis::mg!(file("./public/deployed_code.svg"));
pub static GLOBAL_ASIA: &str = manganis::mg!(file("./public/globe_asia.svg"));
pub static CIRCLE_JOIN: &str = manganis::mg!(file("./public/join.svg"));
pub static MOTION_MODE: &str = manganis::mg!(file("./public/motion_mode.svg"));
pub static DELETE: &str = manganis::mg!(file("./public/delete.svg"));
pub static UNDO: &str = manganis::mg!(file("./public/undo.svg"));
pub static HORIZ: &str = manganis::mg!(file("./public/more_horiz.svg"));

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
fn CollectionBackground(
    collection_id: ReadOnlySignal<CollectionId>,
    onmounted: EventHandler<Event<MountedData>>,
) -> Element {
    let launch_game = use_coroutine(move |mut rx| async move {
        while let Some(action) = rx.next().await {
            match action {
                Action::Start => {
                    use_future(move || async move {
                        let mut collection = collection_id.read().get_mut_collection();
                        collection.launch_game().await.unwrap();
                    });
                }
                Action::Stop => {}
            }
        }
    });
    let id = collection_id.read();
    let collection = id.get_collection();
    rsx! {
        div {
            onmounted,
            class: "sticky top-0 p-[50px] rounded-2xl grid grid-flow-col items-stretch",
            div {
                class: "fixed inset-0 h-[800px]",
                background: format!(
                    "radial-gradient(198.55% 100% at 50% 0%, rgba(25, 25, 25, 0.00) 0%, #191919 82.94%), url(\'{}\') lightgray 50% / cover no-repeat",
                    DISPLAY_BACKGROUND,
                )
            }
            div {
                class: "flex flex-col gap-[35px]",
                div {
                    class: "text-white font-black text-[80px] leading-normal capsize",
                    {collection.display_name.clone()}
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
            div {
                class: "flex justify-end",
                div {
                    class: "flex flex-col space-y-[3px] w-full max-w-[250px]",
                    img {
                        class: "w-full h-[250px] rounded-t-[20px] rounded-b-[5px] object-cover",
                        src: collection.picture_path.to_string_lossy().to_string()
                    }
                    div {
                        class: "flex space-x-[3px] min-w-full",
                        Button {
                            roundness: Roundness::None,
                            string_placements: vec![ContentType::svg(GAME_CONTROLLER).css("svg-[30px]").align_center()],
                            onclick: move |_| {
                                launch_game.send(Action::Start);
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
    }
}

#[component]
pub fn CollectionDisplay(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let status = use_signal(|| (CollectionDisplayTopSelection::Mods, None));
    let mod_search = use_signal(String::new);
    let top_position = use_signal(|| 0.);
    let mut container: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let height = use_resource(move || async move {
        if let Some(x) = container() {
            x.get_client_rect().await.ok().map(|x| x.height())
        } else {
            None
        }
    });
    rsx! {
        div {
            class: "relative flex flex-col",
            CollectionBackground {
                collection_id,
                onmounted: move |x: Event<MountedData>| {
                    container.set(Some(x.data()));
                }
            }
            div {
                class: "relative px-[30px] bg-background rounded-2xl min-h-dvh scroll-smooth",
                // top: "{top_position()}px",
                Separator {
                    top_position,
                    container_height: height().flatten()
                }
                div {
                    class: "flex flex-col gap-[15px]",
                    SelectionBar {
                        sender: mod_search,
                        status
                    }
                    if status().0 == CollectionDisplayTopSelection::Mods {
                        ModViewer {
                            collection_id,
                            search: mod_search()
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Separator(mut top_position: Signal<f64>, container_height: Option<f64>) -> Element {
    rsx! {
        div {
            class: "bg-background flex justify-center h-fit items-center py-[30px]",
            prevent_default: true,
            onmousemove: move |x: Event<MouseData>| {
                if let Some(height) = container_height {
                    if x
                        .data()
                        .held_buttons()
                        .contains(dioxus_elements::input_data::MouseButton::Primary)
                    {
                        let p = dbg!(x.data().client_coordinates().y - height - 35.);
                        top_position.set(p);
                    }
                }
            },
            cursor: "ns-resize",
            {ContentType::svg(manganis::mg!(file("public/Line 155.svg")))}
        }
    }
}

#[component]
fn ModViewer(collection_id: ReadOnlySignal<CollectionId>, search: String) -> Element {
    let mods = use_memo(move || {
        let collection = collection_id.read().get_collection_owned();
        collection.mod_controller.map(move |mut x| {
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
                    SubModViewer {collection_id, mods: x  }
                },
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        div {
            class: "grid grid-flow-row grid-cols-[repeat(auto-fill,273px)] gap-[3px]",
            SearchContainer {
                search,
                childrens: mods,
            }
        }
    }
}

#[component]
fn SubModViewer(
    collection_id: ReadOnlySignal<CollectionId>,
    mods: ReadOnlySignal<ModMetadata>,
) -> Element {
    let clicked = use_signal(|| false);
    let icon = use_memo(move || mods.read().icon_url.clone());
    rsx! {
        div {
            class: "bg-deep-background flex flex-col p-[10px] w-[273px] rounded-[5px]",
            div {
                class: "pb-[10px]",
                div {
                    class: "flex gap-[15px] items-center",
                    if let Some(icon) = &*icon.read() {
                        {ContentType::image(icon.to_string()).css("w-[50px] h-[50px] rounded-[10px]")}
                    }
                    div {
                        class: "flex flex-col gap-[10px]",
                        Text {
                            css: "text-xl font-bold",
                            {mods.read().name.clone()}
                        }
                        if let Some(version) = &mods.read().mod_version {
                            Hint {
                                css: "font-semibold text-xs italic",
                                {version.clone()}
                            }
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
                        string_placements: vec![ContentType::svg(UNARCHIVE).css("svg-[16px]").align_center()],
                        extended_css_class: "bg-background px-[15px] h-[30px]",
                        size: Size::Small,
                        fill_mode: FillMode::Fit
                    }
                    Button {
                        roundness: Roundness::Pill,
                        string_placements: vec![ContentType::svg(DELETE).css("svg-[16px]").align_center()],
                        extended_css_class: "bg-background px-[15px] h-[30px]",
                        size: Size::Small,
                        fill_mode: FillMode::Fit
                    }
                }
                div {
                    class: "justify-self-end",
                    Switch {
                        clicked
                    }
                }
            }
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
