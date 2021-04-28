use std::collections::hash_map::HashMap;

use jirs_data::*;
use seed::app::Orders;
use seed::browser::web_socket::WebSocket;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::styled_select::StyledSelectState;
use crate::pages::invite_page::InvitePage;
use crate::pages::issues_and_filters::IssuesAndFiltersPage;
use crate::pages::profile_page::model::ProfilePage;
use crate::pages::project_page::model::ProjectPage;
use crate::pages::project_settings_page::ProjectSettingsPage;
use crate::pages::reports_page::model::ReportsPage;
use crate::pages::sign_in_page::model::SignInPage;
use crate::pages::sign_up_page::model::SignUpPage;
use crate::pages::users_page::model::UsersPage;
use crate::{BuildMsg, Msg};

pub trait IssueModal {
    fn epic_id_value(&self) -> Option<u32>;

    fn epic_state(&self) -> &StyledSelectState;

    fn update_states(&mut self, msg: &Msg, orders: &mut impl Orders<Msg>);
}

#[derive(Debug, Default)]
pub struct Modals {
    // issue
    pub add_issue: Option<crate::modals::issues_create::Model>,
    pub edit_issue: Option<crate::modals::issues_edit::Model>,
    // epic
    pub delete_epic: Option<crate::modals::epics_delete::Model>,
    pub edit_epic: Option<crate::modals::epics_edit::Model>,

    pub delete_issue_confirm: Option<crate::modals::issues_delete::Model>,
    pub delete_comment_confirm: Option<crate::modals::comments_delete::Model>,
    pub time_tracking: Option<crate::modals::time_tracking::Model>,
    pub delete_issue_status_modal: Option<crate::modals::issue_statuses_delete::Model>,
    #[cfg(debug_assertions)]
    pub debug_modal: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModalType {
    // issue
    AddIssue(Option<i32>),
    EditIssue(Option<i32>),
    DeleteIssueConfirm(Option<i32>),
    // epic
    DeleteEpic(Option<i32>),
    EditEpic(Option<i32>),

    DeleteCommentConfirm(Option<i32>),
    TimeTracking(Option<i32>),
    DeleteIssueStatusModal(Option<i32>),
    #[cfg(debug_assertions)]
    DebugModal(Option<i32>),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct CommentForm {
    pub id: Option<CommentId>,
    pub body: String,
    pub creating: bool,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Page {
    Project,
    // epic
    DeleteEpic(EpicId),
    EditEpic(EpicId),
    // issue
    EditIssue(EpicId),
    AddIssue,
    IssuesAndFilters,
    // settings
    ProjectSettings,
    // auth
    SignIn,
    SignUp,
    Invite,
    Users,
    Profile,
    Reports,
}

impl Page {
    #[inline(always)]
    pub fn to_path(self) -> String {
        match self {
            Page::Project => "/board".to_string(),
            Page::DeleteEpic(id) => format!("/delete-epic/{id}", id = id),
            Page::EditEpic(id) => format!("/edit-epic/{id}", id = id),
            Page::EditIssue(id) => format!("/issues/{id}", id = id),
            Page::AddIssue => "/add-issue".to_string(),
            Page::IssuesAndFilters => "/issues-and-filters".to_string(),
            Page::ProjectSettings => "/project-settings".to_string(),
            Page::SignIn => "/login".to_string(),
            Page::SignUp => "/register".to_string(),
            Page::Invite => "/invite".to_string(),
            Page::Users => "/users".to_string(),
            Page::Profile => "/profile".to_string(),
            Page::Reports => "/reports".to_string(),
        }
    }

    pub fn build_content(&self) -> PageContent {
        match self {
            Page::Project
            | Page::DeleteEpic(..)
            | Page::EditEpic(..)
            | Page::EditIssue(_)
            | Page::AddIssue => PageContent::Project(Box::new(ProjectPage::default())),
            //
            Page::SignIn => PageContent::SignIn(Box::new(SignInPage::default())),
            Page::SignUp => PageContent::SignUp(Box::new(SignUpPage::default())),
            Page::Invite => PageContent::Invite(Box::new(InvitePage::default())),
            Page::Users => PageContent::Users(Box::new(UsersPage::default())),
            Page::Reports => PageContent::Reports(Box::new(ReportsPage::default())),
            // for those which requires additional data
            _ => PageContent::Project(Box::new(ProjectPage::default())),
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

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
#[repr(C)]
pub enum InvitationFormState {
    Initial = 1,
    Sent = 2,
    Succeed = 3,
    Failed = 4,
}

impl Default for InvitationFormState {
    fn default() -> Self {
        InvitationFormState::Initial
    }
}

#[macro_export]
macro_rules! match_page {
    ($model: ident, $ty: ident) => {
        match &$model.page_content {
            crate::model::PageContent::$ty(page) => page,
            _ => return,
        }
    };
    ($model: ident, $ty: ident; Empty) => {
        match &$model.page_content {
            crate::model::PageContent::$ty(page) => page,
            _ => return Node::Empty,
        }
    };
}
#[macro_export]
macro_rules! match_page_mut {
    ($model: ident, $ty: ident) => {
        match &mut $model.page_content {
            PageContent::$ty(page) => page,
            _ => return,
        }
    };
}

#[derive(Debug)]
pub enum PageContent {
    SignIn(Box<SignInPage>),
    SignUp(Box<SignUpPage>),
    Project(Box<ProjectPage>),
    ProjectSettings(Box<ProjectSettingsPage>),
    Invite(Box<InvitePage>),
    Users(Box<UsersPage>),
    Profile(Box<ProfilePage>),
    Reports(Box<ReportsPage>),
    IssuesAndFilters(Box<IssuesAndFiltersPage>),
}

#[derive(Debug)]
pub struct Model {
    // pub hi_worker: Worker,
    pub ws: Option<WebSocket>,
    pub ws_queue: Vec<WsMsg>,
    pub host_url: String,
    pub ws_url: String,
    pub access_token: Option<Uuid>,
    pub about_tooltip_visible: bool,
    pub messages_tooltip_visible: bool,

    // mapped
    pub comments_by_project_id: HashMap<ProjectId, Vec<Comment>>,

    // forms
    pub project_form: Option<UpdateProjectForm>,
    pub issue_form: Option<CreateIssueForm>,
    pub comment_form: Option<CreateCommentForm>,

    // modals
    modals_stack: Vec<ModalType>,
    pub modals: Modals,

    // pages
    pub page: Page,
    pub page_content: PageContent,

    pub current_user_project: Option<UserProject>,

    // issues
    issues: Vec<Issue>,
    pub issues_by_id: HashMap<EpicId, Issue>,

    // users
    pub user: Option<User>,
    pub users: Vec<User>,
    pub users_by_id: HashMap<UserId, User>,

    // user settings
    pub user_settings: Option<UserSetting>,

    // comments
    pub comments: Vec<Comment>,
    pub comments_by_id: HashMap<CommentId, Comment>,

    // issue_statuses
    pub issue_statuses: Vec<IssueStatus>,
    pub issue_statuses_by_id: HashMap<IssueStatusId, IssueStatus>,
    pub issue_statuses_by_name: HashMap<String, IssueStatus>,

    // messages
    pub messages: Vec<Message>,

    // user_projects
    pub user_projects: Vec<UserProject>,

    // projects
    pub project: Option<Project>,
    pub projects: Vec<Project>,

    // epics
    pub epics: Vec<Epic>,
    pub epics_by_id: HashMap<EpicId, Epic>,

    pub key_triggers: std::rc::Rc<std::cell::RefCell<HashMap<char, Box<dyn BuildMsg>>>>,
    pub distinct_key_up: crate::shared::on_event::Distinct,

    pub show_extras: bool,
}

impl Model {
    pub fn new(host_url: String, ws_url: String, page: Page) -> Self {
        Self {
            ws: None,
            ws_queue: vec![],
            access_token: None,
            user: None,
            issue_form: None,
            project_form: None,
            comment_form: None,
            comments_by_project_id: Default::default(),
            page_content: page.build_content(),
            page,
            host_url,
            ws_url,
            project: None,
            current_user_project: None,
            about_tooltip_visible: false,
            messages_tooltip_visible: false,
            issues: vec![],
            users: vec![],
            users_by_id: Default::default(),
            user_settings: None,
            comments: vec![],
            comments_by_id: Default::default(),
            issue_statuses: vec![],
            issue_statuses_by_id: Default::default(),
            issue_statuses_by_name: Default::default(),
            messages: vec![],
            user_projects: vec![],
            projects: vec![],
            epics: vec![],
            issues_by_id: Default::default(),
            show_extras: false,
            epics_by_id: Default::default(),
            modals_stack: vec![],
            modals: Default::default(),
            key_triggers: std::rc::Rc::new(std::cell::RefCell::new(HashMap::with_capacity(20))),
            distinct_key_up: crate::shared::on_event::distinct(),
        }
    }

    #[inline(always)]
    pub fn issues(&self) -> &[Issue] {
        &self.issues
    }

    #[inline(always)]
    pub fn issues_mut(&mut self) -> &mut Vec<Issue> {
        &mut self.issues
    }

    #[inline(always)]
    pub fn issue_statuses(&self) -> &[IssueStatus] {
        &self.issue_statuses
    }

    #[inline(always)]
    pub fn epics(&self) -> &[Epic] {
        &self.epics
    }

    #[inline(always)]
    pub fn user(&self) -> &Option<User> {
        &self.user
    }

    #[inline(always)]
    pub fn user_id(&self) -> Option<UserId> {
        self.user.as_ref().map(|u| u.id)
    }

    #[inline(always)]
    pub fn user_name(&self) -> Option<&str> {
        self.user.as_ref().map(|u| u.name())
    }

    #[inline(always)]
    pub fn project_id(&self) -> Option<ProjectId> {
        self.project.as_ref().map(|p| p.id)
    }

    #[inline(always)]
    pub fn project_name(&self) -> Option<&str> {
        self.project.as_ref().map(|u| u.name())
    }

    pub fn current_user_role(&self) -> UserRole {
        self.current_user_project
            .as_ref()
            .map(|up| up.role)
            .unwrap_or_default()
    }

    pub fn epic_issue_ids(&self, epic_id: EpicId) -> Vec<IssueId> {
        self.issues()
            .iter()
            .filter_map(|issue| {
                if issue.epic_id == Some(epic_id) {
                    Some(issue.id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn modals(&self) -> &Modals {
        &self.modals
    }

    pub fn modals_mut(&mut self) -> &mut Modals {
        &mut self.modals
    }

    pub fn modal_stack(&self) -> &[ModalType] {
        &self.modals_stack
    }

    pub fn modal_stack_mut(&mut self) -> &mut Vec<ModalType> {
        &mut self.modals_stack
    }
}
