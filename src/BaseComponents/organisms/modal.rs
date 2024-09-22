use dioxus::prelude::*;

#[component]
pub fn Modal(active: Signal<bool>, id: String, children: Element) -> Element {
    let id_cloned = id.clone();
    use_effect(move || {
        if active() {
            eval(&format!(
                "document.getElementById(\"{id_cloned}\").showModal();"
            ));
        } else {
            eval(&format!(
                "document.getElementById(\"{id_cloned}\").close();"
            ));
        }
    });
    rsx! {
        dialog {
            class: "[&::backdrop]:!m-0 [&::backdrop]:!p-0 [&::backdrop]:!border-0 overflow-x-hidden overflow-y-hidden opacity-100 [@starting-style]:opacity-0 backdrop-opacity-100 [@starting-style]:backdrop-opacity-0 bg-deep-background/80 w-screen h-screen overflow-y-scroll",
            transition: "all 0.7s allow-discrete",
            id,
            {children}
        }
    }
}
