use dioxus::prelude::*;

use rust_lib::api::backend_exclusive::download::Progress;
use rust_lib::api::shared_resources::collection::CollectionId;

use crate::impl_context_switcher;
use crate::BaseComponents::{
    atoms::button::{Button, FillMode, Roundness},
    molecules::switcher::{StateSwitcherSelectionBar, ToClass},
    string_placements::{ContentType, Hint, Image, StringPlacements, Text},
};
use crate::DRAG_INDICATOR;
use rust_lib::api::shared_resources::entry::DOWNLOAD_PROGRESS;
use strum::EnumIter;

#[component]
fn ListItem(
    collection_id: ReadOnlySignal<CollectionId>,
    progress: ReadOnlySignal<Progress>,
) -> Element {
    let collection = collection_id().get_collection();
    let progress = progress.read();
    rsx! {
        Button {
            roundness: Roundness::Pill,
            string_placements: rsx! {
                div { class: "justify-self-center w-full flex gap-[15px]",
                    {ContentType::svg(DRAG_INDICATOR).css("self-center svg-[30px]")}
                    div { class: "w-full flex gap-[20px]",
                        Image { css: "bg-cover bg-white w-[80px] h-[80px] rounded-[10px]",
                            {collection.read().picture_path().to_string_lossy().to_string()}
                        }
                        div { class: "w-full flex flex-col justify-start gap-[10px]",
                            Text { css: "text-[25px] fond-bold", {collection.read().display_name().clone()} }
                            div { class: "flex gap-[4px]",
                                Hint { css: "text-base font-semibold",
                                    {format!("{} / {} |", progress.current_size.unwrap_or_default().display_size_from_megabytes(), progress.total_size.unwrap_or_default().display_size_from_megabytes())}
                                }
                                Text { css: "text-base font-semibold",
                                    "{progress.speed.unwrap_or_default().display_size_from_megabytes()}"
                                }
                            }
                            div { class: "w-full h-full flex items-end",
                                div { class: "rounded-[50px] w-full h-[7px] bg-zinc-800",
                                    div {
                                        class: "transition-all rounded-[50px] bg-white h-[7px]",
                                        width: format!("{}%", progress.percentages)
                                    }
                                }
                            }
                        }
                    }
                }
            },
            extended_css_class: "rounded-[5px]",
            fill_mode: FillMode::Fill
        }
    }
}

#[component]
fn FirstProgressView(
    collection_id: ReadOnlySignal<CollectionId>,
    progress: ReadOnlySignal<Progress>,
) -> Element {
    let collection = collection_id().get_collection();
    let progress = progress.read();
    rsx! {
        div {
            class: "w-full h-[350px] p-[30px] rounded-[20px]",
            background: format!(
                "linear-gradient(88deg, #0E0E0E 14.88%, rgba(14, 14, 14, 0.70) 100%), url('{}') lightgray 50% / cover no-repeat",
                collection.read().picture_path().to_string_lossy().to_string(),
            ),
            div {
                class: "w-full grid grid-flow-col",
                div {
                    class: "justify-self-start flex flex-col gap-[20px]",
                    Text {
                        css: "text-[60px] font-black text-white",
                        {collection.read().display_name().clone()}
                    }
                    Hint {
                        css: "font-medium",
                        {
                            format!("總計 {}/已下載 {}",
                                progress.total_size.unwrap_or_default().display_size_from_megabytes(),
                                progress.current_size.unwrap_or_default().display_size_from_megabytes()
                            )
                        }
                    }
                }
                div {
                    class: "justify-self-end flex",
                    Text {
                        css: "text-[50px] font-bold text-white",
                        "{progress.speed.unwrap_or_default().display_size_from_megabytes()}"
                    }
                    Hint {
                        css: "text-[50px] font-bold",
                        "/s"
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, EnumIter, Debug)]
pub enum ProgressState {
    Running,
    Finished,
}

impl From<ProgressState> for StringPlacements {
    fn from(value: ProgressState) -> Self {
        match value {
            ProgressState::Running => vec![
                ContentType::text("K").align_left(),
                ContentType::text("正在進行").align_right(),
            ],
            ProgressState::Finished => vec![
                ContentType::text("K").align_left(),
                ContentType::text("已完成").align_right(),
            ],
        }
        .into()
    }
}

impl ToClass for ProgressState {}

impl_context_switcher!(ProgressState);

#[component]
fn ProgressStateBar() -> Element {
    rsx! {
        div {
            class: "w-full grid grid-flow-col justify-stretch",
            StateSwitcherSelectionBar {
                default_state: ProgressState::Running
            }
            div {
                class: "justify-self-end",
                "go fuck yourself"
            }
        }
    }
}

#[component]
pub fn DownloadProgress() -> Element {
    let mut progress = DOWNLOAD_PROGRESS()
        .0
        .into_iter()
        .filter(|(_, x)| x.percentages < 100.)
        .map(|(id, progress)| (id.collection_id, progress))
        .peekable();
    rsx! {
        div {
            class: "flex flex-col gap-[20px]",
            if let Some((collection_id, progress)) = progress.peek().cloned() {
                FirstProgressView {
                    collection_id,
                    progress
                }
                ProgressStateBar {

                }
            }
            for (collection_id , progress) in progress {
                ListItem {
                    collection_id,
                    progress
                }
            }
        }
    }
}

pub(crate) trait SizeFromMegaBytes {
    fn display_size_from_megabytes(&self) -> String;
}

impl SizeFromMegaBytes for f64 {
    fn display_size_from_megabytes(&self) -> String {
        match *self {
            f if f < 1_000. => format!("{f:.2} bytes"),
            f if f < 1_000_000. => format!("{:.2} KB", f / 1_000.),
            f if f < 1_000_000_000. => format!("{:.2} MB", f / 1_000_000.),
            f => format!("{:.2} GB", f / 1_000_000_000.),
        }
    }
}
