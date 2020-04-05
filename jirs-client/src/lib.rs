#[macro_use]
extern crate lazy_static;

use seed::fetch::FetchObject;
use seed::{prelude::*, *};

use jirs_data::IssueStatus;

use crate::api::ws;
use crate::model::{ModalType, Model, Page};
use crate::shared::styled_select::StyledSelectChange;

mod api;
mod api_handlers;
mod login;
mod modal;
mod model;
mod project;
mod project_settings;
mod register;
mod shared;

pub type UserId = i32;
pub type IssueId = i32;
pub type AvatarFilterActive = bool;

#[derive(Clone, Debug)]
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
}

#[derive(Clone, Debug)]
pub enum FieldChange {
    LinkCopied(FieldId, bool),
}

#[derive(Clone, Debug)]
pub enum Msg {
    NoOp,
    StyledSelectChanged(FieldId, StyledSelectChange),

    ChangePage(model::Page),
    CurrentProjectResult(FetchObject<String>),
    CurrentUserResult(FetchObject<String>),
    InternalFailure(String),
    ToggleAboutTooltip,

    // project
    ProjectTextFilterChanged(String),
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
    IssueUpdateResult(FetchObject<String>),
    IssueDeleteResult(FetchObject<String>),
    DeleteIssue(IssueId),

    // modals
    ModalOpened(ModalType),
    ModalDropped,
    ModalChanged(FieldChange),
}

fn update(msg: Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    if cfg!(debug_assertions) {
        log!(msg);
    }
    match msg {
        Msg::ChangePage(page) => {
            model.page = page;
        }
        _ => (),
    }
    crate::shared::update(&msg, model, orders);
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

#[wasm_bindgen]
pub fn set_host_url(url: String) {
    unsafe {
        HOST_URL = url;
    }
}

fn after_mount(_url: Url, _orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    ws();
    let model = Model::default();
    AfterMount::new(model).url_handling(UrlHandling::None)
}

#[wasm_bindgen]
pub fn render() {
    App::builder(update, view)
        .routes(routes)
        .after_mount(after_mount)
        .build_and_start();
}
