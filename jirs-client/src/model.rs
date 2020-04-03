use std::collections::hash_map::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use jirs_data::*;

use crate::{IssueId, UserId, HOST_URL};

pub type ProjectId = i32;

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub enum ModalType {
    EditIssue(IssueId, EditIssueModal),
    DeleteIssueConfirm(IssueId),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub struct EditIssueModal {
    pub id: i32,
    pub top_select_opened: bool,
    pub top_select_filter: String,
    pub value: IssueType,
    pub link_copied: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Page {
    Project,
    EditIssue(IssueId),
    ProjectSettings,
    Login,
    Register,
}

impl Page {
    pub fn to_path(&self) -> String {
        match self {
            Page::Project => "/board".to_string(),
            Page::EditIssue(id) => format!("/issues/{id}", id = id),
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
    pub modals: Vec<ModalType>,
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
            },
            modals: vec![],
        }
    }
}
