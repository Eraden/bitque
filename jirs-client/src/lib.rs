use std::sync::RwLock;

use seed::{prelude::*, *};

use jirs_data::*;

use crate::api::send_ws_msg;
use crate::model::{ModalType, Model, Page};
use crate::shared::styled_editor::Mode as TabMode;
use crate::shared::styled_select::StyledSelectChange;

mod api;
mod modal;
mod model;
mod project;
mod project_settings;
mod shared;
mod sign_in;
mod sign_up;
mod validations;
mod ws;

pub type AvatarFilterActive = bool;
pub type AppType = App<Msg, Model, Node<Msg>>;

#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum EditIssueModalSection {
    Issue(IssueFieldId),
    Comment(CommentFieldId),
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum FieldId {
    SignIn(SignInFieldId),
    SignUp(SignUpFieldId),
    // issue
    AddIssueModal(IssueFieldId),
    EditIssueModal(EditIssueModalSection),
    // project boards
    TextFilterBoard,
    CopyButtonLabel,

    ProjectSettings(ProjectFieldId),
}

impl std::fmt::Display for FieldId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldId::EditIssueModal(sub) => match sub {
                EditIssueModalSection::Issue(IssueFieldId::Type) => {
                    f.write_str("issueTypeEditModalTop")
                }
                EditIssueModalSection::Issue(IssueFieldId::Title) => {
                    f.write_str("titleIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::Description) => {
                    f.write_str("descriptionIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::Status) => {
                    f.write_str("statusIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::Assignees) => {
                    f.write_str("assigneesIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::Reporter) => {
                    f.write_str("reporterIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::Priority) => {
                    f.write_str("priorityIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::Estimate) => {
                    f.write_str("estimateIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::TimeSpend) => {
                    f.write_str("timeSpendIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::TimeRemaining) => {
                    f.write_str("timeRemainingIssueEditModal")
                }
                EditIssueModalSection::Comment(CommentFieldId::Body) => {
                    f.write_str("editIssue-commentBody")
                }
                EditIssueModalSection::Issue(IssueFieldId::ListPosition) => {
                    f.write_str("editIssue-listPosition")
                }
            },
            FieldId::AddIssueModal(sub) => match sub {
                IssueFieldId::Type => f.write_str("issueTypeAddIssueModal"),
                IssueFieldId::Title => f.write_str("summaryAddIssueModal"),
                IssueFieldId::Description => f.write_str("descriptionAddIssueModal"),
                IssueFieldId::Reporter => f.write_str("reporterAddIssueModal"),
                IssueFieldId::Assignees => f.write_str("assigneesAddIssueModal"),
                IssueFieldId::Priority => f.write_str("issuePriorityAddIssueModal"),
                IssueFieldId::Status => f.write_str("addIssueModal-status"),
                IssueFieldId::Estimate => f.write_str("addIssueModal-estimate"),
                IssueFieldId::TimeSpend => f.write_str("addIssueModal-timeSpend"),
                IssueFieldId::TimeRemaining => f.write_str("addIssueModal-timeRemaining"),
                IssueFieldId::ListPosition => f.write_str("addIssueModal-listPosition"),
            },
            FieldId::TextFilterBoard => f.write_str("textFilterBoard"),
            FieldId::CopyButtonLabel => f.write_str("copyButtonLabel"),
            FieldId::ProjectSettings(sub) => match sub {
                ProjectFieldId::Name => f.write_str("projectSettings-name"),
                ProjectFieldId::Url => f.write_str("projectSettings-url"),
                ProjectFieldId::Description => f.write_str("projectSettings-description"),
                ProjectFieldId::Category => f.write_str("projectSettings-category"),
            },
            FieldId::SignIn(sub) => match sub {
                SignInFieldId::Email => f.write_str("login-email"),
                SignInFieldId::Username => f.write_str("login-username"),
                SignInFieldId::Token => f.write_str("login-token"),
            },
            FieldId::SignUp(sub) => match sub {
                SignUpFieldId::Username => f.write_str("signUp-email"),
                SignUpFieldId::Email => f.write_str("signUp-username"),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FieldChange {
    LinkCopied(FieldId, bool),
    TabChanged(FieldId, TabMode),
    ToggleCommentForm(FieldId, bool),
    EditComment(FieldId, i32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    NoOp,
    GlobalKeyDown {
        key: String,
        shift: bool,
        ctrl: bool,
        alt: bool,
    },

    // Auth Token
    AuthTokenStored,
    AuthTokenErased,
    SignInRequest,
    BindClientRequest,

    // sign up
    SignUpRequest,

    StyledSelectChanged(FieldId, StyledSelectChange),

    ChangePage(model::Page),
    InternalFailure(String),
    ToggleAboutTooltip,

    // project
    ProjectAvatarFilterChanged(UserId, AvatarFilterActive),
    ProjectToggleOnlyMy,
    ProjectToggleRecentlyUpdated,
    ProjectClearFilters,
    ProjectSaveChanges,

    // dragging
    IssueDragStarted(IssueId),
    IssueDragStopped(IssueId),
    DragLeave(IssueId),
    ExchangePosition(IssueId),
    IssueDragOverStatus(IssueStatus),
    IssueDropZone(IssueStatus),
    UnlockDragOver,

    // inputs
    InputChanged(FieldId, String),

    // issues
    AddIssue,
    DeleteIssue(IssueId),

    // comments
    SaveComment,
    DeleteComment(CommentId),

    // modals
    ModalOpened(Box<ModalType>),
    ModalDropped,
    ModalChanged(FieldChange),

    WsMsg(jirs_data::WsMsg),
}

fn update(msg: Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    if msg == Msg::NoOp {
        return;
    }
    if cfg!(debug_assertions) {
        log!(msg);
    }
    match &msg {
        Msg::AuthTokenStored => {
            seed::push_route(vec!["dashboard"]);
            orders.skip().send_msg(Msg::ChangePage(Page::Project));
            authorize_or_redirect();
            return;
        }
        Msg::AuthTokenErased => {
            seed::push_route(vec!["login"]);
            orders.skip().send_msg(Msg::ChangePage(Page::SignIn));
            authorize_or_redirect();
            return;
        }
        Msg::ChangePage(page) => {
            model.page = *page;
        }
        Msg::ToggleAboutTooltip => {
            model.about_tooltip_visible = !model.about_tooltip_visible;
        }
        _ => (),
    }
    crate::ws::update(&msg, model, orders);
    crate::modal::update(&msg, model, orders);
    match model.page {
        Page::Project | Page::AddIssue | Page::EditIssue(..) => project::update(msg, model, orders),
        Page::ProjectSettings => project_settings::update(msg, model, orders),
        Page::SignIn => sign_in::update(msg, model, orders),
        Page::SignUp => sign_up::update(msg, model, orders),
    }
    if cfg!(debug_assertions) {
        // debug!(model);
    }
}

fn view(model: &model::Model) -> Node<Msg> {
    match model.page {
        Page::Project | Page::AddIssue => project::view(model),
        Page::EditIssue(_id) => project::view(model),
        Page::ProjectSettings => project_settings::view(model),
        Page::SignIn => sign_in::view(model),
        Page::SignUp => sign_up::view(model),
    }
}

fn routes(url: Url) -> Option<Msg> {
    if url.path.is_empty() {
        return Some(Msg::ChangePage(model::Page::Project));
    }

    match url.path[0].as_ref() {
        "board" => Some(Msg::ChangePage(model::Page::Project)),
        "issues" => match url.path.get(1).as_ref().map(|s| s.parse::<i32>()) {
            Some(Ok(id)) => Some(Msg::ChangePage(model::Page::EditIssue(id))),
            _ => None,
        },
        "add-issue" => Some(Msg::ChangePage(Page::AddIssue)),
        "project-settings" => Some(Msg::ChangePage(model::Page::ProjectSettings)),
        "login" => Some(Msg::ChangePage(model::Page::SignIn)),
        "register" => Some(Msg::ChangePage(model::Page::SignUp)),
        _ => Some(Msg::ChangePage(model::Page::Project)),
    }
}

pub static mut HOST_URL: String = String::new();
pub static mut APP: Option<RwLock<App<Msg, Model, Node<Msg>>>> = None;

#[wasm_bindgen]
pub fn set_host_url(url: String) {
    unsafe {
        HOST_URL = url;
    }
}

#[wasm_bindgen]
pub fn handle_ws_message(value: &wasm_bindgen::JsValue) {
    let a = js_sys::Uint8Array::new(value);
    let mut v = Vec::new();
    for idx in 0..a.length() {
        v.push(a.get_index(idx));
    }
    if let Ok(msg) = bincode::deserialize(v.as_slice()) {
        ws::handle(msg);
    };
}

#[wasm_bindgen]
pub fn reconnected() {
    authorize_or_redirect();
}

#[wasm_bindgen]
extern "C" {
    pub fn send_bin_code(data: wasm_bindgen::JsValue);
}

#[wasm_bindgen]
pub fn render() {
    seed::set_interval(
        Box::new(|| {
            let binary = bincode::serialize(&jirs_data::WsMsg::Ping).unwrap();
            let data = JsValue::from_serde(&binary).unwrap();
            send_bin_code(data);
        }) as Box<dyn Fn()>,
        5000,
    );

    if let Some(body) = seed::html_document().body() {
        use wasm_bindgen::JsCast;

        let body = body.dyn_ref::<web_sys::HtmlBodyElement>().unwrap().clone();
        let key_up_closure =
            wasm_bindgen::closure::Closure::wrap(Box::new(|event: web_sys::KeyboardEvent| {
                if let Some(Ok(app)) = unsafe { APP.as_mut().map(|app| app.write()) } {
                    let msg = Msg::GlobalKeyDown {
                        key: event.key(),
                        shift: event.shift_key(),
                        ctrl: event.ctrl_key(),
                        alt: event.alt_key(),
                    };
                    app.update(msg);
                }
            })
                as Box<dyn Fn(web_sys::KeyboardEvent)>);
        body.add_event_listener_with_callback("keyup", key_up_closure.as_ref().unchecked_ref())
            .unwrap();
        key_up_closure.forget();
    }

    let app = seed::App::builder(update, view)
        .routes(routes)
        .build_and_start();

    authorize_or_redirect();

    let cell_app = std::sync::RwLock::new(app);
    unsafe {
        APP = Some(cell_app);
    };
}

#[inline]
fn authorize_or_redirect() {
    match crate::shared::read_auth_token() {
        Ok(token) => {
            send_ws_msg(WsMsg::AuthorizeRequest(token));
        }
        Err(..) => {
            let pathname = seed::document().location().unwrap().pathname().unwrap();
            match pathname.as_str() {
                "/login" | "/register" => {}
                _ => {
                    seed::push_route(vec!["login"]);
                }
            };
        }
    };
}
