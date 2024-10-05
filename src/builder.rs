use std::path::PathBuf;

use dioxus_logger::tracing::info;
use rust_lib::api::{
    backend_exclusive::{errors::ManifestProcessingError, vanilla::version::VersionMetadata},
    shared_resources::{
        collection::{CollectionError, ModLoader, ModLoaderType},
        entry,
    },
};
use snafu::prelude::*;

use crate::get_random_collection_picture;

#[derive(Snafu, Debug)]
pub enum CollectionBuilderError {
    #[snafu(display("Invalid version id {id}"))]
    InvalidVersionId { id: String },
    #[snafu(display("Failed to parse version id {id}"))]
    VersionIdParsing {
        id: String,
        source: ManifestProcessingError,
    },
    #[snafu(transparent)]
    CollectionError { source: CollectionError },
}

/// # Errors
///
/// This function will return an error if:
///
/// - The provided `version_id` is not a valid Minecraft version ID.
/// - Manifest of `version_id` failed to parse
/// - An error occurs while creating the collection entry.
/// - An error occurs while adding mods from Modrinth.
/// - An error occurs while downloading the mods.
pub async fn collection_builder(
    picture_path: impl Into<Option<PathBuf>> + Send,
    version_id: impl Into<String> + Send,
) -> Result<(), CollectionBuilderError> {
    let version_id = version_id.into();
    let version = VersionMetadata::from_id(&version_id)
        .await
        .context(VersionIdParsingSnafu { id: &version_id })?
        .context(InvalidVersionIdSnafu { id: &version_id })?;

    let mut collection = entry::create_collection(
        "新的收藏",
        picture_path
            .into()
            .unwrap_or_else(|| get_random_collection_picture().into()),
        version,
        ModLoader::new(ModLoaderType::Fabric, None),
        None,
    )
    .await?;
    info!("Adding mods...");
    collection
        .add_multiple_modrinth_mod(
            vec![
                "fabric-api",
                "sodium",
                "modmenu",
                "ferrite-core",
                "lazydfu",
                "create-fabric",
                "iris",
                "indium",
            ],
            vec![],
            None,
        )
        .await?;
    collection.download_mods().await?;
    info!("Finished downloading mods");
    Ok(())
}

pub mod component {
    use std::path::PathBuf;

    use dioxus::prelude::*;
    use rust_lib::api::backend_exclusive::{
        errors::ManifestProcessingError, vanilla::version::VersionMetadata,
    };

    use crate::{
        get_random_collection_picture,
        svgs::{
            self, ARROW_DOWN, CLOSE_CROSS, CREATE_COLLECTION, FOLDER_UPLOAD, SHADOW_ADD,
            UPLOAD_FILE,
        },
        BaseComponents::{
            atoms::switch::{self, FloatingSwitch},
            organisms::modal::Modal,
        },
        ErrorFormatted, ToRenderError,
    };
    #[component]
    fn Title(title: String) -> Element {
        rsx!(div {
            class: "text-[20px] font-medium trim",
            {title}
        })
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
                        SHADOW_ADD {}
                    }
                    div {
                        class: "flex flex-col gap-[15px] justify-center",
                        div {
                            class: "text-[30px] font-bold trim",
                            "建立合集",
                        }
                        div {
                            class: "text-[15px] font-normal text-secondary-text trim",
                            "從頭開始建立你的合集",
                        }
                    }
                }
                FloatingSwitch {
                    class: "justify-self-end h-[60px] bg-deep-background",
                    lhs_width: 80.,
                    lhs: rsx! { CREATE_COLLECTION { size: svgs::Size::Medium } },
                    rhs_width: 60.,
                    rhs: rsx! { FOLDER_UPLOAD {} },
                    floater: "bg-secondary-surface",
                    state
                }
            }
        }
    }

    #[component]
    fn Center(
        children: Element,
        #[props(extends = GlobalAttributes, extends = div)] mut attributes: Vec<Attribute>,
    ) -> Element {
        rsx! {
            div {
                flex_basis: "33.3%",
            }
            div {
                flex_basis: "33.3%",
                ..attributes,
                {children}
            }
            div {
                flex_basis: "33.3%",
            }
        }
    }

    #[component]
    fn PicturePicker(cover_img: Signal<PathBuf>, background_img: Signal<PathBuf>) -> Element {
        let button  = "inline-flex items-center justify-center bg-background min-w-full max-w-full h-full p-[10px] rounded-[20px]";
        rsx! {
            div {
                class: "flex flex-col gap-[20px]",
                Title {
                    title: "封面與背景圖片"
                }
                div {
                    class: "flex gap-[20px]",
                    div {
                        class: "flex gap-[5px] grow",
                        div {
                            class: "grow border-[2px] border-surface min-w-[140px] size-full aspect-square rounded-[20px]",
                            background: "url(\'{cover_img.read().to_string_lossy()}\') lightgray 50% / cover no-repeat"
                        }
                        div {
                            class: "flex flex-col grow w-full gap-[5px]",
                            div {
                                class: button,
                                UPLOAD_FILE {}
                            }
                            div {
                                class: button,
                                CLOSE_CROSS {
                                    class: "[&_*]:fill-red"
                                }
                            }
                        }
                    }
                    div {
                        class: "flex gap-[5px] grow",
                        div {
                            class: "border-[2px] border-surface grow min-h-[140px] h-full rounded-[20px]",
                            background: "url(\'{background_img.read().to_string_lossy()}\') lightgray 50% / cover no-repeat",
                            aspect_ratio: "2/1",
                        }
                        div {
                            class: "flex flex-col grow w-full gap-[5px]",
                            div {
                                class: button,
                                UPLOAD_FILE {}
                            }
                            div {
                                class: button,
                                CLOSE_CROSS {
                                    class: "[&_*]:fill-red"
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
                    title: "合集名稱"
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
                        CLOSE_CROSS {}
                    }
                }
            }
        }
    }

    #[component]
    pub fn GameVersion(
        selected_version: Resource<Result<VersionMetadata, ManifestProcessingError>>,
    ) -> Element {
        let binding = selected_version.read();
        let current_version = match &*binding {
            Some(Ok(x)) => x,
            Some(Err(err)) => {
                return Err(err.to_formatted().to_render_error());
            }
            None => {
                return VNode::empty();
            }
        };

        rsx! {
            div {
                class: "flex flex-col gap-[20px]",
                Title {
                    title: "遊戲版本"
                }
                div {
                    class: "flex gap-[5px] h-[60px]",
                    div {
                        class: "pl-[20px] pr-[15px] bg-background w-full grow grid grid-flow-col justify-stretch items-center rounded-[20px]",
                        div {
                            class: "justify-self-start grow trim text-[18px] font-english",
                            {current_version.id.clone()}
                        }
                        ARROW_DOWN {
                            class: "justify-self-end"
                        }
                    }
                    div {
                        class: "bg-background min-w-[220px] max-w-[220px] grid grid-flow-col justify-stretch items-center gap-[10px] pl-[20px] pr-[15px] rounded-[20px]",
                        div {
                            class: "justify-self-start grow w-full font-display text-[18px] inline-flex items-center font-normal text-secondary trim",
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

    #[component]
    pub fn BuildCollection(active: Signal<bool>) -> Element {
        let title = use_signal(|| None);
        let cover_img = use_signal(|| get_random_collection_picture().into());
        let background_img = use_signal(|| get_random_collection_picture().into());

        let selected_version = use_resource(VersionMetadata::latest_release);

        rsx! {
            Modal {
                active,
                div {
                    class: "flex min-w-[700px] w-full border-2 border-surface rounded-[20px]",
                    box_shadow: "10px 10px 30px 0px rgba(0, 0, 0, 0.25)",
                    Center {
                        class: "flex flex-col",
                        Header {}
                        div {
                            class: "flex flex-col bg-deep-background p-[30px] gap-[35px]",
                            SetupName { title }
                            PicturePicker {
                                cover_img,
                                background_img
                            }
                            GameVersion {
                                selected_version,
                            }
                            Title {
                                title: "合集名稱"
                            }
                        }
                    }
                }
            }
        }
    }
}
