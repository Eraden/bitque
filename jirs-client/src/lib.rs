use seed::{prelude::*, *};
use web_sys::File;

use jirs_data::*;

use crate::model::{ModalType, Model, Page};
use crate::shared::styled_select::StyledSelectChange;
use crate::shared::{go_to_board, go_to_login};
use crate::ws::{open_socket, read_incoming, send_ws_msg};

mod changes;
mod fields;
mod invite;
mod modal;
mod model;
mod profile;
mod project;
mod project_settings;
mod shared;
mod sign_in;
mod sign_up;
mod users;
pub mod validations;
mod ws;

pub use changes::*;
pub use fields::*;

pub type AppType = App<Msg, Model, Node<Msg>>;

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

    StyledSelectChanged(FieldId, StyledSelectChange),
    InternalFailure(String),
    ToggleAboutTooltip,

    // Auth Token
    AuthTokenStored,
    AuthTokenErased,
    SignInRequest,
    BindClientRequest,

    // users
    InviteRequest,
    InviteRevokeRequest(InvitationId),
    InviteApproveRequest(InvitationId),
    InvitedUserRemove(EmailString),

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

    // issues
    AddIssue,
    DeleteIssue(IssueId),

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
                    authorize_or_redirect(model);
                    send_ws_msg(WsMsg::Ping, model.ws.as_ref());
                    return;
                }
                WebSocketChanged::SendPing => {
                    send_ws_msg(WsMsg::Ping, model.ws.as_ref());
                    return;
                }
                WebSocketChanged::WebSocketMessage(incoming) => {
                    orders.perform_cmd(read_incoming(incoming));
                    return;
                }
                WebSocketChanged::WsMsg(ref ws_msg) => {
                    ws::update(ws_msg, model, orders);
                }
                WebSocketChanged::WebSocketMessageLoaded(v) => {
                    if let Ok(m) = bincode::deserialize(v.clone().as_slice()) {
                        match m {
                            WsMsg::Ping | WsMsg::Pong => {
                                orders.perform_cmd(cmds::timeout(1000, || {
                                    Msg::WebSocketChange(WebSocketChanged::SendPing)
                                }));
                            }
                            _ => {
                                orders
                                    .skip()
                                    .send_msg(Msg::WebSocketChange(WebSocketChanged::WsMsg(m)));
                            }
                        }
                    };
                    return;
                }
                WebSocketChanged::WebSocketClosed => {
                    open_socket(model, orders);
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
            go_to_board();
            orders.skip().send_msg(Msg::ChangePage(Page::Project));
            authorize_or_redirect(model);
            return;
        }
        Msg::AuthTokenErased => {
            go_to_login();
            orders.skip().send_msg(Msg::ChangePage(Page::SignIn));
            authorize_or_redirect(model);
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
    crate::modal::update(&msg, model, orders);
    match model.page {
        Page::Project | Page::AddIssue | Page::EditIssue(..) => project::update(msg, model, orders),
        Page::ProjectSettings => project_settings::update(msg, model, orders),
        Page::SignIn => sign_in::update(msg, model, orders),
        Page::SignUp => sign_up::update(msg, model, orders),
        Page::Invite => invite::update(msg, model, orders),
        Page::Users => users::update(msg, model, orders),
        Page::Profile => profile::update(msg, model, orders),
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

    // if let Some(body) = seed::html_document().body() {
    // use wasm_bindgen::JsCast;
    // let body = body.dyn_ref::<web_sys::HtmlBodyElement>().unwrap().clone();
    // let key_up_closure =
    //     wasm_bindgen::closure::Closure::wrap(Box::new(|event: web_sys::KeyboardEvent| {
    //         if let Some(Ok(app)) = unsafe { APP.as_mut().map(|app| app.write()) } {
    //             let msg = Msg::GlobalKeyDown {
    //                 key: event.key(),
    //                 shift: event.shift_key(),
    //                 ctrl: event.ctrl_key(),
    //                 alt: event.alt_key(),
    //             };
    //             app.update(msg);
    //         }
    //     })
    //         as Box<dyn Fn(web_sys::KeyboardEvent)>);
    // body.add_event_listener_with_callback("keyup", key_up_closure.as_ref().unchecked_ref())
    //     .unwrap();
    // key_up_closure.forget();
    // }

    let _app = seed::App::builder(update, view)
        .routes(routes)
        .after_mount(after_mount)
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
    model.page = resolve_page(url).unwrap_or_else(|| Page::Project);
    log!(model);
    open_socket(&mut model, orders);
    AfterMount::new(model).url_handling(UrlHandling::PassToRoutes)
}

#[inline]
fn authorize_or_redirect(model: &Model) {
    match crate::shared::read_auth_token() {
        Ok(token) => {
            send_ws_msg(WsMsg::AuthorizeRequest(token), model.ws.as_ref());
        }
        Err(..) => {
            let pathname = seed::document().location().unwrap().pathname().unwrap();
            match pathname.as_str() {
                "/login" | "/register" | "/invite" => {}
                _ => {
                    go_to_login();
                }
            };
        }
    };
}
