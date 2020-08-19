use seed::{prelude::*, *};

use jirs_data::{IssuePriority, IssueType};

use crate::shared::ToNode;
use crate::Msg;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Icon {
    Stopwatch,

    Bug,
    Task,
    Story,
    Epic,

    DoubleLeft,
    DoubleRight,

    ArrowDown,
    ArrowLeftCircle,
    ArrowUp,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    Board,
    Help,
    Link,
    Menu,
    More,
    Attach,
    Plus,
    Search,
    Issues,
    Settings,
    Close,
    Check,
    Feedback,
    Trash,
    Github,
    Shipping,
    Component,
    Reports,
    Page,
    Calendar,
    ArrowLeft,
    ArrowRight,
    Cop,
    Message,
    User,

    AlignCenter,
    AlignLeft,
    AlignRight,
    AllCaps,
    Bold,
    Brush,
    ClipBoard,
    CodeAlt,
    ColorBucket,
    ColorPicker,
    CopyInvert,
    Copy,
    Cut,
    DeleteAlt,
    EditAlt,
    EraserAlt,
    Font,
    Heading,
    Indent,
    ItalicAlt,
    Italic,
    JustifyAll,
    JustifyCenter,
    JustifyLeft,
    JustifyRight,
    LinkBroken,
    ListingDots,
    ListingNumber,
    Outdent,
    PaperClip,
    Paragraph,
    Pin,
    Printer,
    Redo,
    Rotation,
    Save,
    SmallCap,
    StrikeThrough,
    SubListing,
    Subscript,
    Superscript,
    Table,
    TextHeight,
    TextWidth,
    Underline,
    Undo,
}

impl Icon {
    pub fn to_color(self) -> Option<String> {
        match self {
            Icon::Bug | Icon::Task | Icon::Story | Icon::Epic => Some(format!("var(--{})", self)),
            _ => None,
        }
    }

    pub fn into_styled_builder(self) -> StyledIconBuilder {
        StyledIcon::build(self)
    }
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Icon::Bug => "bug",
            Icon::Stopwatch => "stopwatch",
            Icon::Task => "task",
            Icon::Story => "story",
            Icon::ArrowDown => "arrowDown",
            Icon::ArrowLeftCircle => "arrowLeftCircle",
            Icon::ArrowUp => "arrowUp",
            Icon::ChevronDown => "chevronDown",
            Icon::ChevronLeft => "chevronLeft",
            Icon::ChevronRight => "chevronRight",
            Icon::ChevronUp => "chevronUp",
            Icon::Board => "board",
            Icon::Help => "help",
            Icon::Link => "link",
            Icon::Menu => "menu",
            Icon::More => "more",
            Icon::Attach => "attach",
            Icon::Plus => "plus",
            Icon::Search => "search",
            Icon::Issues => "issues",
            Icon::Settings => "settings",
            Icon::Close => "close",
            Icon::Check => "check",
            Icon::Feedback => "feedback",
            Icon::Trash => "trash",
            Icon::Github => "github",
            Icon::Shipping => "shipping",
            Icon::Component => "component",
            Icon::Reports => "reports",
            Icon::Page => "page",
            Icon::Calendar => "calendar",
            Icon::ArrowLeft => "arrowLeft",
            Icon::ArrowRight => "arrowRight",
            Icon::Cop => "cop",
            Icon::Message => "message",
            Icon::User => "user",
            // rte
            Icon::AlignCenter => "align-center",
            Icon::AlignLeft => "align-left",
            Icon::AlignRight => "align-right",
            Icon::AllCaps => "all-caps",
            Icon::Bold => "bold",
            Icon::Brush => "brush",
            Icon::ClipBoard => "clip-board",
            Icon::CodeAlt => "code-alt",
            Icon::ColorBucket => "color-bucket",
            Icon::ColorPicker => "color-picker",
            Icon::CopyInvert => "copy-invert",
            Icon::Copy => "copy",
            Icon::Cut => "cut",
            Icon::DeleteAlt => "delete-alt",
            Icon::EditAlt => "edit-alt",
            Icon::EraserAlt => "eraser-alt",
            Icon::Font => "font",
            Icon::Heading => "heading",
            Icon::Indent => "indent",
            Icon::ItalicAlt => "italic-alt",
            Icon::Italic => "italic",
            Icon::JustifyAll => "justify-all",
            Icon::JustifyCenter => "justify-center",
            Icon::JustifyLeft => "justify-left",
            Icon::JustifyRight => "justify-right",
            Icon::LinkBroken => "link-broken",
            Icon::Outdent => "outdent",
            Icon::PaperClip => "paper-clip",
            Icon::Paragraph => "paragraph",
            Icon::Pin => "pin",
            Icon::Printer => "printer",
            Icon::Redo => "redo",
            Icon::Rotation => "rotation",
            Icon::Save => "save",
            Icon::SmallCap => "small-cap",
            Icon::StrikeThrough => "strike-through",
            Icon::SubListing => "sub-listing",
            Icon::Subscript => "subscript",
            Icon::Superscript => "superscript",
            Icon::Table => "table",
            Icon::TextHeight => "text-height",
            Icon::TextWidth => "text-width",
            Icon::Underline => "underline",
            Icon::Undo => "undo",
            Icon::ListingDots => "listing-dots",
            Icon::ListingNumber => "listing-number",
            Icon::Epic => "epic",

            Icon::DoubleLeft => "double-left",
            Icon::DoubleRight => "double-right",
        };
        f.write_str(code)
    }
}

impl From<IssueType> for Icon {
    fn from(t: IssueType) -> Self {
        use IssueType::*;
        match t {
            Task => Icon::Task,
            Bug => Icon::Bug,
            Story => Icon::Story,
        }
    }
}

impl From<IssuePriority> for Icon {
    fn from(t: IssuePriority) -> Self {
        match t {
            IssuePriority::Highest => Icon::ArrowUp,
            IssuePriority::High => Icon::ArrowUp,
            IssuePriority::Medium => Icon::ArrowUp,
            IssuePriority::Low => Icon::ArrowDown,
            IssuePriority::Lowest => Icon::ArrowDown,
        }
    }
}

impl Into<StyledIcon> for Icon {
    fn into(self) -> StyledIcon {
        StyledIcon::build(self).build()
    }
}

impl ToNode for Icon {
    fn into_node(self) -> Node<Msg> {
        let styled: StyledIcon = self.into();
        styled.into_node()
    }
}

pub struct StyledIconBuilder {
    icon: Icon,
    size: Option<i32>,
    class_list: Vec<String>,
    style_list: Vec<String>,
    on_click: Option<EventHandler<Msg>>,
}

impl StyledIconBuilder {
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn add_class<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.class_list.push(name.into());
        self
    }

    pub fn add_style<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.style_list.push(name.into());
        self
    }

    pub fn on_click(mut self, on_click: EventHandler<Msg>) -> Self {
        self.on_click = Some(on_click);
        self
    }

    pub fn build(self) -> StyledIcon {
        StyledIcon {
            icon: self.icon,
            size: self.size,
            class_list: self.class_list,
            style_list: self.style_list,
            on_click: self.on_click,
        }
    }
}

pub struct StyledIcon {
    icon: Icon,
    size: Option<i32>,
    class_list: Vec<String>,
    style_list: Vec<String>,
    on_click: Option<EventHandler<Msg>>,
}

impl StyledIcon {
    pub fn build(icon: Icon) -> StyledIconBuilder {
        StyledIconBuilder {
            icon,
            size: None,
            class_list: vec![],
            style_list: vec![],
            on_click: None,
        }
    }
}

impl ToNode for StyledIcon {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub fn render(values: StyledIcon) -> Node<Msg> {
    let StyledIcon {
        icon,
        size,
        mut class_list,
        mut style_list,
        on_click,
    } = values;

    if let Some(s) = icon.to_color() {
        style_list.push(format!("color: {}", s));
    }

    if let Some(size) = size {
        style_list.push(format!("font-size: {s}px", s = size));
    }

    class_list.push(format!("styledIcon {}", icon));

    i![
        attrs![At::Class => class_list.join(" "), At::Style => style_list.join(";")],
        on_click,
        ""
    ]
}
