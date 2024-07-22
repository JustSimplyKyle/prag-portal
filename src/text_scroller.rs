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
                if scroll / client != 0. && scroll / client != 1. && !(scroll / client).is_nan() {
                    println!("{}%", scroll / client * 100.);
                }
                status.set(scroll > client)
            }
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    });
    (mounted, status)
}
