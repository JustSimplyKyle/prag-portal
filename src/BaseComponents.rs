use std::rc::Rc;

use dioxus::prelude::*;

use tailwind_fuse::*;

use crate::{collections::SEARCH, main_page::ARROW_LEFT, TOP_LEVEL_COMPONENT};

#[derive(Clone)]
pub struct ComponentPointer<P: Properties> {
    pub name: String,
    pub props: P,
    pub pointer: Component<P>,
}

#[component]
pub fn Modal(
    children: Element,
    name: String,
    active: Signal<bool>,
    #[props(default = true)] close_on_outer_click: bool,
) -> Element {
    let props = subModalProps::builder()
        .children(children)
        .active(active)
        .close_on_outer_click(close_on_outer_click)
        .build();
    if TOP_LEVEL_COMPONENT().into_iter().all(|x| &x.name != &name) {
        #[allow(deprecated)]
        let pointer = ComponentPointer {
            name,
            props,
            pointer: subModal,
        };
        TOP_LEVEL_COMPONENT.write().push(pointer);
    }
    None
}

#[component]
#[deprecated = "DO NOT USE. Use `Modal` instead, this should be private, but Dioxus does not allow it."]
pub fn subModal(
    children: Element,
    mut active: Signal<bool>,
    close_on_outer_click: bool,
) -> Element {
    let mut modal_hover = use_signal(|| false);
    rsx! {
        div {
            class: "inline-block z-[200] aria-[selected=false]:hidden aria-[selected=false]:z-0 flex justify-center items-center absolute left-0 top-0 w-screen h-screen bg-white/30",
            "aria-selected": active(),
            onclick: move |_| {
                if !modal_hover() && close_on_outer_click {
                    *active.write() = false;
                }
            },
            div {
                class: "w-fit h-fit",
                onmouseenter: move |_| {
                    if close_on_outer_click {
                        *modal_hover.write() = true;
                    }
                },
                onmouseleave: move |_| {
                    if close_on_outer_click {
                        *modal_hover.write() = false;
                    }
                },
                {children}
            }
        }
    }
}

/// `Switcher` is a trait for objects that can be compared with a global signal,
/// switch the global signal to themselves, and provide a hashed value for equality checks.
pub trait Switcher {
    /// Compares the object with the global signal.
    /// Returns `true` if they are the same, `false` otherwise.
    fn compare(&self) -> bool;

    /// Changes the global signal to match the object.
    fn switch_active_to_self(&self);

    /// Returns a hashed value of the object for use in equality checks.
    fn hashed_value(&self) -> u64;
}

impl<'a, 'b> PartialEq<dyn Switcher + 'b> for dyn Switcher + 'a {
    fn eq(&self, other: &(dyn Switcher + 'b)) -> bool {
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
    // automatically roundness correction
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

#[component]
pub fn Switch(clicked: Signal<bool>, onclick: Option<EventHandler>) -> Element {
    rsx! {
        button {
            class: "group w-[45px] p-[5px] rounded-full flex justify-start items-center bg-background",
            "aria-selected": clicked(),
            onclick: move |_| {
                clicked.toggle();
                if let Some(onclick) = onclick {
                    onclick(());
                }
            },
            div { class: "transition-all duration-700 ease-linear flex-none group-aria-selected:flex-auto" }
            div { class: "transition-all ease-in-out w-[20px] h-[20px] group-aria-selected:border-4 group-aria-selected:border-green group-aria-selected:bg-none rounded-full bg-secondary-surface" }
        }
    }
}
#[component]
pub fn SearchBar(sender: Option<Signal<String>>) -> Element {
    let mut total_clicked = use_signal(|| false);
    let mut input_clicked = use_signal(|| false);
    let mut search = use_signal(|| String::from("搜尋"));
    use_effect(move || {
        if !total_clicked() {
            search.set(String::from("搜尋"));
        }
        if input_clicked() {
            search.set(String::new());
        }
    });
    use_effect(move || {
        if let Some(mut sender) = sender {
            sender.set(search());
        }
    });
    rsx! {
        Button {
            roundness: Roundness::Pill,
            string_placements: rsx! {
                div { class: "relative text-stone-950/20 ", resize: false,
                    input {
                        r#type: "text",
                        id: "test",
                        class: "aboslute overflow-x-scroll w-full grow-0 inset-0 hidden group-aria-selected:block align-middle border-0 overflow-x-clip",
                        value: search(),
                        oninput: move |event| {
                            search.set(event.value());
                        },
                        onclick: move |event| {
                            if !input_clicked() {
                                input_clicked.set(true);
                            }
                            event.stop_propagation();
                        }
                    }
                }
                div { class: "flex flex-row-reverse items-baseline",
                    {ContentType::svg(ARROW_LEFT).css("svg-[20px]")},
                    {ContentType::svg(SEARCH).css("svg-[30px]")}
                }
            },
            onclick: move |()| {
                input_clicked.set(false);
                total_clicked.toggle();
            },
            focus_color_change: true,
            fill_mode: FillMode::Fit,
            extended_css_class: "group transition-all w-20 grid grid-flow-col justify-stretch content-center [&_*]:transition-all h-[55px] aria-selected:w-[300px] aria-selected:bg-white pl-[15px] pr-[10px]"
        }
    }
}

#[component]
pub fn Button(
    roundness: Roundness,
    #[props(into)] string_placements: StringPlacements,
    #[props(default)] extended_css_class: String,
    signal: Option<Rc<dyn Switcher>>,
    #[props(default = true)] clickable: bool,
    #[props(into)] onclick: Option<EventHandler>,
    #[props(extends = GlobalAttributes, extends = div)] mut attributes: Vec<Attribute>,
    #[props(default)] size: Size,
    #[props(default)] fill_mode: FillMode,
    #[props(default = false)] focus_color_change: bool,
) -> Element {
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
    let mut clickiness = use_signal(|| false);
    let class = tw_merge!(class, extended_css_class);
    rsx! {
        div {
            class,
            role: if clickable { "button" } else { "" },
            aria_selected: {
                if let Some(x) = signal.as_ref() {
                    x.compare()
                } else {
                    focus_color_change && clickiness()
                }
            },
            onclick: move |_| {
                if signal.is_none() && focus_color_change {
                    clickiness.toggle();
                }
                if let Some(x) = onclick {
                    x(());
                } else if let Some(x) = &mut signal {
                    x.switch_active_to_self();
                }
            },
            ..attributes,
            {
                match string_placements {
                    StringPlacements::Designed(s) => rsx! {
                        for x in s {
                            { x }
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
        self.css = tw_merge!(self.css, css.into());
        self
    }
    pub fn get_element(self) -> Element {
        let alignment_class = self.alignment.get_alignment_class();
        rsx! {
            div { class: tw_merge!(alignment_class, self.css),
                for x in self.contents {
                    {x}
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Content {
    content: ContentType,
    css: String,
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
        self.css = tw_merge!(self.css, css.into(), "leading-[1.2] capsize");
        self
    }

    pub fn alignment(self, alignment: Alignment) -> Contents {
        Contents::new(vec![self], alignment)
    }

    pub fn get_element(self) -> Element {
        match self.content {
            ContentType::Svg(x) => {
                rsx! {
                    div { class: tw_merge!(self.css, "[&_*]:pointer-events-none"),
                        object { id: "mysvg", r#type: "image/svg+xml", data: x }
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
                        class: self.css,
                        background_size,
                        background_position: "center",
                        background_image: format!("url(\'{}\')", x)
                    }
                }
            }
            ContentType::Text(x) | ContentType::Hint(x) => {
                rsx! {
                    div { class: self.css, { x } }
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
        }
    }

    pub fn image(string: impl Into<String>) -> Content {
        let content_type = Self::Image(string.into());
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
        let css = String::from("text-[1em] leading-[1.2] capsize");
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
        let css = String::from("text-[17px] text-hint leading-[1.2] capsize");
        Content {
            content: content_type,
            css,
        }
    }

    #[must_use]
    pub fn custom(custom: impl Into<Element>) -> Content {
        let content_type = Self::Custom(custom.into());
        let css = String::new();
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
            Self::Left => "text-left justify-self-start flex",
            Self::Center => "text-center justify-self-center flex",
            Self::Right => "text-right justify-self-end flex",
            Self::Custom(ref class) => class,
        }
        .into()
    }
}
