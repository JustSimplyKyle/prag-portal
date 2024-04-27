use std::rc::Rc;

use dioxus::prelude::*;

use tailwind_fuse::*;

use crate::TOP_LEVEL_COMPONENT;

pub type ComponentPointer = (String, subModalProps, fn(subModalProps) -> Element);

#[component]
pub fn Modal(children: Element, active: Signal<bool>, name: String) -> Element {
    let props = subModalProps::builder()
        .children(children)
        .active(active)
        .build();
    if TOP_LEVEL_COMPONENT().iter().all(|x| x.0 != name) {
        #[allow(deprecated)]
        TOP_LEVEL_COMPONENT.write().push((name, props, subModal));
    }
    None
}

#[component]
#[deprecated]
pub fn subModal(children: Element, active: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "inline-block z-[200] aria-[selected=false]:hidden aria-[selected=false]:z-0 absolute left-0 top-0 w-screen h-screen flex justify-center items-center bg-white/30 backdrop-filter backdrop-brightness-[.1] backdrop-blur-[100px]",
            "aria-selected": active(),
            {children}
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct ButtonProps {
    pub roundness: Roundness,
    #[props(into)]
    pub string_placements: StringPlacements,
    #[props(default)]
    pub extended_css_class: String,
    #[props(into)]
    pub signal: Option<Rc<dyn ActiveCompare>>,
    #[props(default = true)]
    pub is_button: bool,
    pub onclick: Option<EventHandler>,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    #[props(default)]
    pub size: Size,
    #[props(default)]
    pub fill_mode: FillMode,
    #[props(default = true)]
    pub focus_color_change: bool,
}

pub trait ActiveCompare {
    // Compare with the Global Signal
    fn compare(&self) -> bool;
    // Changes the current Global Signal to this
    fn switch_active(&self);
    // Hashed value to use for partial eq implementation
    fn hashed_value(&self) -> u64;
}

impl<'a, 'b> PartialEq<dyn ActiveCompare + 'b> for dyn ActiveCompare + 'a {
    fn eq(&self, other: &(dyn ActiveCompare + 'b)) -> bool {
        self.hashed_value() == other.hashed_value()
    }
}
#[derive(TwClass, Clone, Copy)]
#[tw(
    class = "transition-all ease-in-out drop-shadow-lg duration-300 text-white bg-deep-background items-center"
)]
pub struct ButtonClass {
    pub roundness: Roundness,
    pub items_count: ItemsCount,
    pub size: Size,
    pub fill_mode: FillMode,
}

impl ButtonClass {
    const fn setup(&self) -> &str {
        match self.items_count {
            ItemsCount::One => "",
            ItemsCount::AboveOne => match self.roundness {
                Roundness::Top | Roundness::None | Roundness::Bottom => "pr-8",
                Roundness::Pill => match self.size {
                    Size::Fat => "",
                    Size::Medium => "pr-[25px]",
                    Size::Small => "pr-[5px]",
                },
            },
        }
    }
    #[must_use]
    pub fn to_class(&self) -> String {
        tw_merge!(IntoTailwindClass::to_class(self), self.setup())
    }
    #[must_use]
    pub fn with_class(&self, string: impl AsRef<str>) -> String {
        let class = IntoTailwindClass::with_class(self, string);
        tw_merge!(class, self.setup())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, TwVariant)]
pub enum FillMode {
    #[tw(default, class = "min-w-full")]
    Fill,
    #[tw(class = "min-w-fit space-x-2.5")]
    Fit,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, TwVariant)]
pub enum Size {
    #[tw(default, class = "text-2xl p-5 font-bold")]
    Fat,
    #[tw(class = "pl-[20px] py-[12px] text-lg")]
    Medium,
    #[tw(class = "py-[5px] px-[20px] text-[17px]")]
    Small,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, TwVariant)]
pub enum ItemsCount {
    #[tw(class = "flex justify-center items-center")]
    One,
    #[tw(default, class = "grid grid-flow-col justify-stretch items-center")]
    AboveOne,
}

impl From<usize> for ItemsCount {
    fn from(value: usize) -> Self {
        if value == 1 {
            Self::One
        } else {
            Self::AboveOne
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, TwVariant)]
pub enum Roundness {
    #[tw(class = "rounded-t-3xl")]
    Top,
    #[tw(default, class = "")]
    None,
    #[tw(class = "rounded-b-3xl")]
    Bottom,
    #[tw(class = "rounded-full")]
    Pill,
}

/// # Props
/// - roundness: `Roundness`
/// - string_placements: `StringPlacements`,
/// - signal: `Option<Pages>`,
/// - extended_css_class: `String`, // optional
/// - is_button: `bool`, // default = true
/// - onclick: `Option<EventHandler>`,
/// - attributes: `Vec<Attribute>`,
/// - size: `Size`, // optional, default = Size::Fat
/// - fill_mode: `FillMode`, // optional default = FillMode::Fill
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let ButtonProps {
        roundness,
        string_placements,
        signal,
        extended_css_class,
        is_button,
        onclick,
        mut attributes,
        size,
        fill_mode,
        focus_color_change,
    } = props;
    attributes.retain(|x| x.name != "class");
    let class = ButtonClass {
        roundness,
        items_count: string_placements.len().into(),
        size,
        fill_mode,
    }
    .with_class(if focus_color_change {
        "aria-selected:bg-white aria-selected:text-black"
    } else {
        ""
    });
    let class = tw_merge!(class, extended_css_class);
    rsx! {
        div {
            class,
            role: if is_button { "button" } else { "" },
            aria_selected: signal.as_ref().map(|x| x.compare()).unwrap_or(false),
            onclick: move |_| {
                if let Some(x) = onclick {
                    x(());
                } else if let Some(x) = &signal {
                    x.switch_active();
                }
            },
            ..attributes,
            {
                match string_placements {
                    StringPlacements::Designed(s) => rsx! {
                        for x in s {
                            { x.get_element() }
                        }
                    },
                    StringPlacements::Custom(x) => x,
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum StringPlacements {
    Designed(Vec<Contents>),
    Custom(Element),
}

impl StringPlacements {
    pub fn len(&self) -> usize {
        match &self {
            Self::Designed(x) => x.len(),
            Self::Custom(x) => x.as_ref().map_or(0, |x| x.dynamic_nodes.len()),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contents {
    contents: Vec<Content>,
    css: String,
    alignment: Alignment, // Positioning
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
        self.css = tw_merge!(self.css, css.into());
        self
    }
    pub fn get_element(self) -> Element {
        let alignment_class = self.alignment.get_alignment_class();
        rsx! {
            div { class: tw_merge!(alignment_class, self.css),
                for x in self.contents {
                    {x.get_element()}
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content {
    content: ContentType,
    css: String,
}

impl Content {
    // Constructor for ease of creation
    pub fn new(content: ContentType) -> Self {
        Self {
            content,
            css: String::new(),
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
        self.css = tw_merge!(self.css, css.into());
        self
    }

    pub fn alignment(self, alignment: Alignment) -> Contents {
        Contents::new(vec![self], alignment)
    }

    pub fn get_element(self) -> Element {
        match self.content {
            ContentType::Svg(x) => {
                rsx! {
                    div { class: self.css,
                        object { r#type: "image/svg+xml", data: x }
                    }
                }
            }
            ContentType::Image(x) => {
                rsx! {
                    // div { class: self.css,
                    img { class: self.css, src: x }
                }
            }
            ContentType::Text(x) | ContentType::Hint(x) => {
                rsx! {
                    div { class: self.css, { x } }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    Svg(&'static str),
    Text(String),
    Hint(String),
    Image(String),
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
        let css = String::new();
        Content {
            content: content_type,
            css,
        }
    }

    pub fn image(string: String) -> Content {
        let content_type = Self::Image(string);
        let css = String::new();
        Content {
            content: content_type,
            css,
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
        let css = String::from("text-[1em] leading-normal capsize");
        Content {
            content: content_type,
            css,
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
        let css = String::from("text-[17px] text-hint leading-normal capsize");
        Content {
            content: content_type,
            css,
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
            Self::Left => "text-left",
            Self::Center => "text-center flex justify-center items-center",
            Self::Right => "text-right flex justify-end items-center",
            Self::Custom(ref class) => class,
        }
        .into()
    }
}
