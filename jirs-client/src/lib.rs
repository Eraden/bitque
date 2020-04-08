use std::sync::RwLock;

use seed::fetch::FetchObject;
use seed::{prelude::*, *};

use jirs_data::{IssueStatus, WsMsg};

use crate::api::send_ws_msg;
use crate::model::{ModalType, Model, Page};
use crate::shared::read_auth_token;
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

pub type UserId = i32;
pub type IssueId = i32;
pub type AvatarFilterActive = bool;

#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum FieldId {
    // edit issue
    IssueTypeEditModalTop,
    // project boards
    TextFilterBoard,
    //
    CopyButtonLabel,
    // add issue
    IssueTypeAddIssueModal,
    SummaryAddIssueModal,
    DescriptionAddIssueModal,
    ReporterAddIssueModal,
    AssigneesAddIssueModal,
    IssuePriorityAddIssueModal,
}

impl std::fmt::Display for FieldId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldId::IssueTypeEditModalTop => f.write_str("issueTypeEditModalTop"),
            FieldId::TextFilterBoard => f.write_str("textFilterBoard"),
            FieldId::CopyButtonLabel => f.write_str("copyButtonLabel"),
            FieldId::IssueTypeAddIssueModal => f.write_str("issueTypeAddIssueModal"),
            FieldId::SummaryAddIssueModal => f.write_str("summaryAddIssueModal"),
            FieldId::DescriptionAddIssueModal => f.write_str("descriptionAddIssueModal"),
            FieldId::ReporterAddIssueModal => f.write_str("reporterAddIssueModal"),
            FieldId::AssigneesAddIssueModal => f.write_str("assigneesAddIssueModal"),
            FieldId::IssuePriorityAddIssueModal => f.write_str("issuePriorityAddIssueModal"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum FieldChange {
    LinkCopied(FieldId, bool),
}

#[derive(Clone, Debug)]
pub enum Msg {
    NoOp,

    // Auth Token
    AuthTokenStored,
    AuthTokenErased,

    StyledSelectChanged(FieldId, StyledSelectChange),

    ChangePage(model::Page),
    CurrentProjectResult(FetchObject<String>),
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
    IssueDropZone(IssueStatus),

    // inputs
    InputChanged(FieldId, String),

    // issues
    AddIssue,
    DeleteIssue(IssueId),

    // modals
    ModalOpened(ModalType),
    ModalDropped,
    ModalChanged(FieldChange),

    WsMsg(jirs_data::WsMsg),
}

fn update(msg: Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    if cfg!(debug_assertions) {
        log!(msg);
    }
    match &msg {
        Msg::ChangePage(page) => {
            model.page = page.clone();
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
