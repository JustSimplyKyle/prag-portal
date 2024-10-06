use dioxus::prelude::*;
use itertools::Itertools;
use rust_lib::api::{
    backend_exclusive::mod_management::mods::{ModMetadata, SupportedSide},
    shared_resources::collection::CollectionId,
};

use crate::{
    collection_display::mod_renderer::use_active_controller,
    svgs::{CURSEFORGE_OUTLINE, MODRINTH_OUTLINE},
    BaseComponents::{
        atoms::{
            center::Center,
            markdown_to_html,
            switch::{self, FloatingSwitch, Switch},
        },
        organisms::{markdown_renderer::RenderTranslatedMarkdown, modal::Modal},
    },
};

#[component]
fn PlatformSwitch(platform: Signal<switch::State>) -> Element {
    rsx! {
        FloatingSwitch {
            lhs_width: 80.,
            lhs: rsx! {
                CURSEFORGE_OUTLINE {
                    class: "transition-all fill-deep-background w-[30px] group-data-[selected=Right]:fill-orange"
                }
            },
            lhs_css: "px-[20px] py-[10px]",
            rhs_width: 60.,
            rhs: rsx! {
                MODRINTH_OUTLINE {
                    class: "transition-all fill-deep-background w-[35px] group-data-[selected=Left]:w-[30px] group-data-[selected=Left]:fill-modrinth"
                }
            },
            rhs_css: "px-[20px] py-[10px]",
            floater: "bg-orange rounded-[20px] group-data-[selected=Right]:bg-modrinth",
            class: "h-[60px] bg-deep-background rounded-[20px]",
            state: platform,
        }
    }
}

#[component]
fn Authors(mods: ReadOnlySignal<ModMetadata>) -> Element {
    rsx! {
        div {
            class: "flex w-fit justify-start gap-[7px] items-center",
            div {
                class: "bg-white size-[20px]"
            }
            div {
                class: "text-[15px] font-english font-medium text-hint trim",
                {mods.read().authors.join(" ")},
            }
        }
    }
}

#[component]
fn DownloadCount(mods: ReadOnlySignal<ModMetadata>) -> Element {
    rsx! {
        div {
            class: "flex w-fit justify-start gap-[7px] items-center",
            div {
                class: "bg-white size-[20px]"
            }
            div {
                class: "text-[15px] font-english font-medium text-hint trim",
                {mods.read().get_formatted_download_count()},
            }
        }
    }
}

#[component]
fn LastUpdated(mods: ReadOnlySignal<ModMetadata>) -> Element {
    rsx! {
        div {
            class: "flex w-fit justify-start gap-[7px] items-center",
            div {
                class: "bg-white size-[20px]"
            }
            div {
                class: "text-[15px] font-english font-medium text-hint trim",
                {mods.read().last_updated.date_naive().format("%Y.%m.%d").to_string()},
            }
        }
    }
}

#[component]
fn TotalSize(mods: ReadOnlySignal<ModMetadata>) -> Element {
    let file_size =
        use_resource(move || async move { mods.read().get_formatted_accumlated_size().await });

    let size = file_size.as_ref().and_then(|x| x.as_ref().ok().cloned());
    rsx! {
        div {
            class: "flex w-fit justify-start gap-[7px] items-center",
            div {
                class: "bg-white size-[20px]"
            }
            div {
                class: "text-[15px] font-english font-medium text-hint trim",
                {size.unwrap_or_default()},
            }
        }
    }
}

#[component]
fn SupportedSideDisplay(mods: ReadOnlySignal<ModMetadata>) -> Element {
    let side = mods.read().supported_sides.as_ref().map(|sides| {
        let client = sides.contains(&SupportedSide::Client);
        let server = sides.contains(&SupportedSide::Server);
        match (client, server) {
            (true, true) => "客戶端 & 伺服端",
            (true, false) => "客戶端",
            (false, true) => "伺服端",
            (false, false) => "未知",
        }
    });

    rsx! {
        div {
            class: "flex w-fit justify-start gap-[7px] items-center",
            div {
                class: "bg-white size-[20px]"
            }
            div {
                class: "text-[15px] font-english font-medium text-hint trim",
                {side},
            }
        }
    }
}

#[component]
fn InnerModDetails(
    mods: ReadOnlySignal<ModMetadata>,
    active: Signal<bool>,
    clicked: Signal<bool>,
    collection_id: ReadOnlySignal<CollectionId>,
) -> Element {
    use_active_controller(clicked, collection_id, mods);

    let description = markdown_to_html(&mods.read().long_description);

    if description.contains("Welcome to Create") {
        println!("{description}");
        println!("{}", mods.read().long_description);
    }

    let file_paths = mods
        .read()
        .get_filepaths()
        .into_iter()
        .flatten()
        .map(|x| x.display().to_string())
        .join("\n");

    let platform = use_signal(|| switch::State::Left);

    rsx! {
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
                        div {
                            class: "text-[30px] font-english font-bold trim",
                            {mods.read().name.clone()}
                        }
                        div {
                            class: "text-secondary-text text-[15px] font-medium font-english trim",
                            {file_paths}
                        }
                    }
                }
                div {
                    class: "justify-self-end flex gap-[10px]",
                    PlatformSwitch {
                        platform
                    }
                    Switch {
                        class: "bg-deep-background",
                        clicked,
                    }
                }
            }
            div {
                class: "flex items-center h-[50px] gap-[25px] px-[20px]",
                Authors {
                    mods,
                }
                DownloadCount {
                    mods,
                }

                LastUpdated {
                    mods,
                }
                TotalSize {
                    mods,
                }
                SupportedSideDisplay {
                    mods,
                }
            }
            RenderTranslatedMarkdown {
                html: description
            }
        }
    }
}

#[component]
pub fn ModDetails(
    mods: ReadOnlySignal<ModMetadata>,
    active: Signal<bool>,
    clicked: Signal<bool>,
    collection_id: ReadOnlySignal<CollectionId>,
) -> Element {
    rsx! {
        Modal {
            active,
            div {
                class: "w-full flex justify-center",
                Center {
                    percentage_center_bias: 80.,
                    InnerModDetails {
                        mods,
                        active,
                        clicked,
                        collection_id
                    }
                }
            }
        }
    }
}
