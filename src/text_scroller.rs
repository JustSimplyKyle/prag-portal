use std::time::Duration;

use dioxus::prelude::*;

pub fn use_text_scroller() -> (Signal<Option<MountedEvent>>, Signal<bool>, Signal<String>) {
    let mounted = use_signal(|| None);
    let mut status = use_signal(|| false);
    let mut style = use_signal(String::new);
    use_future(move || async move {
        loop {
            let element: Option<MountedEvent> = mounted();
            if let Some(ele) = element {
                let client = ele.get_client_rect().await.unwrap().width().round();
                let scroll = ele.get_scroll_size().await.unwrap().width.round();
                if scroll / client != 0. && scroll / client != 1. && !(scroll / client).is_nan() {
                    style.set(format!(
                        "--from-width:{}px; --to-width:-{}px",
                        client, scroll
                    ));
                }
                status.set(scroll > client)
            }
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    });
    (mounted, status, style)
}
