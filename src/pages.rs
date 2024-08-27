use std::{
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use dioxus::prelude::*;
use dioxus_logger::tracing::warn;
use document::eval_provider;
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
}

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
                format!("collection-page-{}-{hash}", state.to_string())
            }
        }
    }
}

impl Scrollable for Pages {
    const GROUP_SELECTOR: &'static str = "group-pages";
}

impl Pages {
    pub fn slide_in_id(&self) -> String {
        format!("flyinout-{}", self.to_string())
    }
    pub const fn collection_display(id: CollectionId) -> Self {
        Self::CollectionPage {
            id,
            state: CollectionPageState::Display,
        }
    }
    pub const fn collection_edit(id: CollectionId) -> Self {
        Self::CollectionPage {
            id,
            state: CollectionPageState::Edit,
        }
    }

    pub fn should_render(&self) -> bool {
        HISTORY.read().active() == self || HISTORY.read().prev_peek() == Some(self)
    }

    /// Applies slide-in animations to HTML elements based on data attributes.
    ///
    /// This function dynamically applies CSS animations to elements within a webpage
    /// using Tailwind CSS-defined animations. It targets elements with a specific class ('.group')
    /// and adjusts their styles according to their data attributes.
    ///
    /// ## Attributes
    /// The function expects HTML elements to have certain attributes and structure:
    /// * Top level element should have the class `group`.
    /// * Each `group` element should contain at least one child element with an `id` that is acquired by `self.slide_in_id()`
    ///
    /// ## Data Attributes
    /// * `data-prev`: This attribute specifies whether the element was the previous element in a
    ///   sequence. If `true`, the `slideRight` animation is applied.
    /// * `data-selected`: This attribute indicates if the element is the currently selected one.
    ///   If `true`, the `slideLeft` animation is applied.
    ///
    /// ## Usage
    /// To use this function, ensure that your HTML elements are set up correctly with the
    /// required `id` and data attributes. Additionally, for most use cases involving animations or transitions,
    /// it's essential to manage the positioning context correctly:
    ///
    /// - The parent container should have a **relative** positioning to serve as the positioning context for its children.
    /// - Child elements, which are the targets of the animations, should be styled with **absolute** positioning to overlay
    ///   within the relative container seamlessly.
    ///
    /// It is crucial to call this function at the start of each component's lifecycle to properly initialize
    /// the animations.
    ///
    /// Here is an example element setup:
    ///
    /// ```rust
    /// fn Component() {
    ///     Pages::DownloadProgress.apply_slide_in();
    ///     rsx! {
    ///         div {
    ///             "data-selected": selected.to_string(),
    ///             "data-prev": prev.map_or_else(String::new, |x| x.to_string()),
    ///             div { class: "w-full min-h-screen relative",
    ///                 div { class: "absolute inset-0 z-0 min-h-full min-w-full", id: Pages::DownloadProgress.slide_in_id(), LayoutContainer { DownloadProgress {} } }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn apply_slide_in(self) {
        let function = r#"
            function applyStyles(dataValue) {
                const groups = document.querySelectorAll('.group-pages');            
                groups.forEach(group => {
                    const prev = group.getAttribute('data-prev') === dataValue;
                    const selected = group.getAttribute('data-selected') === dataValue;
                    const target = group.querySelector('#flyinout-' + dataValue);

                    // Reset styles first
                    target.style.insetInlineStart = '';
                    target.style.zIndex = '0';
                    target.style.display = 'none';
                    target.style.animation = '';

                    if (selected) {
                        target.style.zIndex = '100';
                        target.style.display = 'block';                        
                        target.style.animation = 'slideLeft 500ms';
                    }
                    if (prev) {
                        target.style.insetInlineStart = '100dvw';
                        target.style.zIndex = '51';
                        target.style.display = 'block';                        
                        target.style.animation = 'slideRight 500ms';
                    } 
                });
            }
        "#;
        if let Err(x) = eval(&format!(
            " {function}
                  applyStyles(\"{}\");
                ",
            self.to_string()
        ))
        .send(().into())
        {
            warn!("Javascript Error: {x}");
        }
    }
}
