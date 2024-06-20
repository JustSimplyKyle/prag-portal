use dioxus::prelude::*;

#[component]
pub fn Switch(clicked: Signal<bool>, onclick: Option<EventHandler>) -> Element {
    rsx! {
        button {
            class: "group w-[45px] p-[5px] rounded-full flex justify-start items-center bg-background",
            "aria-selected": clicked(),
            onclick: move |_| {
                clicked.toggle();
                if let Some(onclick) = onclick {
                    onclick(());
                }
            },
            div {
                class: "transition-all duration-700 ease-linear flex-none group-aria-selected:flex-auto"
            }
            div {
                class: "transition-all ease-in-out w-[20px] h-[20px] group-aria-selected:border-4 group-aria-selected:border-green group-aria-selected:bg-none rounded-full bg-secondary-surface"
            }
        }
    }
}
