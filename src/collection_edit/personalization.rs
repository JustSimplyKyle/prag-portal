pub const ADD: &str = manganis::mg!("./public/add.svg");
pub const HALLWAY: &str = manganis::mg!("./public/hallway.svg");
pub const PHOTO_LIBRARY: &str = manganis::mg!("./public/photo_library.svg");

use std::path::PathBuf;

use dioxus::prelude::*;
use rust_lib::api::shared_resources::collection::CollectionId;
use tailwind_fuse::tw_merge;

use crate::{
    collection_display::GAME_CONTROLLER,
    collection_edit::EditTemplate,
    use_error_handler,
    BaseComponents::{
        atoms::button::{Button, Roundness},
        string_placements::{Alignment, ContentType, Contents},
    },
    ThrowResource, COLLECTION_PICS,
};

#[component]
pub fn Personalization(collection_id: ReadOnlySignal<CollectionId>) -> Element {
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
    rsx! {
        div {
            class: "flex flex-col gap-[3px]",
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
            CollectionNameEdit {
                collection_id,
                class: "bg-deep-background rounded-b-3xl"
            }
        }
    }
}

#[component]
pub fn CollectionNameEdit(
    collection_id: ReadOnlySignal<CollectionId>,
    custom_input: Option<Signal<Option<String>>>,
    #[props(default)] class: String,
) -> Element {
    let input = use_signal(|| None);
    let mut input = custom_input.unwrap_or(input);
    let mut error_handler = use_error_handler();
    use_effect(move || {
        if let Some(x) = input() {
            let err = collection_id()
                .with_mut_collection(|ele| {
                    ele.display_name = x;
                })
                .map_err(Into::into);

            if let Err(err) = err {
                error_handler.set(Err(err));
            }
        }
    });
    rsx! {
        input {
            aria_selected: input.read().is_none(),
            class: tw_merge!("p-[25px] text-white aria-selected:text-zinc-800 text-[20px]", class),
            oninput: move |x| {
                input.set(Some(x.value()));
            },
            value: {
                input().map_or_else(|| collection_id().get_collection().read().display_name().clone(), |x| x)
            },
        }
    }
}

#[component]
fn ModifyPicture(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let mut change = use_signal(|| false);
    let collection = collection_id().get_collection();
    let mut active = use_signal(|| {
        COLLECTION_PICS
            .read()
            .iter()
            .find(|(_, x)| **x == collection.read().picture_path().to_string_lossy())
            .map(|x| *x.0)
    });

    let mut filename: Signal<Option<String>> = use_signal(|| None);

    use_effect(move || {
        let mut binding = || {
            if change() {
                if let Some(x) = active() {
                    let path = PathBuf::from(COLLECTION_PICS.read()[x].to_owned());
                    collection_id().with_mut_collection(|x| x.picture_path = path)?;
                    change.set(false);
                }
            }
            Ok::<(), anyhow::Error>(())
        };
        binding.throw();
    });

    use_effect(move || {
        let mut binding = || {
            if let Some(x) = filename() {
                if !x.is_empty() {
                    let path = PathBuf::from(x);
                    collection_id().with_mut_collection(|x| x.picture_path = path)?;
                }
            }
            Ok::<(), anyhow::Error>(())
        };
        binding.throw();
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
                                        div { class: "text-xl trim", "預設封面圖片" }
                                        div { class: "text-[13px] text-secondary-text trim", "使用Era Connect提供的預設圖片" }
                                    }
                                }
                            }).align_left(),
                            ContentType::custom(rsx!{
                                div {
                                    class: "flex gap-[3px]",
                                    button {
                                        onclick: move |_| {
                                            active.set(Some("a"));
                                            change.set(true);
                                        },
                                        {ContentType::image(COLLECTION_PICS.read()["a"].to_owned()).css("bg-cover w-10 h-10 rounded-full border-2 border-zinc-900 group-data-[active=a]:border-white group-data-[active=a]:w-20")}
                                    }
                                    button {
                                        onclick: move |_| {
                                            active.set(Some("b"));
                                            change.set(true);
                                        },
                                        {ContentType::image(COLLECTION_PICS.read()["b"].to_owned()).css("bg-cover w-10 h-10 rounded-full border-2 border-zinc-900 group-data-[active=b]:border-white group-data-[active=b]:w-20")}
                                    }
                                    button {
                                        onclick: move |_| {
                                            active.set(Some("c"));
                                            change.set(true);
                                        },
                                        {ContentType::image(COLLECTION_PICS.read()["c"].to_owned()).css("bg-cover w-10 h-10 rounded-full border-2 border-zinc-900 group-data-[active=c]:border-white group-data-[active=c]:w-20")}
                                    }
                                    button {
                                        onclick: move |_| {
                                            active.set(Some("d"));
                                            change.set(true);
                                        },
                                        {ContentType::image(COLLECTION_PICS.read()["d"].to_owned()).css("bg-cover w-10 h-10 rounded-full border-2 border-zinc-900 group-data-[active=d]:border-white group-data-[active=d]:w-20")}
                                    }
                                    button {
                                        onclick: move |_| {
                                            active.set(Some("e"));
                                            change.set(true);
                                        },
                                        {ContentType::image(COLLECTION_PICS.read()["e"].to_owned()).css("bg-cover w-10 h-10 rounded-full border-2 border-zinc-900 group-data-[active=e]:border-white group-data-[active=e]:w-20")}
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
                                        div { class: "text-xl trim", "從電腦尋找" }
                                        div { class: "text-[13px] text-secondary-text trim", "使用你電腦中的圖片" }
                                    }
                                }
                            }).align_left(),
                            ContentType::custom(rsx!{
                                label {
                                    class: "relative w-10 h-10 p-2.5 bg-zinc-900 rounded-full flex items-center justify-center",
                                    role: "button",
                                    input {
                                        r#type: "file",
                                        class: "hidden",
                                        accept: ".png,.jpg,.avif,.heif",
                                        multiple: false,
                                        onchange: move |evt| {
                                            if let Some(files) = evt.files() {
                                                filename.set(files.files().first().cloned());
                                            }
                                        },
                                    }
                                    {ContentType::svg(ADD).css("svg-[20px]")}
                                }
                            }).align_right()
                        ]
                    }
                }
                {ContentType::image(collection().picture_path().to_string_lossy().to_string()).css("flex-initial bg-cover min-w-[163px] min-h-[163px] max-w-[163px] max-h-[163px] p-[15px] rounded-[5px]")}
            }
        }
    }
}
