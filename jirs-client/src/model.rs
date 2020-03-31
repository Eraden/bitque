use std::collections::hash_map::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use jirs_data::*;

use crate::{IssueId, UserId, HOST_URL};

pub type ProjectId = i32;
pub type StatusCode = u32;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialOrd, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Page {
    Project,
    ProjectSettings,
    Login,
    Register,
}

impl Page {
    pub fn to_path(&self) -> String {
        match self {
            Page::Project => "/board",
            Page::ProjectSettings => "/project-settings",
            Page::Login => "/login",
            Page::Register => "/register",
        }
        .to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCommentForm {
    pub fields: CreateCommentPayload,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateIssueForm {
    pub fields: CreateIssuePayload,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProjectForm {
    pub id: ProjectId,
    pub fields: UpdateProjectPayload,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectPage {
    pub about_tooltip_visible: bool,
    pub text_filter: String,
    pub active_avatar_filters: Vec<UserId>,
    pub only_my_filter: bool,
    pub recently_updated_filter: bool,
    pub dragged_issue_id: Option<IssueId>,
    pub drag_point: Point,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub access_token: Option<Uuid>,
    pub user: Option<User>,
    pub project: Option<FullProject>,
    pub project_form: Option<UpdateProjectForm>,
    pub issue_form: Option<CreateIssueForm>,
    pub comment_form: Option<CreateCommentForm>,
    pub issues: Vec<Issue>,
    pub comments_by_project_id: HashMap<ProjectId, Vec<Comment>>,
    pub page: Page,
    pub host_url: String,
    pub project_page: ProjectPage,
}

impl Default for Model {
    fn default() -> Self {
        let host_url = unsafe { HOST_URL.clone() };
        Self {
            access_token: None,
            project: None,
            user: None,
            issue_form: None,
            project_form: None,
            comment_form: None,
            issues: vec![],
            comments_by_project_id: Default::default(),
            page: Page::Project,
            host_url,
            project_page: ProjectPage {
                about_tooltip_visible: false,
                text_filter: "".to_string(),
                active_avatar_filters: vec![],
                only_my_filter: false,
                recently_updated_filter: false,
                dragged_issue_id: None,
                drag_point: Point::default(),
            },
        }
    }
}

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
