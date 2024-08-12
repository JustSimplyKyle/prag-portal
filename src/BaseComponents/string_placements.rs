use std::str::FromStr;

use dioxus::{prelude::*, CapturedError};
use dioxus_core::DynamicNode;
use tailwind_fuse::*;

#[derive(Clone, PartialEq)]
pub enum StringPlacements {
    Designed(Vec<Contents>),
    Custom(Element),
}

impl StringPlacements {
    pub fn len(&self) -> usize {
        match &self {
            Self::Designed(x) => x.len(),
            Self::Custom(_) => 1,
        }
    }
    pub fn is_empty(&self) -> bool {
        match &self {
            Self::Designed(x) => x.is_empty(),
            Self::Custom(x) => x.as_ref().map_or(true, |x| x.dynamic_nodes.is_empty()),
        }
    }
}

impl From<Vec<Contents>> for StringPlacements {
    fn from(value: Vec<Contents>) -> Self {
        Self::Designed(value)
    }
}
impl From<Element> for StringPlacements {
    fn from(value: Element) -> Self {
        Self::Custom(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Contents {
    contents: Vec<Content>,
    css: String,
    alignment: Alignment, // Positioning
}

impl IntoDynNode for Contents {
    fn into_dyn_node(self) -> dioxus_core::DynamicNode {
        self.get_element().into_dyn_node()
    }
}

impl Contents {
    pub fn new(contents: impl Into<Vec<Content>>, alignment: Alignment) -> Self {
        Self {
            contents: contents.into(),
            css: String::new(),
            alignment,
        }
    }
    pub fn css(mut self, css: impl Into<String>) -> Self {
        let css = css.into();
        self.css = tw_merge!(self.css, css);
        self
    }
    pub fn get_element(self) -> Element {
        let alignment_class = self.alignment.get_alignment_class();
        rsx! {
            div {
                class: tw_merge!(alignment_class, self.css),
                for x in self.contents {
                    {x}
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Content {
    content: ContentType,
    css: String,
    style: String,
    onmounted: Option<Signal<Option<Event<MountedData>>>>,
    onmouseover: Option<EventHandler>,
}
impl std::fmt::Debug for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Content")
            .field("content", &self.content)
            .field("css", &self.css)
            .field("onmounted", &"DEBUG onomunted")
            .finish()
    }
}

impl IntoDynNode for Content {
    fn into_dyn_node(self) -> dioxus_core::DynamicNode {
        self.get_element().into_dyn_node()
    }
}

impl Content {
    // Constructor for ease of creation
    pub fn new(content: ContentType) -> Self {
        Self {
            content,
            css: String::new(),
            style: String::new(),
            onmounted: None,
            onmouseover: None,
        }
    }

    pub fn align_left(self) -> Contents {
        Contents {
            contents: vec![self],
            css: String::new(),
            alignment: Alignment::Left,
        }
    }

    pub fn align_right(self) -> Contents {
        Contents {
            contents: vec![self],
            css: String::new(),
            alignment: Alignment::Right,
        }
    }

    pub fn align_center(self) -> Contents {
        Contents {
            contents: vec![self],
            css: String::new(),
            alignment: Alignment::Center,
        }
    }

    pub fn align_custom(self, custom: impl Into<String>) -> Contents {
        Contents {
            contents: vec![self],
            css: String::new(),
            alignment: Alignment::Custom(custom.into()),
        }
    }

    // Method to apply additional CSS, modifying existing styling
    pub fn css(mut self, css: impl Into<String>) -> Self {
        let css = css.into();
        if css.contains("font") {
            self.css = tw_join!(self.css, css, "leading-[1.2] capsize");
        } else {
            self.css = tw_merge!(self.css, css, "leading-[1.2] capsize");
        }
        self
    }

    pub fn style(mut self, style: impl Into<String>) -> Self {
        let style = style.into();
        self.style = style;
        self
    }

    pub fn onmounted(mut self, signal: Signal<Option<Event<MountedData>>>) -> Self {
        self.onmounted = Some(signal);
        self
    }

    pub fn onmouseover(mut self, closure: impl FnMut(()) + 'static) -> Self {
        let p = Callback::new(closure);
        self.onmouseover = Some(p);
        self
    }

    pub fn alignment(self, alignment: Alignment) -> Contents {
        Contents::new(vec![self], alignment)
    }

    pub fn get_element(self) -> Element {
        match self.content {
            ContentType::Svg(x) => {
                rsx! {
                    div {
                        class: tw_merge!(self.css, "[&_*]:pointer-events-none"),
                        style: self.style,
                        onmounted: move |x| {
                            if let Some(mut signal) = self.onmounted {
                                signal.set(Some(x));
                            }
                        },
                        onmouseover: move |_| {
                            if let Some(x) = self.onmouseover {
                                x(());
                            }
                        },
                        object {
                            id: "mysvg",
                            r#type: "image/svg+xml",
                            data: x
                        }
                    }
                }
            }
            ContentType::Image(x) => {
                let background_size =
                    if self.css.contains("object-cover") || self.css.contains("bg-cover") {
                        "cover"
                    } else {
                        "contain"
                    };
                rsx! {
                    div {
                        onmounted: move |x| {
                            if let Some(mut signal) = self.onmounted {
                                signal.set(Some(x));
                            }
                        },
                        class: self.css,
                        style: self.style,
                        background_size,
                        background_position: "center",
                        background_image: format!("url(\'{}\')", x)
                    }
                }
            }
            ContentType::Text(x) | ContentType::Hint(x) => {
                rsx! {
                    div {
                        style: self.style,
                        onmounted: move |x| {
                            if let Some(mut signal) = self.onmounted {
                                signal.set(Some(x));
                            }
                        },
                        class: self.css,
                        { x }
                    }
                }
            }
            ContentType::Custom(x) => x,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    Svg(&'static str),
    Text(String),
    Hint(String),
    Image(String),
    Custom(Element),
}

impl ContentType {
    /// Constructs an SVG content.
    ///
    /// This function returns a `Content` instance rather than a `ContentType`.
    /// It is advised to use this constructor function for creating SVG content
    /// instead of directly constructing `ContentType`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let my_svg = ContentType::svg("my svg");
    /// assert_eq!(my_svg, Content::new(ContentType::Svg("my svg")));
    /// ```
    #[must_use]
    pub fn svg(string: &'static str) -> Content {
        let content_type = Self::Svg(string);
        let css = String::from("flex items-center");
        Content {
            content: content_type,
            css,
            style: String::new(),
            onmounted: None,
            onmouseover: None,
        }
    }

    pub fn image(string: impl Into<String>) -> Content {
        let content_type = Self::Image(string.into());
        let css = String::new();
        Content {
            content: content_type,
            css,
            style: String::new(),
            onmounted: None,
            onmouseover: None,
        }
    }

    /// Constructs a text content.
    ///
    /// This function returns a `Content` instance rather than a `ContentType`.
    /// It is advised to use this constructor function for creating text content
    /// instead of directly constructing `ContentType`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let my_text = ContentType::text("Hello, world!");
    /// assert_eq!(my_text, Content::new(ContentType::Text("Hello, world!")));
    /// ```
    #[must_use]
    pub fn text(string: impl Into<String>) -> Content {
        let content_type = Self::Text(string.into());
        let css = String::from("text-[1em] leading-[1.2] capsize");
        Content {
            content: content_type,
            css,
            style: String::new(),
            onmounted: None,
            onmouseover: None,
        }
    }

    /// Constructs a text content.
    ///
    /// This function returns a `Content` instance rather than a `ContentType`.
    /// It is advised to use this constructor function for creating text content
    /// instead of directly constructing `ContentType`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let my_text = ContentType::hint("Hello, world!");
    /// assert_eq!(my_text, Content::new(ContentType::Hint("Hello, world!")));
    /// ```
    #[must_use]
    pub fn hint(string: impl Into<String>) -> Content {
        let content_type = Self::Hint(string.into());
        let css = String::from("text-[17px] text-hint leading-[1.2] capsize");
        Content {
            content: content_type,
            css,
            style: String::new(),
            onmounted: None,
            onmouseover: None,
        }
    }

    #[must_use]
    pub fn custom(custom: impl Into<Element>) -> Content {
        let content_type = Self::Custom(custom.into());
        let css = String::new();
        Content {
            content: content_type,
            css,
            style: String::new(),
            onmounted: None,
            onmouseover: None,
        }
    }
    /// Returns `true` if the content type is [`Svg`].
    ///
    /// [`Svg`]: ContentType::Svg
    #[must_use]
    pub const fn is_svg(&self) -> bool {
        matches!(self, Self::Svg(..))
    }

    /// Returns `true` if the content type is [`Text`].
    ///
    /// [`Text`]: ContentType::Text
    #[must_use]
    pub const fn is_text(&self) -> bool {
        matches!(self, Self::Text(..))
    }

    /// Returns `true` if the content type is [`Hint`].
    ///
    /// [`Hint`]: ContentType::Hint
    #[must_use]
    pub const fn is_hint(&self) -> bool {
        matches!(self, Self::Hint(..))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Custom(String), // Allows for custom alignment classes
}

impl Alignment {
    pub fn get_alignment_class(&self) -> String {
        match self {
            Self::Left => "text-left justify-self-start flex",
            Self::Center => "text-center justify-self-center flex",
            Self::Right => "text-right justify-self-end flex",
            Self::Custom(ref class) => class,
        }
        .into()
    }
}
#[component]
pub fn Text(
    children: Element,
    css: Option<String>,
    #[props(into)] style: Option<String>,
    onmounted: Option<Signal<Option<MountedEvent>>>,
) -> Element {
    sub_content_builder(
        ContentType::text,
        children,
        css.unwrap_or_default(),
        style.unwrap_or_default(),
        onmounted,
    )
}

#[component]
pub fn Hint(
    children: Element,
    css: Option<String>,
    style: Option<String>,
    onmounted: Option<Signal<Option<MountedEvent>>>,
) -> Element {
    sub_content_builder(
        ContentType::hint,
        children,
        css.unwrap_or_default(),
        style.unwrap_or_default(),
        onmounted,
    )
}

#[component]
pub fn Image(
    children: Element,
    css: Option<String>,
    style: Option<String>,
    onmounted: Option<Signal<Option<MountedEvent>>>,
) -> Element {
    sub_content_builder(
        ContentType::image,
        children,
        css.unwrap_or_default(),
        style.unwrap_or_default(),
        onmounted,
    )
}

fn sub_content_builder(
    content_type: fn(String) -> Content,
    ele: Element,
    css: String,
    style: String,
    onmounted: Option<Signal<Option<MountedEvent>>>,
) -> Element {
    let vnode = ele?;
    let dynamic = vnode.dynamic_nodes.first();
    let inplace = vnode.template.roots.first();

    let text = matcher(dynamic, inplace)?;

    if let Some(x) = onmounted {
        content_type(text)
            .css(css)
            .onmounted(x)
            .style(style)
            .get_element()
    } else {
        content_type(text).css(css).style(style).get_element()
    }
}

fn matcher(
    dynamic: Option<&DynamicNode>,
    inplace: Option<&TemplateNode>,
) -> Result<String, RenderError> {
    let text = match (dynamic, inplace) {
        (Some(DynamicNode::Text(text)), _) => text.value.clone(),
        (_, Some(TemplateNode::Text { text })) => (*text).to_string(),
        (Some(DynamicNode::Fragment(x)), _) => matcher(
            x.first().unwrap().dynamic_nodes.first(),
            x.first().unwrap().template.roots.first(),
        )?,
        _ => {
            return Err(RenderError::Aborted(
                CapturedError::from_str("please input only text in a `[Text/Hint/Image]` element")
                    .unwrap(),
            ));
        }
    };
    Ok(text)
}
