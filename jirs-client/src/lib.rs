use std::sync::RwLock;

use seed::{prelude::*, *};

use jirs_data::*;

use crate::api::send_ws_msg;
use crate::model::{ModalType, Model, Page};
use crate::shared::read_auth_token;
use crate::shared::styled_editor::Mode as TabMode;
use crate::shared::styled_select::StyledSelectChange;

mod api;
mod login;
mod modal;
mod model;
mod project;
mod project_settings;
mod register;
mod shared;
mod ws;

pub type AvatarFilterActive = bool;
pub type AppType = App<Msg, Model, Node<Msg>>;

#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum EditIssueModalFieldId {
    IssueType,
    Title,
    Description,
    Status,
    Assignees,
    Reporter,
    Priority,
    Estimate,
    TimeSpend,
    TimeRemaining,
    // comment
    CommentBody,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum AddIssueModalFieldId {
    IssueType,
    Summary,
    Description,
    Reporter,
    Assignees,
    Priority,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum ProjectSettingsFieldId {
    Name,
    Url,
    Description,
    Category,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum FieldId {
    // issue
    AddIssueModal(AddIssueModalFieldId),
    EditIssueModal(EditIssueModalFieldId),
    // project boards
    TextFilterBoard,
    CopyButtonLabel,

    ProjectSettings(ProjectSettingsFieldId),
}

impl std::fmt::Display for FieldId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldId::EditIssueModal(sub) => match sub {
                EditIssueModalFieldId::IssueType => f.write_str("issueTypeEditModalTop"),
                EditIssueModalFieldId::Title => f.write_str("titleIssueEditModal"),
                EditIssueModalFieldId::Description => f.write_str("descriptionIssueEditModal"),
                EditIssueModalFieldId::Status => f.write_str("statusIssueEditModal"),
                EditIssueModalFieldId::Assignees => f.write_str("assigneesIssueEditModal"),
                EditIssueModalFieldId::Reporter => f.write_str("reporterIssueEditModal"),
                EditIssueModalFieldId::Priority => f.write_str("priorityIssueEditModal"),
                EditIssueModalFieldId::Estimate => f.write_str("estimateIssueEditModal"),
                EditIssueModalFieldId::TimeSpend => f.write_str("timeSpendIssueEditModal"),
                EditIssueModalFieldId::TimeRemaining => f.write_str("timeRemainingIssueEditModal"),
                EditIssueModalFieldId::CommentBody => f.write_str("editIssue-commentBody"),
            },
            FieldId::AddIssueModal(sub) => match sub {
                AddIssueModalFieldId::IssueType => f.write_str("issueTypeAddIssueModal"),
                AddIssueModalFieldId::Summary => f.write_str("summaryAddIssueModal"),
                AddIssueModalFieldId::Description => f.write_str("descriptionAddIssueModal"),
                AddIssueModalFieldId::Reporter => f.write_str("reporterAddIssueModal"),
                AddIssueModalFieldId::Assignees => f.write_str("assigneesAddIssueModal"),
                AddIssueModalFieldId::Priority => f.write_str("issuePriorityAddIssueModal"),
            },
            FieldId::TextFilterBoard => f.write_str("textFilterBoard"),
            FieldId::CopyButtonLabel => f.write_str("copyButtonLabel"),
            FieldId::ProjectSettings(sub) => match sub {
                ProjectSettingsFieldId::Name => f.write_str("projectSettings-name"),
                ProjectSettingsFieldId::Url => f.write_str("projectSettings-url"),
                ProjectSettingsFieldId::Description => f.write_str("projectSettings-description"),
                ProjectSettingsFieldId::Category => f.write_str("projectSettings-category"),
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

    StyledSelectChanged(FieldId, StyledSelectChange),

    ChangePage(model::Page),
    InternalFailure(String),
    ToggleAboutTooltip,

    // project
    ProjectAvatarFilterChanged(UserId, AvatarFilterActive),
    ProjectToggleOnlyMy,
    ProjectToggleRecentlyUpdated,
    ProjectClearFilters,

    // dragging
    IssueDragStarted(IssueId),
    IssueDragStopped(IssueId),
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
    ModalOpened(ModalType),
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
        Msg::ChangePage(page) => {
            model.page = page.clone();
        }
        Msg::ToggleAboutTooltip => {
            model.about_tooltip_visible = !model.about_tooltip_visible;
        }
        _ => (),
    }
    crate::ws::update(&msg, model, orders);
    crate::modal::update(&msg, model, orders);
    match model.page {
        Page::Project | Page::AddIssue => project::update(msg, model, orders),
        Page::EditIssue(_id) => project::update(msg, model, orders),
        Page::ProjectSettings => project_settings::update(msg, model, orders),
        Page::Login => login::update(msg, model, orders),
        Page::Register => register::update(msg, model, orders),
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
        Page::Login => login::view(model),
        Page::Register => register::view(model),
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
        "login" => Some(Msg::ChangePage(model::Page::Login)),
        "register" => Some(Msg::ChangePage(model::Page::Register)),
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
    match bincode::deserialize(v.as_slice()) {
        Ok(msg) => {
            ws::handle(msg);
        }
        _ => (),
    };
}

#[wasm_bindgen]
pub fn reconnected() {
    if let Ok(uuid) = read_auth_token() {
        send_ws_msg(WsMsg::AuthorizeRequest(uuid));
    }
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

    match crate::shared::read_auth_token() {
        Ok(uuid) => send_ws_msg(WsMsg::AuthorizeRequest(uuid)),
        _ => (),
    };

    let cell_app = std::sync::RwLock::new(app);
    unsafe {
        APP = Some(cell_app);
    };
}
