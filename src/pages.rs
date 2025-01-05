use std::hash::{DefaultHasher, Hash, Hasher};

use dioxus::prelude::*;
use rust_lib::api::shared_resources::collection::CollectionId;

use crate::{scrollable::Scrollable, BaseComponents::molecules::switcher::StateSwitcher, HISTORY};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Pages {
    MainPage,
    Explore,
    Collections,
    DownloadProgress,
    CollectionPage {
        id: CollectionId,
        state: CollectionPageState,
    },
    OnHover,
}

impl Scrollable for Pages {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, derive_more::Display)]
pub enum CollectionPageState {
    Display,
    Edit,
}

impl StateSwitcher for Pages {
    fn hashed_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn compare(&self) -> bool {
        HISTORY.read().active() == self
    }

    fn switch_active_to_self(&self) {
        HISTORY.write().focus_with_history(self.clone());
    }
}

impl ToString for Pages {
    fn to_string(&self) -> String {
        match self {
            Self::MainPage => "main-page".into(),
            Self::Explore => "explore".into(),
            Self::Collections => "collections".into(),
            Self::DownloadProgress => "download-progress".into(),
            Self::CollectionPage { id, state } => {
                let mut hasher = DefaultHasher::new();
                id.hash(&mut hasher);
                let hash = hasher.finish();
                format!("collection-page-{state}-{hash}")
            }
            Self::OnHover => "hover".into(),
        }
    }
}

impl Pages {
    #[must_use]
    pub const fn collection_display(id: CollectionId) -> Self {
        Self::CollectionPage {
            id,
            state: CollectionPageState::Display,
        }
    }

    #[must_use]
    pub const fn collection_edit(id: CollectionId) -> Self {
        Self::CollectionPage {
            id,
            state: CollectionPageState::Edit,
        }
    }

    #[must_use]
    pub fn should_render(&self) -> bool {
        HISTORY.with(|x| {
            x.active() == self || x.prev_peek() == Some(self) || x.history().contains(self)
        })
    }

    #[must_use]
    pub fn flyer_attributes(&self, history: ReadableRef<Signal<crate::History>>) -> Vec<Attribute> {
        let no_slideout = history.prev_peek() == Some(self)
            && !matches!(
                history.active,
                Self::CollectionPage { .. } | Self::DownloadProgress
            );

        let edit_case = matches!(
            history.prev_peek(),
            Some(Self::CollectionPage {
                state: CollectionPageState::Edit,
                ..
            })
        ) && matches!(
            self,
            Self::CollectionPage {
                state: CollectionPageState::Edit,
                ..
            }
        );

        let left = if no_slideout || edit_case {
            "100dvw"
        } else {
            ""
        };

        let z_index = if &history.active == self {
            "100"
        } else if history.prev_peek() == Some(self) {
            "51"
        } else {
            "0"
        };

        let display = if &history.active == self || history.prev_peek() == Some(self) {
            "block"
        } else {
            "none"
        };

        let animation = if &history.active == self {
            "slideLeft 500ms var(--gentle-easing)"
        } else if no_slideout || edit_case {
            "slideRight 500ms var(--gentle-easing)"
        } else {
            ""
        };

        let display = Attribute::new("display", display, Some("style"), false);
        let animation = Attribute::new("animation", animation, Some("style"), false);
        let z_index = Attribute::new("z-index", z_index, Some("style"), false);
        let left = Attribute::new("left", left, Some("style"), false);

        vec![display, animation, z_index, left]
    }
}
