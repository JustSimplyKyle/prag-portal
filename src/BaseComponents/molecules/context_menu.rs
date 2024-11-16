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
