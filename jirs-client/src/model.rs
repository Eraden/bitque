use std::collections::hash_map::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use jirs_data::*;

use crate::{IssueId, UserId, HOST_URL};

pub type ProjectId = i32;

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum ModalType {
    AddIssue(AddIssueModal),
    EditIssue(IssueId, EditIssueModal),
    DeleteIssueConfirm(IssueId),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Hash)]
pub struct EditIssueModal {
    pub id: i32,
    pub top_select_opened: bool,
    pub top_select_filter: String,
    pub value: IssueType,
    pub link_copied: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialOrd, PartialEq, Hash)]
pub struct AddIssueModal {
    pub title: String,
    #[serde(rename = "type")]
    pub issue_type: IssueType,
    pub status: IssueStatus,
    pub priority: IssuePriority,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub project_id: i32,
    pub user_ids: Vec<i32>,
    pub reporter_id: Option<i32>,

    // modal fields
    pub type_select_filter: String,
    pub type_select_opened: bool,

    pub reporter_select_filter: String,
    pub reporter_select_opened: bool,

    pub assignees_select_filter: String,
    pub assignees_select_opened: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Page {
    Project,
    EditIssue(IssueId),
    AddIssue,
    ProjectSettings,
    Login,
    Register,
}

impl Page {
    pub fn to_path(&self) -> String {
        match self {
            Page::Project => "/board".to_string(),
            Page::EditIssue(id) => format!("/issues/{id}", id = id),
            Page::AddIssue => format!("/add-issues"),
            Page::ProjectSettings => "/project-settings".to_string(),
            Page::Login => "/login".to_string(),
            Page::Register => "/register".to_string(),
        }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectPage {
    pub about_tooltip_visible: bool,
    pub text_filter: String,
    pub active_avatar_filters: Vec<UserId>,
    pub only_my_filter: bool,
    pub recently_updated_filter: bool,
    pub dragged_issue_id: Option<IssueId>,
}

#[derive(Debug)]
pub struct Model {
    pub host_url: String,
    pub access_token: Option<Uuid>,

    // mapped
    pub comments_by_project_id: HashMap<ProjectId, Vec<Comment>>,

    // forms
    pub project_form: Option<UpdateProjectForm>,
    pub issue_form: Option<CreateIssueForm>,
    pub comment_form: Option<CreateCommentForm>,

    // modals
    pub modals: Vec<ModalType>,

    // pages
    pub page: Page,
    pub project_page: ProjectPage,

    pub project: Option<Project>,
    pub user: Option<User>,
    pub issues: Vec<Issue>,
    pub users: Vec<User>,
}

impl Default for Model {
    fn default() -> Self {
        let host_url = unsafe { HOST_URL.clone() };
        Self {
            access_token: None,
            user: None,
            issue_form: None,
            project_form: None,
            comment_form: None,
            issues: vec![],
            users: vec![],
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
            },
            modals: vec![],
            project: None,
        }
    }
}
