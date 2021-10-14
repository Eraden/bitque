#![feature(type_ascription, trait_alias, drain_filter)]

pub use changes::*;
pub use components::*;
pub use fields::*;
pub use images::*;
use jirs_data::msg::WsMsgSession;
use jirs_data::*;
use seed::prelude::*;
use web_sys::File;

use crate::components::styled_date_time_input::StyledDateTimeChanged;
use crate::components::styled_rte::RteMsg;
use crate::components::styled_select::StyledSelectChanged;
use crate::components::styled_tooltip;
use crate::components::styled_tooltip::{TooltipVariant as StyledTooltip, TooltipVariant};
use crate::modals::DebugMsg;
use crate::model::{ModalType, Model, Page};
use crate::pages::issues_and_filters::IssuesAndFiltersMsg;
use crate::pages::sign_in_page::SignInMsg;
use crate::shared::go_to_login;
use crate::ws::{flush_queue, open_socket, read_incoming, send_ws_msg};

// use crate::shared::styled_rte::RteMsg;

mod changes;
mod components;
mod fields;
mod images;
mod location;
mod modals;
mod model;
mod pages;
mod shared;
pub mod validations;
mod ws;

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
#[repr(C)]
pub enum ResourceKind {
    Issue,
    IssueStatus,
    Epic,
    Project,
    User,
    UserSetting,
    UserProject,
    Message,
    Comment,
    Auth,
}

#[derive(Debug)]
#[repr(C)]
pub enum OperationKind {
    ListLoaded,
    SingleLoaded,
    SingleCreated,
    SingleRemoved,
    SingleModified,
}

pub trait BuildMsg: std::fmt::Debug {
    fn build(&self) -> Msg;
}

#[derive(Debug)]
pub enum Msg {
    #[cfg(debug_assertions)]
    Debug(DebugMsg),
    PageChanged(PageChanged),
    ChangePage(model::Page),

    Rte(FieldId, RteMsg),

    UserChanged(Option<User>),
    ProjectChanged(Option<Project>),

    StyledSelectChanged(FieldId, StyledSelectChanged),
    StyledDateTimeInputChanged(FieldId, StyledDateTimeChanged),
    InternalFailure(String),
    ToggleTooltip(StyledTooltip),

    // Auth Token
    AuthTokenStored,
    AuthTokenErased,
    SignInRequest,
    BindClientRequest,
    InvalidPair,
    SignIn(SignInMsg),

    // users
    InviteRequest,
    InviteRevokeRequest(InvitationId),
    InviteApproveRequest(InvitationId),
    InvitedUserRemove(UserId),

    // sign up
    SignUpRequest,

    // project
    ProjectAvatarFilterChanged(UserId, AvatarFilterActive),
    ProjectToggleOnlyMy,
    ProjectToggleRecentlyUpdated,
    ProjectClearFilters,

    // issues and filters
    IssuesAndFilters(IssuesAndFiltersMsg),

    // inputs
    StrInputChanged(FieldId, String),

    U32InputChanged(FieldId, u32),
    FileInputChanged(FieldId, Vec<File>),
    // Rte(FieldId, RteMsg),

    // issues
    AddIssue,
    DeleteIssue(EpicId),
    SetActiveIssue(Option<IssueId>),

    // epics
    AddEpic,
    DeleteEpic,
    UpdateEpic,
    TransformEpic,

    // issue statuses
    DeleteIssueStatus(IssueStatusId),

    // comments
    SaveComment,
    DeleteComment(CommentId),

    // profile
    AvatarUpdateFetched(String),

    // modals
    ModalOpened(ModalType),
    ModalDropped,
    ModalChanged(FieldChange),

    // popups
    MessageSeen(MessageId),
    MessageInvitationApproved(InvitationToken),
    MessageInvitationDismiss(InvitationToken),

    // WebSocket
    WebSocketChange(WebSocketChanged),

    // resource changes
    ResourceChanged(ResourceKind, OperationKind, Option<i32>),
}

fn update(msg: Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    if model.ws.is_none() {
        open_socket(model, orders);
    }

    let msg = match msg {
        Msg::WebSocketChange(change) => match change {
            WebSocketChanged::WebSocketOpened => {
                flush_queue(model, orders);
                send_ws_msg(WsMsg::Ping, model.ws.as_ref(), orders);
                authorize_or_redirect(model, orders);
                orders.skip();
                return;
            }
            WebSocketChanged::SendPing => {
                send_ws_msg(WsMsg::Ping, model.ws.as_ref(), orders);
                orders.skip();
                return;
            }
            WebSocketChanged::WebSocketMessage(incoming) => {
                orders.perform_cmd(read_incoming(incoming));
                orders.skip();
                return;
            }
            WebSocketChanged::WsMsg(mut ws_msg) => {
                ws::update(&mut ws_msg, model, orders);
                orders.skip();
                return;
            }
            WebSocketChanged::WebSocketMessageLoaded(v) => {
                match bincode::deserialize(v.as_slice()) {
                    Ok(WsMsg::Ping | WsMsg::Pong) => {
                        orders.skip().perform_cmd(cmds::timeout(300, || {
                            Msg::WebSocketChange(WebSocketChanged::SendPing)
                        }));
                    }
                    Ok(m) => {
                        log::info!("INCOMING {:?}", m);
                        orders
                            .skip()
                            .send_msg(Msg::WebSocketChange(WebSocketChanged::WsMsg(m)));
                    }
                    _ => (),
                };
                return;
            }
            WebSocketChanged::WebSocketClosed => {
                open_socket(model, orders);
                return;
            }
            WebSocketChanged::Bounced(ws_msg) => {
                model.ws_queue.push(ws_msg);
                open_socket(model, orders);
                return;
            }
        },
        _ => msg,
    };

    if cfg!(debug_assertions) {
        log::info!("msg {:?}", msg);
    }

    match &msg {
        Msg::AuthTokenStored => {
            authorize_or_redirect(model, orders);
            // go_to_board(orders);
            return;
        }
        Msg::AuthTokenErased => {
            go_to_login(orders);
            return;
        }
        Msg::ChangePage(page) => {
            model.page = *page;
        }
        Msg::ToggleTooltip(variant) => match variant {
            styled_tooltip::TooltipVariant::About => {
                model.about_tooltip_visible = !model.about_tooltip_visible;
            }
            styled_tooltip::TooltipVariant::Messages => {
                model.messages_tooltip_visible = !model.messages_tooltip_visible;
            }
            styled_tooltip::TooltipVariant::CodeBuilder => {}
            TooltipVariant::TableBuilder => {}
            TooltipVariant::DateTimeBuilder => {}
        },
        _ => (),
    }

    if !matches!(model.page, Page::SignIn | Page::SignUp) {
        use crate::shared::{aside, navbar_left};
        aside::update(&msg, model, orders);
        navbar_left::update(&msg, model, orders);
    }
    crate::modals::update(&msg, model, orders);

    match model.page {
        Page::Project
        | Page::AddIssue
        | Page::EditIssue(..)
        | Page::DeleteEpic(..)
        | Page::EditEpic(..) => pages::project_page::update(msg, model, orders),
        Page::ProjectSettings => pages::project_settings_page::update(msg, model, orders),
        Page::SignIn => pages::sign_in_page::update(msg, model, orders),
        Page::SignUp => pages::sign_up_page::update(msg, model, orders),
        Page::Invite => pages::invite_page::update(msg, model, orders),
        Page::Users => pages::users_page::update(msg, model, orders),
        Page::Profile => pages::profile_page::update(msg, model, orders),
        Page::Reports => pages::reports_page::update(msg, model, orders),
        Page::IssuesAndFilters => pages::issues_and_filters::update(msg, model, orders),
        Page::Epics => pages::epics_page::update(msg, model, orders),
    }
}

fn view(model: &model::Model) -> Node<Msg> {
    model.key_triggers.borrow_mut().clear();

    match model.page {
        Page::Project
        | Page::AddIssue
        | Page::EditIssue(..)
        | Page::DeleteEpic(..)
        | Page::EditEpic(..) => pages::project_page::view(model),
        Page::ProjectSettings => pages::project_settings_page::view(model),
        Page::SignIn => pages::sign_in_page::view(model),
        Page::SignUp => pages::sign_up_page::view(model),
        Page::Invite => pages::invite_page::view(model),
        Page::Users => pages::users_page::view(model),
        Page::Profile => pages::profile_page::view(model),
        Page::Reports => pages::reports_page::view(model),
        Page::IssuesAndFilters => pages::issues_and_filters::view(model),
        Page::Epics => pages::epics_page::view(model),
    }
}

#[inline(always)]
fn resolve_page(url: Url) -> Option<Page> {
    if url.path().is_empty() {
        return Some(Page::Project);
    }

    let page = match url.path()[0].as_ref() {
        "board" => Page::Project,
        "profile" => Page::Profile,
        "issues" => match url.path().get(1).as_ref().map(|s| s.parse::<i32>()) {
            Some(Ok(id)) => Page::EditIssue(id),
            _ => return None,
        },
        "add-issue" => Page::AddIssue,
        "project-settings" => Page::ProjectSettings,
        "login" => Page::SignIn,
        "register" => Page::SignUp,
        "invite" => Page::Invite,
        "users" => Page::Users,
        "reports" => Page::Reports,
        "delete-epic" => match url.path().get(1).as_ref().map(|s| s.parse::<i32>()) {
            Some(Ok(id)) => Page::DeleteEpic(id),
            _ => return None,
        },
        "edit-epic" => match url.path().get(1).as_ref().map(|s| s.parse::<i32>()) {
            Some(Ok(id)) => Page::EditEpic(id),
            _ => return None,
        },
        "issues-and-filters" => Page::IssuesAndFilters,
        "epics" => Page::Epics,
        _ => Page::Project,
    };
    Some(page)
}

#[wasm_bindgen]
pub fn render() {
    let app = seed::App::start("app", init, update, view);
    wasm_logger::init(if cfg!(debug_assertions) {
        console_error_panic_hook::set_once();
        wasm_logger::Config::default()
    } else {
        wasm_logger::Config::new(log::Level::Error)
            .message_on_new_line()
            .module_prefix("jirs")
    });

    #[cfg(debug_assertions)]
    crate::shared::on_event::keydown(move |ev| {
        let app = app.clone();
        let event = seed::to_keyboard_event(&ev);

        if seed::document()
            .active_element()
            .map(|el| el.tag_name() != "BODY")
            .unwrap_or(true)
            || !event.shift_key()
            || !event.ctrl_key()
        {
            return;
        }

        let key: String = event.key();
        match key.as_str() {
            ">" => app.update(Msg::Debug(DebugMsg::Console)),
            "?" => app.update(Msg::Debug(DebugMsg::Modal)),
            _ => {}
        };
    });

    crate::shared::on_event::keydown(move |_ev| {
        let element = match seed::document().active_element() {
            Some(el) => el,
            _ => return,
        };
        let class_list = element.class_name();
        if !class_list.contains("textAreaInput") {
            return;
        }
        if element.get_attribute("rows").as_deref() != Some("auto") {
            return;
        }
        crate::components::styled_textarea::handle_resize(&element);
    });
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let sender = orders.msg_sender();
    let page = resolve_page(url).unwrap_or(Page::Project);
    let mut model = Model::new(
        location::host_url().to_string(),
        location::ws_url().to_string(),
        page,
    );
    let key_triggers = model.key_triggers.clone();

    let sender_clone = sender.clone();
    crate::shared::on_event::keypress(move |ev| {
        let sender = sender_clone.clone();
        let key_triggers = key_triggers.clone();
        let event = seed::to_keyboard_event(&ev);

        if seed::document()
            .active_element()
            .map(|el| el.tag_name() != "BODY")
            .unwrap_or(true)
        {
            return;
        }
        ev.prevent_default();
        ev.stop_propagation();

        let key: String = event.key();
        let t = key_triggers.borrow();
        if let Some(b) = key.chars().next().and_then(|c| t.get(&c)) {
            let msg = b.build();
            sender.clone()(Some(msg));
        }
    });

    {
        let sender_clone = sender.clone();
        let id = FieldId::ProjectSettings(ProjectFieldId::Description);
        model
            .distinct_key_up
            .keyup_wih_reset(id.to_str(), 20, move |ev| {
                let sender = sender_clone.clone();
                let key_ev = seed::to_keyboard_event(&ev);
                let target = key_ev.target().unwrap();
                let el = seed::to_html_el(&target);
                let value = el.inner_html();
                sender.clone()(Some(Msg::StrInputChanged(id.clone(), value)));
            });
    }

    open_socket(&mut model, orders);

    model
}

#[inline(always)]
fn authorize_or_redirect(model: &mut Model, orders: &mut impl Orders<Msg>) {
    let pathname = seed::document().location().unwrap().pathname().unwrap();
    match crate::shared::read_auth_token() {
        Ok(token) => {
            send_ws_msg(
                WsMsgSession::AuthorizeLoad(token).into(),
                model.ws.as_ref(),
                orders,
            );
        }
        Err(..) => {
            match pathname.as_str() {
                "/login" | "/register" | "/invite" => {}
                _ => {
                    go_to_login(orders);
                }
            };
        }
    };
}
