use dioxus::prelude::*;

use crate::{use_mounted, use_scroll_size, use_visible_size};

#[component]
pub fn Foldable(mut enabled: Signal<bool>, title: Element, children: Element) -> Element {
    let mut mounted = use_mounted();
    let mut container_mounted = use_mounted();
    let title_size = use_visible_size(mounted);
    let container_size = use_scroll_size(container_mounted);
    rsx! {
        div {
            "data-enabled": enabled(),
            class: "group flex flex-col h-fit overflow-y-clip ease-slow duration-200 transition-all [&_*]:transition-all",
            onmounted: move |e| container_mounted.set(Some(e.data)),
            max_height: if enabled() {
                "{container_size().unwrap_or_default().height}px"
            } else {
                "{title_size().unwrap_or_default().height}px"

            },
            div {
                class: "container z-[1000] *:z-[1000]",
                onmounted: move |e| mounted.set(Some(e.data)),
                onclick: move |_| {
                    enabled.toggle();
                },
                {title}
            }
            div {
                class: "
                    scale-y-0 origin-top group-data-[enabled=true]:scale-y-100
                    z-0
                    *:z-0
                    *:opacity-100
                    *:group-data-[enabled=false]:opacity-0
                ",
                {children}
            }
        }
    }
}
