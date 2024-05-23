use dioxus::prelude::*;
use manganis::ImageAsset;
use rust_lib::api::shared_resources::collection::Collection;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use crate::{
    main_page::COLLECTION_PIC,
    BaseComponents::{Button, ContentType, FillMode, Roundness, Switcher},
    Pages, ACTIVE_PAGE, EXPLORE,
};

pub static DISPLAY_BACKGROUND: ImageAsset = manganis::mg!(image("./public/cool_image.png")
    .format(ImageType::Avif)
    .preload());

pub static GAME_CONTROLLER: &str = manganis::mg!(file("./public/stadia_controller.svg"));
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
                            if let Some(x) = ACTIVE_PAGE().1 {
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
    let mut mods = collection().mod_manager.mods;
    mods.sort_unstable_by_key(|x| x.name.clone());
    rsx! {
        div {
            class: "grid grid-flow-row grid-cols-[repeat(auto-fill,300px)] gap-[3px]",
            for x in mods {
                div {
                    class: "bg-deep-background p-[10px] rounded-[5px]",
                    div {
                        class: "flex gap-[15px] items-center",
                        if let Some(icon) = x.get_icon_path() {
                            div {
                                {
                                    ContentType::image(icon.to_string_lossy()).css("w-[50px] h-[50px]").get_element()
                                }
                            }
                        }
                        div {
                            class: "flex flex-col gap-[10px]",
                            {ContentType::text(x.name).css("text-xl font-bold").get_element()}
                            if let Some(version) = x.mod_version {
                                {ContentType::hint(version).css("font-semibold text-xs italic").get_element()}
                            }
                        }
                    }
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
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    signal: Rc::new(CollectionDisplayTopSelection::Mods) as Rc<dyn Switcher>,
                    string_placements: vec![ContentType::text("A").align_left(), ContentType::text("模組").align_right()]
                }
                Button {
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    signal: Rc::new(CollectionDisplayTopSelection::World) as Rc<dyn Switcher>,
                    string_placements: vec![ContentType::text("B").align_left(), ContentType::text("世界").align_right()]
                }
                Button {
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    signal: Rc::new(CollectionDisplayTopSelection::ResourcePack) as Rc<dyn Switcher>,
                    string_placements: vec![
                        ContentType::text("C").align_left(),
                        ContentType::text("資源包").align_right(),
                    ]
                }
                Button {
                    roundness: Roundness::Pill,
                    fill_mode: FillMode::Fit,
                    signal: Rc::new(CollectionDisplayTopSelection::ShaderPacks) as Rc<dyn Switcher>,
                    string_placements: vec![
                        ContentType::text("D").align_left(),
                        ContentType::text("光影包").align_right(),
                    ]
                }
            }
            div { class: "justify-end flex items-center space-x-[7px] h-[55px] *:h-full",
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
