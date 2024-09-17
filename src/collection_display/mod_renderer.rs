use std::ops::Deref;

use dioxus::{prelude::*, CapturedError};
use itertools::Itertools;
use rust_lib::api::{
    backend_exclusive::mod_management::mods::{ModMetadata, Platform},
    shared_resources::collection::CollectionId,
};
use tailwind_fuse::tw_merge;

use crate::{
    collection_display::{CURSEFORGE, DELETE, HORIZ, MODRINTH, UNARCHIVE},
    use_error_handler,
    BaseComponents::{
        atoms::{
            button::{Button, FillMode, Roundness},
            markdown_to_html,
            switch::Switch,
        },
        molecules::search_bar::fuzzy_search,
        organisms::{markdown_renderer::RenderTranslatedMarkdown, modal::Modal},
        string_placements::{ContentType, Hint, Text},
    },
};

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
pub fn ModViewer(
    collection_id: ReadOnlySignal<CollectionId>,
    search: ReadOnlySignal<String>,
    default: String,
) -> Element {
    let mods = use_resource(move || {
        let value = default.clone();
        let collection = collection_id().get_collection();
        async move {
            let binding = collection.read();
            let mods = binding
                .mod_controller()
                .into_iter()
                .flat_map(move |x| x.manager.mods.clone().into_iter())
                .map(|x| {
                    (
                        x.name.clone(),
                        rsx! {
                            SubModViewer {
                                collection_id,
                                mods: x,
                            }
                        },
                    )
                });
            fuzzy_search(search.read().deref(), &value, mods)
                .into_iter()
                .collect::<Vec<_>>()
        }
    });
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
                    for ele in mods().into_iter().flatten() {
                        {ele}
                    }
                }
            }
        }
    }
}

fn use_active_controller(
    clicked: Signal<bool>,
    collection_id: ReadOnlySignal<CollectionId>,
    mods: ReadOnlySignal<ModMetadata>,
) {
    let mut error_handler = use_error_handler();
    let _ = use_resource(move || {
        let clicked = clicked();
        let id = collection_id();
        let collection = id.get_collection();
        async move {
            let err = || async move {
                let Some(mut controller) = collection.peek().mod_controller.clone() else {
                    return Ok(());
                };
                let manager = &mut controller.manager;
                let modify = manager
                    .mods
                    .iter_mut()
                    .find(|x| x.deref() == mods.read().deref())
                    .unwrap();

                if clicked {
                    modify.enable().await?;
                } else {
                    modify.disable().await?;
                }

                if collection.peek().mod_controller() != Some(&controller) {
                    id.with_mut_collection(|x| x.mod_controller = Some(controller))?;
                }
                Ok(())
            };
            error_handler.set(Some(err().await))
        }
    });
}

#[component]
fn SubModViewer(
    collection_id: ReadOnlySignal<CollectionId>,
    mods: ReadOnlySignal<ModMetadata>,
) -> Element {
    let clicked = use_signal(|| mods.read().enabled);
    let mut dialog = use_signal(|| false);
    use_active_controller(clicked, collection_id, mods);
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
            Text {
                css: "font-medium text-secondary-text text-[15px] font-english",
                {version.clone()}
            }
        }
    );
    let author = rsx!(
        Text {
            css: "text-[15px] text-secondary-text font-english",
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
        onclick: move |()| {
            dialog.set(true);
        },
        extended_css_class:
            "flex items-center justify-center bg-background rounded-[15px] h-[60px]",
        string_placements: vec![ContentType::svg(HORIZ).align_center()],
        fill_mode: FillMode::Fill
    });
    let status = rsx!(Switch { clicked });
    rsx! {
        ModDetails {
            mods: mods.cloned(),
            active: dialog,
            clicked,
            collection_id,
        }
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

#[component]
fn ModDetails(
    mods: ReadOnlySignal<ModMetadata>,
    active: Signal<bool>,
    clicked: Signal<bool>,
    collection_id: ReadOnlySignal<CollectionId>,
) -> Element {
    use_active_controller(clicked, collection_id, mods);

    let description = markdown_to_html(&mods.read().long_description);

    let id = format!(
        "{}-my_dialog-{}",
        collection_id(),
        mods.read().project_id.to_string()
    );

    let file_paths = mods
        .read()
        .get_filepaths()
        .into_iter()
        .flatten()
        .map(|x| x.display().to_string())
        .join("\n");

    let file_size = use_resource(move || {
        let mods = mods();
        async move { mods.get_formatted_accumlated_size().await }
    });

    let size = file_size.as_ref().and_then(|x| x.as_ref().ok().cloned());

    rsx! {
        Modal {
            active,
            id,
            div {
                class: "w-full flex justify-center",
                div {
                    flex_basis: "10%",
                }
                div {
                    flex_basis: "80%",
                    div {
                        class: "flex flex-col w-full bg-background",
                        box_shadow: "10px 10px 30px 0px rgba(0, 0, 0, 0.25)",
                        div {
                            class: "grid grid-flow-col items-center h-fit justify-stretch p-[20px] pr-[30px] rounded-t-[30px] gap-[25px]",
                            div {
                                class: "justify-self-start flex gap-[25px] grow",
                                if let Some(path) = mods.read().get_icon_path() {
                                    img {
                                        class: "flex-0 size-[80px]",
                                        src: path.to_string_lossy().to_string(),
                                    }
                                }
                                div {
                                    class: "grow flex flex-col justify-center gap-[15px]",
                                    Text {
                                        css: "text-[30px] font-english font-bold",
                                        {mods.read().name.clone()}
                                    }
                                    Hint {
                                        css: "text-secondary-text text-[15px] font-medium font-english",
                                        {file_paths}
                                    }
                                }
                            }
                            div {
                                class: "justify-self-end",
                                Switch {
                                    class: "bg-deep-background",
                                    clicked,
                                }
                            }
                        }
                        div {
                            class: "flex items-center h-[50px] gap-[25px] px-[20px]",
                            div {
                                class: "flex w-fit justify-start gap-[7px] items-center",
                                div {
                                    class: "bg-white size-[20px]"
                                }
                                Hint {
                                    css: "text-[15px] font-english font-medium",
                                    {mods.read().authors.join(" ")},
                                }
                            }
                            div {
                                class: "flex w-fit justify-start gap-[7px] items-center",
                                div {
                                    class: "bg-white size-[20px]"
                                }
                                Hint {
                                    css: "text-[15px] font-english font-medium",
                                    {mods.read().get_formatted_download_count()},
                                }
                            }
                            div {
                                class: "flex w-fit justify-start gap-[7px] items-center",
                                div {
                                    class: "bg-white size-[20px]"
                                }
                                Hint {
                                    css: "text-[15px] font-english font-medium",
                                    {mods.read().last_updated.date_naive().format("%Y.%m.%d").to_string()},
                                }
                            }
                            div {
                                class: "flex w-fit justify-start gap-[7px] items-center",
                                div {
                                    class: "bg-white size-[20px]"
                                }
                                Hint {
                                    css: "text-[15px] font-english font-medium",
                                    {size.unwrap_or_default()},
                                }
                            }
                        }
                        RenderTranslatedMarkdown {
                            html: description
                        }
                    }
                }
                div {
                    flex_basis: "10%",
                }
            }
        }
    }
}
