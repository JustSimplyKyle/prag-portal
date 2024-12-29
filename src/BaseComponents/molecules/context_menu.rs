use dioxus::prelude::*;
use tailwind_fuse::tw_merge;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    DownRightEdge,
    DownLeftEdge,
    UpRightEdge,
}

#[component]
pub fn ContextMenu(
    direction: Direction,
    children: Element,
    #[props(default)] class: String,
    #[props(default = String::from("5px"))] gap: String,
) -> Element {
    let displacement = match direction {
        Direction::DownRightEdge => {
            format!("top:calc(100% + {gap}); right:0;")
        }
        Direction::UpRightEdge => {
            format!("top:0; left:calc(100% + {gap});")
        }
        Direction::DownLeftEdge => {
            format!("top:calc(100% + {gap}); left:0;")
        }
    };
    rsx! {
        div {
            class: tw_merge!(
                "absolute border border-surface rounded-[15px] w-[210px] py-[10px] h-fit bg-background z-[1000000] overflow-visible",
                class
            ),
            style: "box-shadow: 10px 10px 30px 0px rgba(0, 0, 0, 0.25); {displacement}",
            {children}
        }
    }
}

#[component]
pub fn DropDown(
    #[props(default)] class: String,
    base: Element,
    children: Element,
    selector_visibility: Signal<bool>,
    onclick: Option<EventHandler>,
) -> Element {
    rsx! {
        div {
            class: tw_merge!("pl-[20px] pr-[15px] bg-background w-full grow grid grid-flow-col justify-stretch items-center rounded-[20px] relative z-50",class),
            onclick: move |_| {
                selector_visibility.toggle();
                if let Some(onclick) = onclick {
                    onclick(());
                }
            },
            box_shadow: "10px 10px 30px 0px rgba(0, 0, 0, 0.25)",
            div {
                class: "justify-self-start grow trim text-[18px] font-english",
                {base}
            }
            crate::svgs::ARROW_DOWN {
                class: "justify-self-end",
            }
            Menu {
                selector_visibility,
                children,
            }
        }
    }
}

#[component]
pub fn Menu(selector_visibility: Signal<bool>, children: Element) -> Element {
    rsx! {
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
