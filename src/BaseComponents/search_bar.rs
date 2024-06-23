pub use dioxus::prelude::*;

use crate::{
    collections::SEARCH,
    main_page::ARROW_LEFT,
    BaseComponents::{
        button::{Button, FillMode, Roundness},
        string_placements::ContentType,
    },
};

#[component]
pub fn SearchBar(sender: Option<Signal<String>>) -> Element {
    let mut total_clicked = use_signal(|| false);
    let mut input_clicked = use_signal(|| false);
    let mut search = use_signal(|| String::from("搜尋"));
    use_effect(move || {
        if !total_clicked() {
            search.set(String::from("搜尋"));
        }
        if input_clicked() {
            search.set(String::new());
        }
    });
    use_effect(move || {
        if let Some(mut sender) = sender {
            sender.set(search());
        }
    });
    rsx! {
        Button {
            roundness: Roundness::Pill,
            string_placements: rsx! {
                div { class: "relative text-stone-950/20 ", resize: false,
                    input {
                        r#type: "text",
                        id: "test",
                        class: "aboslute overflow-x-scroll w-full grow-0 inset-0 hidden group-aria-selected:block align-middle border-0 overflow-x-clip",
                        value: search(),
                        oninput: move |event| {
                            search.set(event.value());
                        },
                        onclick: move |event| {
                            if !input_clicked() {
                                input_clicked.set(true);
                            }
                            event.stop_propagation();
                        }
                    }
                }
                div { class: "flex flex-row-reverse items-baseline",
                    {ContentType::svg(ARROW_LEFT).css("svg-[20px]")},
                    {ContentType::svg(SEARCH).css("svg-[30px]")}
                }
            },
            onclick: move |()| {
                input_clicked.set(false);
                total_clicked.toggle();
            },
            focus_color_change: true,
            fill_mode: FillMode::Fit,
            extended_css_class: "group transition-all w-20 grid grid-flow-col justify-stretch content-center [&_*]:transition-all h-[55px] aria-selected:w-[300px] aria-selected:bg-white pl-[15px] pr-[10px]"
        }
    }
}