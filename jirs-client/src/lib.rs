#![feature(or_patterns, type_ascription)]

use seed::{prelude::*, *};
use web_sys::File;

pub use changes::*;
pub use fields::*;
use jirs_data::*;

use crate::model::{ModalType, Model, Page};
use crate::shared::styled_date_time_input::StyledDateTimeChanged;
use crate::shared::{go_to_board, go_to_login, styled_tooltip};
// use crate::shared::styled_rte::RteMsg;
use crate::shared::styled_select::StyledSelectChanged;
use crate::shared::styled_tooltip::{Variant as StyledTooltip, Variant};
use crate::ws::{flush_queue, open_socket, read_incoming, send_ws_msg};

mod changes;
pub mod elements;
mod fields;
mod invite;
mod modal;
mod model;
mod profile;
mod project;
mod project_settings;
mod reports;
mod shared;
mod sign_in;
mod sign_up;
mod users;
pub mod validations;
mod ws;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
pub enum Msg {
    GlobalKeyDown {
        key: String,
        shift: bool,
        ctrl: bool,
        alt: bool,
    },
    PageChanged(PageChanged),
    ChangePage(model::Page),

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

    // inputs
    StrInputChanged(FieldId, String),
    U32InputChanged(FieldId, u32),
    FileInputChanged(FieldId, Vec<File>),
    // Rte(FieldId, RteMsg),

    // issues
    AddIssue,
    DeleteIssue(IssueId),

    // epics
    AddEpic,
    DeleteEpic,
    UpdateEpic,

    // issue statuses
    DeleteIssueStatus(IssueStatusId),

    // comments
    SaveComment,
    DeleteComment(CommentId),

    // profile
    AvatarUpdateFetched(String),

    // modals
    ModalOpened(Box<ModalType>),
    ModalDropped,
    ModalChanged(FieldChange),

    // popups
    MessageSeen(MessageId),
    MessageInvitationApproved(InvitationToken),
    MessageInvitationDismiss(InvitationToken),

    // WebSocket
    WebSocketChange(WebSocketChanged),
}

fn update(msg: Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    if model.ws.is_none() {
        open_socket(model, orders);
    }

    let msg = match msg {
        Msg::WebSocketChange(change) => {
            match change {
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
                WebSocketChanged::WsMsg(ref ws_msg) => {
                    ws::update(ws_msg, model, orders);
                }
                WebSocketChanged::WebSocketMessageLoaded(v) => {
                    match bincode::deserialize(v.as_slice()) {
                        Ok(WsMsg::Ping | WsMsg::Pong) => {
                            orders.skip();
                            orders.perform_cmd(cmds::timeout(300, || {
                                Msg::WebSocketChange(WebSocketChanged::SendPing)
                            }));
                        }
                        Ok(m) => {
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
                }
                WebSocketChanged::Bounced(ws_msg) => {
                    model.ws_queue.push(ws_msg);
                    open_socket(model, orders);
                    return;
                }
            };
            Msg::WebSocketChange(change)
        }
        _ => msg,
    };

    if cfg!(debug_assertions) {
        log!(msg);
    }

    match &msg {
        Msg::AuthTokenStored => {
            go_to_board(orders);
            return;
        }
        Msg::AuthTokenErased => {
            go_to_login(orders);
            return;
        }
        Msg::ChangePage(page) => {
            orders.skip();
            model.page = *page;
        }
        Msg::ToggleTooltip(variant) => match variant {
            styled_tooltip::Variant::About => {
                model.about_tooltip_visible = !model.about_tooltip_visible;
            }
            styled_tooltip::Variant::Messages => {
                model.messages_tooltip_visible = !model.messages_tooltip_visible;
            }
            styled_tooltip::Variant::CodeBuilder => {}
            Variant::TableBuilder => {}
            Variant::DateTimeBuilder => {}
        },
        _ => (),
    }
    crate::shared::aside::update(&msg, model, orders);
    crate::shared::navbar_left::update(&msg, model, orders);
    crate::modal::update(&msg, model, orders);
    match model.page {
        Page::Project | Page::AddIssue | Page::EditIssue(..) => project::update(msg, model, orders),
        Page::ProjectSettings => project_settings::update(msg, model, orders),
        Page::SignIn => sign_in::update(msg, model, orders),
        Page::SignUp => sign_up::update(msg, model, orders),
        Page::Invite => invite::update(msg, model, orders),
        Page::Users => users::update(msg, model, orders),
        Page::Profile => profile::update(msg, model, orders),
        Page::Reports => reports::update(msg, model, orders),
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
        Page::Invite => invite::view(model),
        Page::Users => users::view(model),
        Page::Profile => profile::view(model),
        Page::Reports => reports::view(model),
    }
}

fn routes(url: Url) -> Option<Msg> {
    match resolve_page(url) {
        Some(page) => Some(Msg::ChangePage(page)),
        _ => None,
    }
}

fn resolve_page(url: Url) -> Option<Page> {
    if url.path().is_empty() {
        return Some(Page::Project);
    }

    let page = match url.path()[0].as_ref() {
        "board" => Page::Project,
        "issues" => match url.path().get(1).as_ref().map(|s| s.parse::<i32>()) {
            Some(Ok(id)) => Page::EditIssue(id),
            _ => return None,
        },
        "profile" => Page::Profile,
        "add-issue" => Page::AddIssue,
        "project-settings" => Page::ProjectSettings,
        "login" => Page::SignIn,
        "register" => Page::SignUp,
        "invite" => Page::Invite,
        "users" => Page::Users,
        "reports" => Page::Reports,
        _ => Page::Project,
    };
    Some(page)
}

pub static mut HOST_URL: String = String::new();
pub static mut WS_URL: String = String::new();

#[wasm_bindgen]
pub fn render(host_url: String, ws_url: String) {
    unsafe {
        HOST_URL = host_url;
        WS_URL = ws_url;
    }
    elements::define();

    let _app = seed::App::builder(update, view)
        .routes(routes)
        .after_mount(after_mount)
        .window_events(window_events)
        .build_and_start();
}

fn after_mount(url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    let host_url = unsafe { HOST_URL.clone() };
    let ws_url = unsafe { WS_URL.clone() };
    let mut model = Model::new(host_url, ws_url);
    unsafe {
        HOST_URL = "".to_string();
        WS_URL = "".to_string();
    }
    model.page = resolve_page(url).unwrap_or(Page::Project);
    open_socket(&mut model, orders);
    AfterMount::new(model).url_handling(UrlHandling::PassToRoutes)
}

fn window_events(_model: &Model) -> Vec<EventHandler<Msg>> {
    vec![keyboard_ev(
        Ev::KeyDown,
        move |event: web_sys::KeyboardEvent| {
            let tag_name: String = seed::document()
                .active_element()
                .map(|el| el.tag_name())
                .unwrap_or_default();

            let key = match tag_name.to_lowercase().as_str() {
                "" | "input" | "textarea" => return None,
                _ => event.key(),
            };

            Some(Msg::GlobalKeyDown {
                key,
                shift: event.shift_key(),
                ctrl: event.ctrl_key(),
                alt: event.alt_key(),
            })
        },
    )]
}

#[inline]
fn authorize_or_redirect(model: &mut Model, orders: &mut impl Orders<Msg>) {
    let pathname = seed::document().location().unwrap().pathname().unwrap();
    match crate::shared::read_auth_token() {
        Ok(token) => {
            send_ws_msg(WsMsg::AuthorizeLoad(token), model.ws.as_ref(), orders);
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
