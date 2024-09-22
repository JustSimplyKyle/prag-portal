use std::time::Duration;

use dioxus::prelude::*;
use dioxus_logger::tracing::debug;

pub fn use_text_scroller() -> (Signal<Option<MountedEvent>>, Signal<bool>, Signal<String>) {
    let mounted = use_signal(|| None);
    let mut status = use_signal(|| false);
    let mut style = use_signal(String::new);
    use_effect(move || {
        let _ = mounted.read();
        debug!("mounted changed");
    });
    use_effect(move || {
        let status = status();
        debug!("status changed: {status}");
    });
    use_effect(move || {
        let style = style.read();
        debug!("style changed: {style}");
    });
    use_future(move || async move {
        loop {
            let element: Option<MountedEvent> = mounted();
            if let Some(ele) = element {
                let client = ele.get_client_rect().await.unwrap().width().round();
                let scroll = ele.get_scroll_size().await.unwrap().width.round();
                if scroll / client != 0. && scroll / client != 1. && !(scroll / client).is_nan() {
                    let new_style = format!("--from-width:{client}px; --to-width:-{scroll}px");
                    if *style.peek() != new_style {
                        style.set(new_style);
                    }
                }
                if *status.peek() != (scroll > client) {
                    status.set(scroll > client);
                }
            }
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    });
    (mounted, status, style)
}
