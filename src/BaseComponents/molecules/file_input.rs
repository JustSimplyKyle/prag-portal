use dioxus::prelude::*;

#[component]
pub fn FileInput(
    #[props(extends = label, extends = GlobalAttributes)] attributes: Vec<Attribute>,
    filename: Signal<Option<String>>,
    children: Element,
) -> Element {
    rsx! {
        label {
            role: "button",
            ..attributes,
            input {
                r#type: "file",
                class: "hidden",
                accept: ".png,.jpg,.avif,.heif",
                multiple: false,
                onchange: move |evt| {
                    if let Some(files) = evt.files() {
                        filename.set(files.files().first().cloned());
                    }
                },
            }
            {children}
        }
    }
}
