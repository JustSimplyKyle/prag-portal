use std::time::Duration;

use dioxus::prelude::*;

pub fn use_text_scroller() -> (Signal<Option<MountedEvent>>, Signal<bool>) {
    let mounted = use_signal(|| None);
    let mut status = use_signal(|| false);
    use_future(move || async move {
        loop {
            let element: Option<MountedEvent> = mounted();
            if let Some(ele) = element {
                let client = ele.get_client_rect().await.unwrap().width();
                let scroll = ele.get_scroll_size().await.unwrap().width;
                status.set(scroll > client)
            }
            tokio::time::sleep(Duration::from_millis(450)).await;
        }
    });
    (mounted, status)
}
