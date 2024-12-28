use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use rust_lib::api::{
    backend_exclusive::vanilla::version::{VersionMetadata, VersionType},
    shared_resources::{
        collection::{use_collections_radio, AdvancedOptions, Memory, ModLoader, ModLoaderType},
        entry,
    },
};

use crate::{
    get_random_collection_picture,
    svgs::{
        self, ARROW_DOWN, CLOSE_CROSS, CREATE_COLLECTION, FOLDER_UPLOAD, LINE, SHADOW_ADD,
        UPLOAD_FILE,
    },
    BaseComponents::{
        atoms::{
            center::Center,
            switch::{self, FloatingSwitch},
        },
        molecules::{file_input::FileInput, foldables::Foldable},
        organisms::modal::Modal,
    },
    SnafuToCapturedError,
};
#[component]
fn Title(title: String) -> Element {
    rsx!(
        div {
            class: "text-[20px] font-medium trim",
            {title}
        }
    )
}

#[component]
fn Header() -> Element {
    let state = use_signal(|| switch::State::Left);
    rsx! {
        div {
            class: "grid grid-flow-col p-[20px] bg-background justify-stretch items-center gap-[25px]",
            div {
                class: "justify-self-start flex items-center gap-[25px]",
                div {
                    class: "inline-flex justify-center items-center size-[80px] bg-white rounded-[20px]",
                    SHADOW_ADD {


                    }
                }
                div {
                    class: "flex flex-col gap-[15px] justify-center",
                    div {
                        class: "text-[30px] font-bold trim",
                        "建立合集"
                    }
                    div {
                        class: "text-[15px] font-normal text-secondary-text trim",
                        "從頭開始建立你的合集"
                    }
                }
            }
            FloatingSwitch {
                class: "justify-self-end h-[60px] bg-deep-background",
                lhs_width: 80.,
                lhs: rsx! {
                    CREATE_COLLECTION {
                        size: svgs::Size::Medium,
                    }
                },
                rhs_width: 60.,
                rhs: rsx! {
                    FOLDER_UPLOAD {

                    }
                },
                floater: "bg-secondary-surface",
                state,
            }
        }
    }
}

#[component]
fn PicturePicker(cover_img: Signal<PathBuf>, background_img: Signal<PathBuf>) -> Element {
    let button  = "inline-flex items-center justify-center bg-background min-w-full max-w-full p-[10px] rounded-[20px]";
    let cover_img_filename = use_signal(|| None);
    let background_img_filename = use_signal(|| None);
    use_effect(move || {
        if let Some(name) = cover_img_filename() {
            cover_img.set(PathBuf::from(name));
        }
    });
    use_effect(move || {
        if let Some(name) = background_img_filename() {
            background_img.set(PathBuf::from(name));
        }
    });
    rsx! {
        div {
            class: "flex flex-col gap-[20px]",
            Title {
                title: "封面與背景圖片",
            }
            div {
                class: "flex gap-[20px] justify-center",
                div {
                    class: "flex gap-[5px]",
                    div {
                        class: "grow border-[2px] border-surface size-[140px] aspect-square rounded-[20px]",
                        background: "url(\'{cover_img.read().to_string_lossy()}\') lightgray 50% / cover no-repeat",
                    }
                    div {
                        class: "flex flex-col gap-[5px] justify-center",
                        width: "95px",
                        FileInput {
                            filename: cover_img_filename,
                            class: button,
                            height: "64.5px",
                            UPLOAD_FILE {


                            }
                        }
                        div {
                            class: button,
                            height: "64.5px",
                            CLOSE_CROSS {
                                class: "[&_*]:fill-red",
                            }
                        }
                    }
                }
                div {
                    class: "flex gap-[5px]",
                    div {
                        class: "border-[2px] border-surface h-[140px] rounded-[20px]",
                        width: "280px",
                        background: "url(\'{background_img.read().to_string_lossy()}\') lightgray 50% / cover no-repeat",
                        aspect_ratio: "2/1",
                    }
                    div {
                        class: "flex flex-col gap-[5px]",
                        width: "95px",
                        FileInput {
                            filename: background_img_filename,
                            class: button,
                            height: "64.5px",
                            UPLOAD_FILE {


                            }
                        }
                        div {
                            class: button,
                            height: "64.5px",
                            CLOSE_CROSS {
                                class: "[&_*]:fill-red",
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SetupName(mut title: Signal<Option<String>>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-[20px]",
            Title {
                title: "合集名稱",
            }
            div {
                class: "flex gap-[5px]",
                input {
                    aria_selected: title.read().is_some(),
                    class: "bg-background font-medium aria-selected:text-white text-[18px] text-secondary-surface rounded-[20px] w-full px-[20px]",
                    oninput: move |x| {
                        title.set(Some(x.value()));
                    },
                    value: title().unwrap_or_else(|| "新的合集".into()),
                }
                button {
                    class: "p-[10px] bg-background rounded-[20px] w-[60px] inline-flex justify-center items-center",
                    onclick: move |_| {
                        title.set(None);
                    },
                    CLOSE_CROSS {

                    }
                }
            }
        }
    }
}

#[component]
fn DropDown(default_ele: Element, children: Element, selector_visibility: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "pl-[20px] pr-[15px] bg-background w-full grow grid grid-flow-col justify-stretch items-center rounded-[20px] relative z-50",
            onclick: move |_| {
                selector_visibility.toggle();
            },
            div {
                class: "justify-self-start grow trim text-[18px] font-english",
                {default_ele}
            }
            ARROW_DOWN {
                class: "justify-self-end",
            }
            div {
                aria_hidden: !selector_visibility(),
                onclick: move |x| {
                    x.stop_propagation();
                },
                class: "absolute inset-x-0 top-full flex flex-col bg-background rounded-[20px] *:py-[15px] *:pl-[25px] *:pr-[20px] gap-[5px] h-fit max-h-[300px] mt-[10px] overflow-y-scroll aria-hidden:opacity-0 aria-hidden:hidden z-50 py-[15px]",
                transition: "all 0.5s allow-discrete",
                {children}
            }
        }
    }
}

#[component]
pub fn GameVersion(selected_version: Signal<Option<VersionMetadata>>) -> Element {
    let latest_version = use_resource(VersionMetadata::latest_release);
    let binding = latest_version.read();
    let latest_version = binding
        .as_ref()
        .map(|x| x.as_ref())
        .transpose()
        .map_err(SnafuToCapturedError::to_render_error)?;

    let mut snapshot_status = use_signal(|| false);
    let mut selector_visibility = use_signal(|| false);

    let game_versions = use_resource(move || async move {
        VersionMetadata::get_version_manifest()
            .await
            .map(|x| x.versions)
    });

    let build_versions = |metadata: VersionMetadata| {
        let aria_selected = selected_version.read().as_ref().map_or_else(
            || Some(&metadata) == latest_version,
            |x| x.id == metadata.id,
        );

        rsx! {
            div {
                onclick: move |_| {
                    selected_version.set(Some(metadata.clone()));
                    selector_visibility.set(false);
                },
                div {
                    aria_selected,
                    class: "font-display text-[20px] trim font-normal text-hint aria-selected:text-white",
                    {metadata.id.clone()}
                }
            }
        }
    };

    let read = game_versions.read();

    let game_versions = read
        .as_ref()
        .map(|x| x.as_deref())
        .transpose()
        .map_err(SnafuToCapturedError::to_render_error)?;

    let all_game_versions = game_versions
        .into_iter()
        .flat_map(|x| x.iter())
        .cloned()
        .map(build_versions);

    let release_game_versions = game_versions
        .into_iter()
        .flat_map(|x| x.iter().filter(|x| x.is_release()))
        .cloned()
        .map(build_versions);

    rsx! {
        div {
            class: "flex flex-col gap-[20px] z-50",
            Title {
                title: "遊戲版本",
            }
            div {
                class: "flex gap-[5px] h-[60px] z-50",
                DropDown {
                    default_ele: rsx! {
                        if let Some(version) = &*selected_version.read() {
                            {version.id.clone()}
                        } else {
                            if let Some(v) = latest_version {
                                {v.id.clone()}
                            }
                        }
                    },
                    selector_visibility,
                    if snapshot_status() {
                        {all_game_versions}
                    } else {
                        {release_game_versions}
                    }
                }
                div {
                    class: "bg-background min-w-[220px] max-w-[220px] grid grid-flow-col justify-stretch items-center gap-[10px] pl-[20px] pr-[15px] rounded-[20px]",
                    button {
                        "data-selected": snapshot_status(),
                        class: "justify-self-start grow w-full font-display text-[18px] font-normal text-hint data-[selected=true]:text-white transition-all",
                        onclick: move |_| {
                            snapshot_status.toggle();
                        },
                        "顯示快照版本"
                    }
                    CLOSE_CROSS {
                        class: "justify-self-end inline-flex justify-center items-center",
                    }
                }
            }
        }
    }
}

const DEFAULT_MEMORY: usize = 8;

#[component]
fn MemorySelector(memory_selected: Signal<usize>) -> Element {
    let memory_selector = [1, 2, 4, 8, 16, 32];

    rsx! {
        div {
            class: "mt-[35px] flex flex-col gap-[20px]",
            Title {
                title: "分配記憶體",
            }
            div {
                class: "h-[60px] flex gap-[5px] *:bg-background *:rounded-[20px]",
                div {
                    class: "aspect-square p-[10px]",
                    onclick: move |_| {
                        memory_selected.set(DEFAULT_MEMORY);
                    }
                }
                for i in memory_selector {
                    div {
                        class: "w-fit px-[20px] flex items-center aria-selected:bg-white [&_*]:pointer-events-none",
                        aria_selected: memory_selected() == i,
                        onclick: move |_| {
                            memory_selected.set(i);
                        },
                        div {
                            class: "flex items-end gap-[3px] font-english",
                            div {
                                class: "text-[20px] trim text-white aria-selected:text-black font-bold",
                                aria_selected: memory_selected() == i,
                                "{i}"
                            }
                            div {
                                class: "text-[15px] text-hint trim font-normal",
                                "GB"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AdvancedOption(memory_selected: Signal<usize>) -> Element {
    let enabled = use_signal(|| false);

    rsx! {
        Foldable {
            enabled,
            title: rsx! {
                div {
                    class: "w-full flex items-center gap-[15px]",
                    div {
                        class: "text-[18px] font-normal min-w-fit text-white group-data-[enabled=false]:text-hint",
                        "進階選項"
                    }
                    LINE {
                        class: "w-full grow [&_*]:stroke-background group-data-[enabled=true]:[&_*]:stroke-secondary-surface",
                        stroke_width: "3px",
                    }
                    div {
                        class: "w-fit grow-0",
                        "進"
                    }
                }
            },
            MemorySelector {
                memory_selected,
            }
        }
    }
}

#[component]
pub fn ModLoaderSelector(modloader_selected: Signal<Option<ModLoader>>) -> Element {
    let mut selector_visibility = use_signal(|| false);
    let mut selected_modloader_type = use_signal(|| None);
    let modloaders = [
        ModLoaderType::NeoForge,
        ModLoaderType::Forge,
        ModLoaderType::Fabric,
        ModLoaderType::Quilt,
    ];

    let aria_selector =
        |x| selected_modloader_type().map_or_else(|| modloaders[0] == x, |v| v == x);

    rsx! {

        div {
            class: "flex flex-col gap-[20px] z-40",
            Title {
                title: "模組載入器",
            }
            div {
                class: "flex gap-[5px] h-[60px] z-40",
                DropDown {
                    default_ele: rsx! {
                        {selected_modloader_type().unwrap_or_else(|| modloaders[0]).to_string()}
                    },
                    selector_visibility,
                    for loader in modloaders {
                        div {
                            onclick: move |_| {
                                selected_modloader_type.set(Some(loader));
                                selector_visibility.set(false);
                            },
                            aria_selected: aria_selector(loader),
                            class: "font-display text-[20px] trim font-normal text-hint aria-selected:text-white",
                            "{loader}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Footer(canceled: Signal<bool>, finished: Signal<bool>) -> Element {
    const BLOCK_CSS: &str = "grid grid-flow-col justify-stretch items-center gap-[10px] bg-deep-background h-full p-[15px] pr-[25px] rounded-[15px]";
    rsx! {
        div {
            class: "bg-background h-[100px] p-[20px] flex justify-end items-center gap-[10px] z-0",
            button {
                class: "{BLOCK_CSS} w-[150px]",
                onclick: move |_| {
                    canceled.set(true);
                },
                div {
                    class: "justify-self-start pointer-events-none",
                    "C"
                }
                div {
                    class: "justify-self-end trim text-[20px] pointer-events-none",
                    "取消"
                }
            }
            button {
                class: "{BLOCK_CSS} w-[180px]",
                onclick: move |_| {
                    finished.set(true);
                },
                div {
                    class: "justify-self-start pointer-events-none",
                    "C"
                }
                div {
                    class: "justify-self-end trim text-[20px] pointer-events-none",
                    "完成"
                }
            }
        }
    }
}

#[component]
pub fn BuildCollection(active: Signal<bool>) -> Element {
    let title = use_signal(|| None);
    let cover_img = use_signal(get_random_collection_picture);
    let background_img = use_signal(get_random_collection_picture);
    let selected_version = use_signal(|| None);
    let modloader_selected = use_signal(|| None);
    let memory_selected = use_signal(|| DEFAULT_MEMORY);

    let canceled = use_signal(|| false);
    let finished = use_signal(|| false);

    use_effect(move || {
        if canceled() {
            active.set(false);
        }
    });

    let collections_radio = use_collections_radio();

    use_effect(move || {
        let finished = finished();
        let version = selected_version();
        let collections_radio = collections_radio;
        spawn(async move {
            if finished {
                active.set(false);

                let version = match version {
                    Some(v) => v,
                    None => match VersionMetadata::latest_release().await {
                        Ok(v) => v,
                        Err(err) => {
                            throw_error(err);
                            return;
                        }
                    },
                };

                if let Err(err) = entry::create_collection(
                    title().unwrap_or_else(|| String::from("新的收藏")),
                    cover_img(),
                    version,
                    ModLoader {
                        mod_loader_type: ModLoaderType::Fabric,
                        version: None,
                    },
                    AdvancedOptions {
                        jvm_max_memory: Some(Memory::Gigabytes(memory_selected())),
                        java_arguments: String::new(),
                    },
                    collections_radio,
                )
                .await
                {
                    throw_error(err);
                }
                info!("Finished collection creation");
                // active.set(false);
            }
        });
    });

    rsx! {
        Modal {
            active,
            div {
                class: "flex min-w-[700px] w-full",
                Center {
                    percentage_center_bias: 50.,
                    class: "flex flex-col border-2 border-surface rounded-[20px] overflow-visible",
                    box_shadow: "10px 10px 30px 0px rgba(0, 0, 0, 0.25)",
                    Header {


                    }
                    div {
                        class: "flex flex-col bg-deep-background p-[30px] gap-[35px] z-50",
                        SetupName {
                            title,
                        }
                        PicturePicker {
                            cover_img,
                            background_img,
                        }
                        div {
                            class: "z-50 container",
                            GameVersion {
                                selected_version,
                            }
                        }
                        div {
                            class: "z-30 container",
                            ModLoaderSelector {
                                modloader_selected
                            }
                        }
                        div {
                            class: "z-20 container",
                            AdvancedOption {
                                memory_selected
                            }
                        }
                    }
                    Footer {
                        canceled,
                        finished,
                    }
                }
            }
        }
    }
}
