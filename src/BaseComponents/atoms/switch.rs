use dioxus::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Switch(
    clicked: Signal<bool>,
    onclick: Option<EventHandler>,
    #[props(default)] class: String,
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
#[derive(Copy, Clone, derive_more::Display)]
pub enum State {
    Left,
    Right,
}

#[component]
pub fn FloatingSwitch(
    lhs_width: f64,
    lhs: Element,
    #[props(default)] lhs_css: String,
    rhs_width: f64,
    rhs: Element,
    #[props(default)] rhs_css: String,
    #[props(default)] class: String,
    mut state: Signal<State>,
) -> Element {
    let left = use_memo(move || match state() {
        State::Left => 0.,
        State::Right => lhs_width,
    });
    let width = use_memo(move || match state() {
        State::Left => lhs_width,
        State::Right => rhs_width,
    });
    rsx! {
        div {
            class: tw_merge!("group relative flex justify-stretch items-center rounded-[30px] bg-background", class),
            "data-selected": state().to_string(),
            width: format!("{}px", lhs_width + rhs_width),
            div {
                class: "z-20 transition-all absolute inset-y-0 rounded-[30px] bg-orange group-data-[selected=Right]:bg-green",
                left: "{left()}px",
                width: "{width()}px",
            }
            button {
                class: tw_merge!("z-30 bg-transparent inline-flex justify-center items-center container", lhs_css),
                onclick: move |_| {
                    state.set(State::Left);
                },
                {lhs}
            }
            button {
                class: tw_merge!("z-30 bg-transparent inline-flex justify-center items-center container", rhs_css),
                onclick: move |_| {
                    state.set(State::Right);
                },
                {rhs}
            }
        }
    }
}
