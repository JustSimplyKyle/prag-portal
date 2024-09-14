use dioxus::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Switch(
    clicked: Signal<bool>,
    onclick: Option<EventHandler>,
    #[props(default = String::new())] class: String,
) -> Element {
    rsx! {
        button {
            class: tw_merge!("group w-[80px] p-[10px] rounded-[10px] flex justify-start items-center bg-background", class),
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
                class: "transition-all ease-in-out w-[25px] h-[40px] group-aria-selected:bg-white rounded-[10px] bg-secondary-surface"
            }
        }
    }
}
