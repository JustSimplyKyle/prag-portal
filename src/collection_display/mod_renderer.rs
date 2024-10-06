mod details;

use std::ops::Deref;

use dioxus::{prelude::*, CapturedError};
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
            switch::Switch,
        },
        molecules::search_bar::fuzzy_search,
        string_placements::ContentType,
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
    let mods = use_memo(move || {
        let value = default.clone();
        let collection = collection_id().get_collection();
        let binding = collection.read();
        let mods = binding
            .mod_controller()
            .into_iter()
            .flat_map(move |x| x.manager.mods.clone().into_iter());
        fuzzy_search(&search.read(), &value, mods, |x| &x.name).collect::<Vec<_>>()
    });
    rsx! {
        div {
            class: "bg-background flex flex-col gap-[20px] rounded-t-[30px] pb-[30px] h-full overflow-x-hidden",
            GridRow {
                class: "w-full border-b-[3px] border-b-secondary-surface rounded-t-[30px] h-[70px] px-[50px] py-[10px] backdrop-blur-[7.5px] sticky top-0 z-[2000]",
                background: "rgba(25, 25, 25, 0.90)",
                items: [
                    rsx!(
                        div {
                            class: "flex-none inline-flex justify-center w-[80px] text-white text-lg h-full trim",
                            "圖示"
                        }
                    ),
                    rsx!(
                        div {
                            class: "text-white text-lg h-full trim",
                            "名稱（來源／文件名稱）"
                        }
                    ),
                    rsx!(
                        div {
                            class: "text-white text-lg h-full trim",
                            "作者"
                        }
                    ),
                    rsx!(
                        div {
                            class: "text-white text-lg h-full trim",
                            "更新"
                        }
                    ),
                    rsx!(
                        div {
                            class: "text-white text-lg h-full trim",
                            "刪除"
                        }
                    ),
                    rsx!(
                        div {
                            class: "text-white text-lg h-full trim",
                            "更多"
                        }
                    ),
                    rsx!(
                        div {
                            class: "text-white text-lg h-full trim",
                            "狀態"
                        }
                    ),
                ]
            }

            div {
                class: "bg-background w-full h-full flex flex-col px-[30px]",
                div {
                    class: "flex flex-col gap-[5px]",
                    for ele in mods() {
                        SubModViewer {
                            collection_id,
                            mods: ele
                        }
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
            let binding = || async move {
                let Some(mut controller) = collection.peek().mod_controller.clone() else {
                    return Ok(());
                };
                let manager = &mut controller.manager;
                #[allow(clippy::unwrap_used)]
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
            if let Err(x) = binding().await {
                error_handler.set(Err(x));
            }
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
    use_effect(move || {
        println!("{}", dialog());
    });
    let name = rsx!(
        div {
            class: "flex gap-[7px]",
            div {
                class: "text-white text-[28px] font-bold font-english trim",
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
            div {
                class: "font-medium text-secondary-text text-[15px] font-english trim",
                {version.clone()}
            }
        }
    );
    let author = rsx!(
        div {
            class: "text-[15px] text-secondary-text font-english trim",
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
        details::ModDetails {
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
