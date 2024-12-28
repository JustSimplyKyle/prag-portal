use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use document::eval;

use crate::{pages::Pages, HISTORY};

#[component]
pub fn Modal(active: Signal<bool>, children: Element) -> Element {
    let id = format!("v{}", current_scope_id()?.0);
    let id1 = id.clone();
    let id2 = id.clone();
    use_resource(move || {
        let id = id.clone();
        let active = active();
        async move {
            if active {
                HISTORY.write().focus_with_history(Pages::OnHover);
                let show = format!(
                    "
                    const d = document.querySelector('dialog.{id}');
                    if(d) {{
                        d.showModal();
                    }}
                "
                );
                eval(&show).await?;
                info!("Creating modal!");
            } else {
                HISTORY.write().go_prev();
                let close = format!(
                    "
                    const d = document.querySelector('dialog.{id}')
                    if(d) {{
                        d.close();
                    }}
                "
                );
                eval(&close).await?;
                info!("Closing modal!");
            }
            Ok::<(), RenderError>(())
        }
    })()
    .transpose()?;

    rsx! {
        dialog {
            class: "[&::backdrop]:!m-0 [&::backdrop]:!p-0 [&::backdrop]:!border-0 opacity-100 [@starting-style]:opacity-0 backdrop-opacity-100 [@starting-style]:backdrop-opacity-0 bg-deep-background/80 w-screen h-screen overflow-y-scroll {id1}",
            onkeypress: |v| {
                if v.code() == Code::Escape {
                    HISTORY.write().go_prev();
                }
            },
            transition: "all 0.7s allow-discrete",
            {children}
        }
    }
}
