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
    use dioxus::prelude::*;

    use crate::BaseComponents::organisms::modal::Modal;
    #[component]
    fn Title(title: String) -> Element {
        rsx!(div {
            class: "text-[20px] font-medium trim",
            {title}
        })
    }

    #[component]
    fn Header() -> Element {
        rsx! {
            div {
                class: "grid grid-flow-col p-[20px] bg-background justify-stretch items-center gap-[25px]",
                div {
                    class: "justify-self-start flex items-center gap-[25px]",
                    div {
                        class: "bg-white size-[80px] rounded-[20px]",
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
                div {
                    class: "justify-self-end bg-red w-[140px]",
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
    fn PicturePicker() -> Element {
        let button  = "inline-flex items-center justify-center bg-background w-full h-full p-[10px] rounded-[20px]";
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
                            class: "bg-white grow min-w-[140px] size-full aspect-square rounded-[20px]",
                        }
                        div {
                            class: "flex flex-col grow w-full gap-[5px]",
                            div {
                                class: button,
                                div {
                                    class: "bg-red size-[30px]"
                                }
                            }
                            div {
                                class: button,
                                div {
                                    class: "bg-red size-[30px]"
                                }
                            }
                        }
                    }
                    div {
                        class: "flex gap-[5px] grow",
                        div {
                            class: "bg-white grow min-h-[140px] h-full rounded-[20px]",
                            aspect_ratio: "2/1",
                        }
                        div {
                            class: "flex flex-col grow w-full gap-[5px]",
                            div {
                                class: button,
                                div {
                                    class: "bg-red size-[30px]"
                                }
                            }
                            div {
                                class: button,
                                div {
                                    class: "bg-red size-[30px]"
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
                        div {
                            class: "bg-white size-[30px]",
                        }
                    }
                }
            }
        }
    }

    #[component]
    pub fn BuildCollection(active: Signal<bool>) -> Element {
        let title = use_signal(|| None);

        rsx! {
            Modal {
                active,
                div {
                    class: "flex w-full border-2 border-surface rounded-[20px]",
                    box_shadow: "10px 10px 30px 0px rgba(0, 0, 0, 0.25)",
                    Center {
                        class: "flex flex-col",
                        Header {}
                        div {
                            class: "flex flex-col bg-deep-background p-[30px] gap-[35px]",
                            SetupName { title }
                            PicturePicker {  }
                            Title {
                                title: "遊戲版本"
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
