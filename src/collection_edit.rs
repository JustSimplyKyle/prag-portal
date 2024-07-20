use std::{borrow::BorrowMut, path::PathBuf};

use dioxus::prelude::*;
use rust_lib::api::shared_resources::{collection::CollectionId, entry::STORAGE};
use strum::{EnumIter, IntoEnumIterator};
use tailwind_fuse::merge::tw_merge;

use crate::{
    collection_display::{DISPLAY_BACKGROUND, GAME_CONTROLLER, UNDO},
    impl_context_switcher, impl_optional_state_switcher,
    main_page::{ARROW_LEFT, ICON},
    pages::Pages,
    scrollable::Scrollable,
    BaseComponents::{
        atoms::button::{Button, FillMode, Roundness},
        molecules::switcher::{Comparison, StateSwitcher},
        string_placements::{Alignment, ContentType, Contents, Hint, Text},
    },
    ARROW_RIGHT, COLLECTION_PICS,
};

pub const ADD: &str = manganis::mg!(file("./public/add.svg"));
pub const HALLWAY: &str = manganis::mg!(file("./public/hallway.svg"));
pub const PHOTO_LIBRARY: &str = manganis::mg!(file("./public/photo_library.svg"));

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, EnumIter)]
pub enum EditState {
    Personalization,
    DataLog,
    Export,
    Advanced,
}

impl std::fmt::Display for EditState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "edit-{}",
            match self {
                EditState::Personalization => "personalization",
                EditState::DataLog => "datalog",
                EditState::Export => "export",
                EditState::Advanced => "advanced",
            }
        )
    }
}

impl Scrollable for EditState {
    const GROUP_SELECTOR: &'static str = "group-edit";
}

impl_context_switcher!(EditState);

impl_optional_state_switcher!(Pages);

#[component]
pub fn CollectionEditContainer() -> Element {
    let binding = STORAGE.collections.read();
    let collection_ids = binding
        .keys()
        .map(|id| (id, Pages::collection_edit(id.clone())));
    rsx! {
        for (collection_id , page) in collection_ids {
            div {
                class: "absolute inset-0 z-0 min-w-full min-h-full",
                id: page.slide_in_id(),
                if page.should_render() {
                    CollectionEdit {
                        key: "{page.slide_in_id()}",
                        collection_id: collection_id.clone()
                    }
                }
            }
        }
    }
}

#[component]
fn EditSidebar(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        div {
            class: "flex flex-col min-w-[400px] max-w-[400px] gap-[20px]",
            EditSidebarInfographic {
                collection_id
            }
            div {
                class: "flex flex-col",
                Button {
                    roundness: Roundness::Top,
                    fill_mode: FillMode::Fit,
                    extended_css_class: "bg-background",
                    focus_color_change: true,
                    switcher: EditState::Personalization,
                    string_placements: vec![
                        ContentType::text("風格化").align_left(),
                        ContentType::svg(ARROW_RIGHT).css("svg-[30px]").align_right(),
                    ]
                }
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "bg-background",
                    fill_mode: FillMode::Fit,
                    focus_color_change: true,
                    switcher: EditState::DataLog,
                    string_placements: vec![
                        ContentType::text("收藏紀錄").align_left(),
                        ContentType::svg(ARROW_RIGHT).css("svg-[30px]").align_right(),
                    ]
                }
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "bg-background",
                    fill_mode: FillMode::Fit,
                    focus_color_change: true,
                    switcher: EditState::Export,
                    string_placements: vec![
                        ContentType::text("分享&匯出").align_left(),
                        ContentType::svg(ARROW_RIGHT).css("svg-[30px]").align_right(),
                    ]
                }
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "bg-background",
                    fill_mode: FillMode::Fit,
                    focus_color_change: true,
                    switcher: EditState::Advanced,
                    string_placements: vec![
                        ContentType::text("進階選項").align_left(),
                        ContentType::svg(ARROW_RIGHT).css("svg-[30px]").align_right(),
                    ]
                }
            }
            div {
                class: "flex justify-stretch w-full gap-[10px]",
                Button {
                    roundness: Roundness::Pill,
                    onclick: move |_| {
                        Pages::collection_display(collection_id()).switch_active_to_self();
                    },
                    extended_css_class: "flex w-auto min-w-auto justify-center items-center bg-background gap-[15px] pl-[20px] pr-[30px]",
                    string_placements: vec![
                        ContentType::svg(UNDO).css("svg-[35px]").align_center(),
                        ContentType::text("返回頁面").align_center(),
                    ]
                }
                Button {
                    roundness: Roundness::Pill,
                    extended_css_class: "flex w-auto min-w-auto items-center bg-background gap-[15px] pl-[20px] pr-[30px]",
                    string_placements: vec![
                        ContentType::svg(ARROW_LEFT).align_center(),
                        ContentType::text("返回頁面").align_center(),
                    ]
                }
            }
        }
    }
}

#[component]
fn EditSidebarInfographic(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let collection = collection_id().get_collection();
    rsx! {
        div {
            class: "overflow-x-clip flex flex-col w-full",
            div {
                class: "flex flex-col p-5 justify-end rounded-t-[50px] w-full min-h-[250px]",
                background: format!(
                    "radial-gradient(171.48% 102.52% at 0% 100%, #000 0%, rgba(0, 0, 0, 0.00) 100%), url(\"{}\") lightgray 50% / cover no-repeat",
                    DISPLAY_BACKGROUND,
                ),
                {
                    ContentType::image(collection.read().picture_path().to_string_lossy().to_string()).css("w-[100px] h-[100px] bg-cover rounded-t-[50px] rounded-bl-[15px] rounded-br-[50px] p-[5px]")
                }
            }
            Button {
                roundness: Roundness::Bottom,
                clickable: false,
                extended_css_class: "bg-background overflow-x-clip justify-start px-5 pt-[22px]",
                string_placements: vec![
                    Contents::new(
                            vec![
                                ContentType::text(collection.read().display_name())
                                    .css("text-3xl font-black min-w-0 text-nowrap overflow-x-clip"),
                                ContentType::hint("由我建立•18 分鐘•不久前開啟")
                                    .css("font-medium text-[15px]"),
                            ],
                            Alignment::Left,
                        )
                        .css("flex flex-col gap-[15px]"),
                ]
            }
        }
    }
}

#[component]
fn CollectionEdit(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let edit_state: Signal<Comparison<EditState>> =
        use_context_provider(|| Signal::new((EditState::Personalization, None)));
    use_effect(move || {
        let vec = EditState::iter().collect::<Vec<_>>();
        EditState::scroller_applyer(vec, |x| &edit_state.read().0 == x).unwrap();
    });
    rsx! {
        div {
            class: "flex w-full bg-deep-background group-edit min-h-screen gap-[20px] rounded-[5px] px-[20px] pb-[20px]",
            "data-prev": edit_state().1.map_or_else(String::new, |x| x.to_string()),
            EditSidebar {
                collection_id
            }
            div {
                class: "w-full min-h-screen relative *:overflow-scroll",
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::Personalization.scroller_id(),
                    Personalization {
                        collection_id
                    }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::DataLog.scroller_id(),
                    DataLog {
                        collection_id
                    }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::Export.scroller_id(),
                    Export {
                        collection_id
                    }
                }
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: EditState::Advanced.scroller_id(),
                    Advanced {
                        collection_id
                    }
                }
            }
        }
    }
}

#[component]
fn EditTemplate(children: Element, title: Element) -> Element {
    rsx! {
        div {
            class: "flex flex-col min-h-screen bg-background px-[30px] pb-[30px] rounded-[30px]",
            div {
                class: "bg-background sticky top-0 z-50",
                div {
                    class: "flex flex-col bg-background pt-[30px] rounded-b-[30px]",
                    {title}
                    div {
                        class: "bg-background py-[10px] rounded-t-[30px]"
                    }
                }
            }
            div {
                class: "flex flex-col overflow-scroll *:z-10 gap-[20px]",
                {children}
            }
        }
    }
}

#[component]
fn Personalization(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        EditTemplate {
            title: rsx! {
                Button {
                    roundness: Roundness::None,
                    clickable: false,
                    extended_css_class: "rounded-[20px] p-[40px] gap-[20px]",
                    string_placements: vec![
                        Contents::new(
                                vec![
                                    ContentType::text("風格化")
                                        .css("font-black text-white text-[40px]"),
                                    ContentType::hint("自訂你的收藏樣式"),
                                ],
                                Alignment::Left,
                            )
                            .css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right(),
                    ]
                }
            },
            ModifyName { collection_id }
            ModifyPicture { collection_id }
        }
    }
}

#[component]
fn ModifyName(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let collection = collection_id().get_collection();
    let mut input = use_signal(|| None);
    rsx! {
        div {
            class: "flex flex-col gap-[3px] group",
            aria_selected: input.read().is_none(),
            Button {
                roundness: Roundness::Top,
                clickable: false,
                extended_css_class: "p-[25px]",
                string_placements: vec![
                    Contents::new(
                            vec![
                                ContentType::text("更改名稱"),
                                ContentType::hint(
                                    "名稱將會套用至此收藏的所有顯示位置",
                                ),
                            ],
                            Alignment::Left,
                        )
                        .css("flex flex-col gap-[15px]"),
                ]
            }
            Button {
                roundness: Roundness::Bottom,
                clickable: false,
                extended_css_class: "p-[25px] text-white group-aria-selected:text-zinc-800",
                string_placements: rsx! {
                    input {
                        oninput: move |x| {
                            let mut collection = collection_id().get_mut_collection();
                            collection.with_mut(|ele| *ele.display_name = x.value()).unwrap();
                            input.with_mut(|input| {
                                if let Some(input) = input {
                                    *input = x.value();
                                } else {
                                    *input = Some(x.value());
                                }
                            });
                        },
                        value: {
                            if let Some(x) = input() {
                                x
                            } else {
                                collection.read().display_name().clone()
                            }
                        },
                    }
                },
            }
        }
    }
}

#[component]
fn ModifyPicture(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let mut active = use_signal(|| "a");
    let mut change = use_signal(|| false);
    let collection = collection_id().get_collection();
    let active_to_collection = use_memo(move || match active() {
        "a" => COLLECTION_PICS[0].clone(),
        "b" => COLLECTION_PICS[1].clone(),
        "c" => COLLECTION_PICS[2].clone(),
        "d" => COLLECTION_PICS[3].clone(),
        "e" => COLLECTION_PICS[4].clone(),
        _ => panic!("impossible"),
    });

    let mut filename: Signal<Option<String>> = use_signal(|| None);

    use_effect(move || {
        if change() {
            let mut collection = collection_id().get_mut_collection();
            let path = PathBuf::from(active_to_collection().path());
            collection.with_mut(|x| *x.picture_path = path).unwrap();
            change.set(false);
        }
    });
    use_effect(move || {
        if let Some(x) = filename() {
            if !x.is_empty() {
                let mut collection = collection_id().get_mut_collection();
                let path = PathBuf::from(x);
                collection.with_mut(|x| *x.picture_path = path).unwrap();
            }
        }
    });
    rsx! {
        div {
            class: "flex flex-col gap-[3px] w-full",
            Button {
                roundness: Roundness::Top,
                clickable: false,
                extended_css_class: "p-[25px]",
                string_placements: vec![
                    Contents::new(
                            vec![
                                ContentType::text("封面與圖示圖片"),
                                ContentType::hint(
                                    "預覽即將套用的圖片，建議使用辨識度較高的圖片",
                                ),
                            ],
                            Alignment::Left,
                        )
                        .css("flex flex-col gap-[15px]"),
                ]
            }
            div {
                class: "flex h-min w-full",
                div {
                    class: "transition-all [&_*]:transition-all w-full h-full flex flex-col gap-[3px] group",
                    "data-active": active(),
                    Button {
                        roundness: Roundness::None,
                        clickable: false,
                        extended_css_class: "p-[20px]",
                        string_placements: vec![
                            ContentType::custom(rsx!{
                                div {
                                    class: "flex gap-[20px] items-center",
                                    {ContentType::svg(HALLWAY).css("svg-[35px]").align_left()}
                                    div {
                                        class: "flex flex-col gap-[10px]",
                                        Text { css: "text-xl", "預設封面圖片" }
                                        Hint { css: "text-[13px]", "使用Era Connect提供的預設圖片" }
                                    }
                                }
                            }).align_left(),
                            ContentType::custom(rsx!{
                                div {
                                    class: "flex gap-[3px]",
                                    button {
                                        onclick: move |_| {
                                            active.set("a");
                                            change.set(true);
                                        },
                                        {ContentType::image(COLLECTION_PICS[0].to_string()).css("bg-cover w-10 h-10 rounded-full border-2 border-zinc-900 group-data-[active=a]:border-white group-data-[active=a]:w-20")}
                                    }
                                    button {
                                        onclick: move |_| {
                                            active.set("b");
                                            change.set(true);
                                        },
                                        {ContentType::image(COLLECTION_PICS[1].to_string()).css("bg-cover w-10 h-10 rounded-full border-2 border-zinc-900 group-data-[active=b]:border-white group-data-[active=b]:w-20")}
                                    }
                                    button {
                                        onclick: move |_| {
                                            active.set("c");
                                            change.set(true);
                                        },
                                        {ContentType::image(COLLECTION_PICS[2].to_string()).css("bg-cover w-10 h-10 rounded-full border-2 border-zinc-900 group-data-[active=c]:border-white group-data-[active=c]:w-20")}
                                    }
                                    button {
                                        onclick: move |_| {
                                            active.set("d");
                                            change.set(true);
                                        },
                                        {ContentType::image(COLLECTION_PICS[3].to_string()).css("bg-cover w-10 h-10 rounded-full border-2 border-zinc-900 group-data-[active=d]:border-white group-data-[active=d]:w-20")}
                                    }
                                    button {
                                        onclick: move |_| {
                                            active.set("e");
                                            change.set(true);
                                        },
                                        {ContentType::image(COLLECTION_PICS[4].to_string()).css("bg-cover w-10 h-10 rounded-full border-2 border-zinc-900 group-data-[active=e]:border-white group-data-[active=e]:w-20")}
                                    }
                                }
                            }).align_right()
                        ]
                    }
                    Button {
                        roundness: Roundness::Bottom,
                        clickable: false,
                        extended_css_class: "p-[20px]",
                        string_placements: vec![
                            ContentType::custom(rsx!{
                                div {
                                    class: "flex gap-[20px] items-center",
                                    {ContentType::svg(PHOTO_LIBRARY).css("svg-[35px]").align_left()}
                                    div {
                                        class: "flex flex-col gap-[10px]",
                                        Text { css: "text-xl", "從電腦尋找" }
                                        Hint { css: "text-[13px]", "使用你電腦中的圖片" }
                                    }
                                }
                            }).align_left(),
                            ContentType::custom(rsx!{
                                div {
                                    class: "relative w-10 h-10 p-2.5 bg-zinc-900 rounded-full flex items-center justify-center",
                                    input {
                                        r#type: "file",
                                        class: "absolute inset-0 w-fit h-fit",
                                        accept: ".png,.jpg,.avif,.heif",
                                        multiple: false,
                                        onchange: move |evt| {
                                            if let Some(files) = evt.files() {
                                                filename.set(files.files().first().cloned());
                                            }
                                        },
                                        {ContentType::svg(ADD).css("svg-[20px]")}
                                    }
                                    {ContentType::svg(ADD).css("svg-[20px]")}
                                }
                            }).align_right()
                        ]
                    }
                }
                {ContentType::image(collection().picture_path().to_string_lossy().to_string()).css("flex-initial bg-cover size-[163px] p-[15px] rounded-[5px]")}
            }
        }
    }
}

#[component]
fn DataLog(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        EditTemplate {
            title: rsx! {
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "rounded-[20px] p-[40px]",
                    string_placements: vec![
                        Contents::new(
                                vec![
                                    ContentType::text("收藏紀錄")
                                        .css("font-black text-white text-[40px]"),
                                    ContentType::hint("查看這個收藏的資訊"),
                                ],
                                Alignment::Left,
                            )
                            .css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right(),
                    ]
                }
            }
        }
    }
}

#[component]
fn Export(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        EditTemplate {
            title: rsx! {
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "rounded-[20px] p-[40px]",
                    string_placements: vec![
                        Contents::new(
                                vec![
                                    ContentType::text("分享").css("font-black text-white text-[40px]"),
                                    ContentType::hint("分享你的收藏或是將它匯出至電腦"),
                                ],
                                Alignment::Left,
                            )
                            .css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right(),
                    ]
                }
            }
        }
    }
}

#[component]
fn Advanced(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    rsx! {
        EditTemplate {
            title: rsx! {
                Button {
                    roundness: Roundness::None,
                    extended_css_class: "rounded-[20px] p-[40px]",
                    string_placements: vec![
                        Contents::new(
                                vec![
                                    ContentType::text("進階選項")
                                        .css("font-black text-white text-[40px]"),
                                    ContentType::hint("單獨修改此收藏的進階選項"),
                                ],
                                Alignment::Left,
                            )
                            .css("flex flex-col gap-[20px]"),
                        ContentType::svg(GAME_CONTROLLER).css("svg-[70px]").align_right(),
                    ]
                }
            }
        }
    }
}
