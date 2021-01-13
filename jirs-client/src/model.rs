use {
    crate::{
        pages::{
            invite_page::InvitePage, profile_page::model::ProfilePage,
            project_page::model::ProjectPage, project_settings_page::ProjectSettingsPage,
            reports_page::model::ReportsPage, sign_in_page::model::SignInPage,
            sign_up_page::model::SignUpPage, users_page::model::UsersPage,
        },
        shared::styled_select::StyledSelectState,
        Msg,
    },
    jirs_data::*,
    seed::{app::Orders, browser::web_socket::WebSocket},
    serde::{Deserialize, Serialize},
    std::collections::hash_map::HashMap,
    uuid::Uuid,
};

pub trait IssueModal {
    fn epic_id_value(&self) -> Option<u32>;

    fn epic_state(&self) -> &StyledSelectState;

    fn update_states(&mut self, msg: &Msg, orders: &mut impl Orders<Msg>);
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum ModalType {
    AddIssue(Box<crate::modals::issues_create::Model>),
    EditIssue(IssueId, Box<crate::modals::issues_edit::Model>),
    DeleteIssueConfirm(IssueId),
    DeleteCommentConfirm(CommentId),
    TimeTracking(IssueId),
    DeleteIssueStatusModal(Box<crate::modals::issue_statuses_delete::Model>),
    #[cfg(debug_assertions)]
    DebugModal,
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
    EditIssue(IssueId),
    AddIssue,
    ProjectSettings,
    SignIn,
    SignUp,
    Invite,
    Users,
    Profile,
    Reports,
}

impl Page {
    pub fn to_path(self) -> String {
        match self {
            Page::Project => "/board".to_string(),
            Page::EditIssue(id) => format!("/issues/{id}", id = id),
            Page::AddIssue => "/add-issue".to_string(),
            Page::ProjectSettings => "/project-settings".to_string(),
            Page::SignIn => "/login".to_string(),
            Page::SignUp => "/register".to_string(),
            Page::Invite => "/invite".to_string(),
            Page::Users => "/users".to_string(),
            Page::Profile => "/profile".to_string(),
            Page::Reports => "/reports".to_string(),
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
    pub modals: Vec<ModalType>,

    // pages
    pub page: Page,
    pub page_content: PageContent,

    pub project: Option<Project>,

    pub current_user_project: Option<UserProject>,

    pub issues: Vec<Issue>,
    pub issues_by_id: HashMap<IssueId, Issue>,

    pub user: Option<User>,
    pub users: Vec<User>,
    pub users_by_id: HashMap<UserId, User>,

    pub comments: Vec<Comment>,

    pub issue_statuses: Vec<IssueStatus>,
    pub issue_statuses_by_id: HashMap<IssueStatusId, IssueStatus>,
    pub issue_statuses_by_name: HashMap<String, IssueStatus>,

    pub messages: Vec<Message>,
    pub user_projects: Vec<UserProject>,
    pub projects: Vec<Project>,
    pub epics: Vec<Epic>,

    pub show_extras: bool,
}

impl Model {
    pub fn new(host_url: String, ws_url: String) -> Self {
        Self {
            ws: None,
            ws_queue: vec![],
            access_token: None,
            user: None,
            issue_form: None,
            project_form: None,
            comment_form: None,
            comments_by_project_id: Default::default(),
            page: Page::Project,
            host_url,
            ws_url,
            page_content: PageContent::Project(Box::new(ProjectPage::default())),
            modals: vec![],
            project: None,
            current_user_project: None,
            about_tooltip_visible: false,
            messages_tooltip_visible: false,
            issues: vec![],
            users: vec![],
            users_by_id: Default::default(),
            comments: vec![],
            issue_statuses: vec![],
            issue_statuses_by_id: Default::default(),
            issue_statuses_by_name: Default::default(),
            messages: vec![],
            user_projects: vec![],
            projects: vec![],
            epics: vec![],
            issues_by_id: Default::default(),
            show_extras: false,
        }
    }

    pub fn current_user_role(&self) -> UserRole {
        self.current_user_project
            .as_ref()
            .map(|up| up.role)
            .unwrap_or_default()
    }
}
