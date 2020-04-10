use seed::{prelude::*, *};

use jirs_data::{IssuePriority, IssueType};

use crate::shared::ToNode;
use crate::Msg;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Icon {
    Bug,
    Stopwatch,
    Task,
    Story,
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
}

impl Icon {
    pub fn to_color(self) -> Option<String> {
        match self {
            Icon::Bug | Icon::Task | Icon::Story => Some(format!("var(--{})", self)),
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
        };
        f.write_str(code)
    }
}

impl From<IssueType> for Icon {
    fn from(t: IssueType) -> Self {
        match t {
            IssueType::Task => Icon::Task,
            IssueType::Bug => Icon::Bug,
            IssueType::Story => Icon::Story,
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
    size: Option<Option<i32>>,
    class_list: Vec<String>,
    style_list: Vec<String>,
}

impl StyledIconBuilder {
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(Some(size));
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

    pub fn build(self) -> StyledIcon {
        StyledIcon {
            icon: self.icon,
            size: self.size.unwrap_or_default(),
            class_list: self.class_list,
            style_list: self.style_list,
        }
    }
}

pub struct StyledIcon {
    pub icon: Icon,
    pub size: Option<i32>,
    pub class_list: Vec<String>,
    pub style_list: Vec<String>,
}

impl StyledIcon {
    pub fn build(icon: Icon) -> StyledIconBuilder {
        StyledIconBuilder {
            icon,
            size: None,
            class_list: vec![],
            style_list: vec![],
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
        ""
    ]
}
