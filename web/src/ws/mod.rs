pub use init_load_sets::*;
use jirs_data::msg::{
    WsMsgComment, WsMsgEpic, WsMsgIssue, WsMsgIssueStatus, WsMsgMessage, WsMsgProject,
    WsMsgSession, WsMsgUser,
};
use jirs_data::*;
use seed::prelude::*;

use crate::model::*;
use crate::shared::{go_to_board, write_auth_token};
use crate::{Msg, OperationKind, ResourceKind, WebSocketChanged};

mod init_load_sets;

pub mod issue;

pub fn flush_queue(model: &mut Model, orders: &mut impl Orders<Msg>) {
    use seed::browser::web_socket::State;
    match model.ws.as_ref() {
        Some(ws) if ws.state() != State::Open => return,
        None => return,
        _ => (),
    };
    let mut old = vec![];
    std::mem::swap(&mut model.ws_queue, &mut old);
    for msg in old {
        send_ws_msg(msg, model.ws.as_ref(), orders);
    }
}

pub fn enqueue_ws_msg(v: Vec<WsMsg>, ws: Option<&WebSocket>, orders: &mut impl Orders<Msg>) {
    for msg in v {
        send_ws_msg(msg, ws, orders);
    }
}

pub fn send_ws_msg(msg: WsMsg, ws: Option<&WebSocket>, orders: &mut impl Orders<Msg>) {
    use seed::browser::web_socket::State;
    let ws = match ws {
        Some(ws) if ws.state() == State::Open => ws,
        _ => {
            orders
                .skip()
                .send_msg(Msg::WebSocketChange(WebSocketChanged::Bounced(msg)));
            return;
        }
    };
    let binary = bincode::serialize(&msg).unwrap_or_default();
    if let Err(e) = ws.send_bytes(binary.as_slice()) {
        log::error!("Failed to send ws msg. {:?}", e);
    }
}

pub fn open_socket(model: &mut Model, orders: &mut impl Orders<Msg>) {
    use seed::browser::web_socket::State;
    use seed::prelude::*;
    use seed::*;
    log::warn!("{:?}", model.ws.as_ref().map(|ws| ws.state()));

    match model.ws.as_ref() {
        Some(ws) if ws.state() != State::Closed => {
            return;
        }
        _ => (),
    };
    if model.host_url.is_empty() {
        return;
    }
    let url = model.ws_url.as_str();

    model.ws = WebSocket::builder(url, orders)
        .on_message(|msg| {
            Some(Msg::WebSocketChange(WebSocketChanged::WebSocketMessage(
                msg,
            )))
        })
        .on_open(|| {
            log::info!("open_socket opened");
            Some(Msg::WebSocketChange(WebSocketChanged::WebSocketOpened))
        })
        .on_close(|_| Some(Msg::WebSocketChange(WebSocketChanged::WebSocketClosed)))
        .on_error(|| {
            error!("Failed to open WebSocket");
            None as Option<Msg>
        })
        // .protocols(&["jirs"])
        .build_and_open()
        .ok();
}

pub async fn read_incoming(msg: WebSocketMessage) -> Msg {
    let bytes = msg.bytes().await.unwrap_or_default();
    Msg::WebSocketChange(WebSocketChanged::WebSocketMessageLoaded(bytes))
}

pub fn update(msg: &mut WsMsg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        // auth
        WsMsg::Session(WsMsgSession::AuthorizeLoaded(Ok((user, setting)))) => {
            model.user = Some(user.clone());
            model.user_settings = Some(setting.clone());

            if is_non_logged_area() {
                go_to_board(orders);
            }

            orders
                .skip()
                .send_msg(Msg::UserChanged(model.user.as_ref().cloned()))
                .send_msg(Msg::ResourceChanged(
                    ResourceKind::User,
                    OperationKind::SingleLoaded,
                    model.user.as_ref().map(|u| u.id),
                ))
                .send_msg(Msg::ResourceChanged(
                    ResourceKind::UserSetting,
                    OperationKind::SingleLoaded,
                    model.user.as_ref().map(|u| u.id),
                ))
                .send_msg(Msg::ResourceChanged(
                    ResourceKind::Auth,
                    OperationKind::SingleLoaded,
                    model.user.as_ref().map(|u| u.id),
                ));
        }
        WsMsg::Session(WsMsgSession::AuthorizeExpired) => {
            log::warn!("Received token expired");
            if let Ok(msg) = write_auth_token(None) {
                orders.skip().send_msg(msg).send_msg(Msg::ResourceChanged(
                    ResourceKind::Auth,
                    OperationKind::SingleRemoved,
                    model.user.as_ref().map(|u| u.id),
                ));
            }
        }
        // project
        WsMsg::Project(WsMsgProject::ProjectsLoaded(v)) => {
            model.projects = std::mem::take(v);
            init_current_project(model, orders);
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Project,
                OperationKind::ListLoaded,
                None,
            ));
        }
        // user projects
        WsMsg::UserProjectsLoaded(v) => {
            model.current_user_project = v.iter().find(|up| up.is_current).cloned();
            model.user_projects = std::mem::take(v);
            init_current_project(model, orders);
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::UserProject,
                OperationKind::ListLoaded,
                None,
            ));
        }
        WsMsg::UserProjectCurrentChanged(user_project) => {
            let mut old = vec![];
            std::mem::swap(&mut old, &mut model.user_projects);
            for mut up in old {
                up.is_current = up.id == user_project.id;
                model.user_projects.push(up);
            }
            model.current_user_project = Some(user_project.clone());
            init_current_project(model, orders);
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::UserProject,
                OperationKind::SingleModified,
                model.current_user_project.as_ref().map(|up| up.id),
            ));
        }
        // user settings
        WsMsg::User(WsMsgUser::AvatarUrlChanged(id, url)) => {
            if let Some(user) = model.user.as_mut().filter(|u| u.id == *id) {
                user.avatar_url = Some(url.clone());
            }
            if let Some(user) = model.users_by_id.get_mut(&id) {
                user.avatar_url = Some(url.clone());
            }
            if let Some(user) = model.users.iter_mut().find(|u| u.id == *id) {
                user.avatar_url = Some(url.clone());
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::User,
                OperationKind::SingleModified,
                Some(*id),
            ));
        }
        WsMsg::User(WsMsgUser::UserSettingUpdated(setting)) => {
            model.user_settings = Some(setting.clone());
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::UserSetting,
                OperationKind::SingleModified,
                model.user_settings.as_ref().map(|up| up.id),
            ));
        }

        // issue statuses
        WsMsg::IssueStatus(WsMsgIssueStatus::IssueStatusesLoaded(v)) => {
            model.issue_statuses = std::mem::take(v);
            model
                .issue_statuses
                .sort_by(|a, b| a.position.cmp(&b.position));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::IssueStatus,
                OperationKind::ListLoaded,
                None,
            ));
        }
        WsMsg::IssueStatus(WsMsgIssueStatus::IssueStatusCreated(is)) => {
            let id = is.id;
            model.issue_statuses.push(is.clone());
            model
                .issue_statuses
                .sort_by(|a, b| a.position.cmp(&b.position));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::IssueStatus,
                OperationKind::SingleCreated,
                Some(id),
            ));
        }
        WsMsg::IssueStatus(WsMsgIssueStatus::IssueStatusUpdated(changed)) => {
            let id = changed.id;
            if let Some(idx) = model.issue_statuses.iter().position(|c| c.id == changed.id) {
                std::mem::swap(&mut model.issue_statuses[idx], changed);
            }
            model
                .issue_statuses
                .sort_by(|a, b| a.position.cmp(&b.position));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::IssueStatus,
                OperationKind::SingleModified,
                Some(id),
            ));
        }
        WsMsg::IssueStatus(WsMsgIssueStatus::IssueStatusDeleted(dropped_id, _count)) => {
            let mut old = vec![];
            std::mem::swap(&mut model.issue_statuses, &mut old);
            for is in old {
                if is.id != *dropped_id {
                    model.issue_statuses.push(is);
                }
            }
            model
                .issue_statuses
                .sort_by(|a, b| a.position.cmp(&b.position));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::IssueStatus,
                OperationKind::SingleRemoved,
                Some(*dropped_id),
            ));
        }
        // issues
        WsMsg::Project(WsMsgProject::ProjectIssuesLoaded(v)) => {
            v.sort_by(|a, b| (a.list_position as i64).cmp(&(b.list_position as i64)));
            {
                let _ = std::mem::replace(model.issues_mut(), v.clone());
            };
            model.issues_by_id.clear();
            for issue in v {
                model.issues_by_id.insert(issue.id, issue.clone());
            }

            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Issue,
                OperationKind::ListLoaded,
                None,
            ));
        }
        WsMsg::Issue(WsMsgIssue::IssueCreated(issue)) => {
            let id = issue.id;
            model.issues_by_id.insert(id, issue.clone());
            if let Some(idx) = model.issues().iter().position(|i| i.id == issue.id) {
                let _ = std::mem::replace(&mut model.issues_mut()[idx], issue.clone());
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Issue,
                OperationKind::SingleCreated,
                Some(id),
            ));
        }
        WsMsg::Issue(WsMsgIssue::IssueUpdated(issue)) => {
            let id = issue.id;
            model.issues_by_id.remove(&id);
            model.issues_by_id.insert(id, issue.clone());
            if let Some(idx) = model.issues().iter().position(|i| i.id == issue.id) {
                let _ = std::mem::replace(&mut model.issues_mut()[idx], issue.clone());
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Issue,
                OperationKind::SingleModified,
                Some(id),
            ));
        }
        WsMsg::Issue(WsMsgIssue::IssueDeleted(id, _count)) => {
            let old = std::mem::take(model.issues_mut());
            for is in old {
                if is.id == *id {
                    continue;
                }
                model.issues_mut().push(is);
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Issue,
                OperationKind::SingleRemoved,
                Some(*id),
            ));
        }
        // users
        WsMsg::Project(WsMsgProject::ProjectUsersLoaded(v)) => {
            model.users = v.clone();
            model.users_by_id.clear();
            for user in v {
                model.users_by_id.insert(user.id, user.clone());
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::User,
                OperationKind::ListLoaded,
                None,
            ));
        }
        // comments
        WsMsg::Comment(WsMsgComment::IssueCommentsLoaded(comments)) => {
            let issue_id = match &model.modals().edit_issue {
                Some(modal) => modal.id,
                _ => return,
            };
            if comments.iter().any(|c| c.issue_id != issue_id) {
                return;
            }
            comments.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));
            model.comments = comments.clone();
            for comment in std::mem::take(comments) {
                model.comments_by_id.insert(comment.id, comment);
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Comment,
                OperationKind::ListLoaded,
                None,
            ));
        }
        WsMsg::Comment(WsMsgComment::CommentUpdated(comment)) => {
            let comment_id = comment.id;
            if let Some(idx) = model.comments.iter().position(|c| c.id == comment.id) {
                let _ = std::mem::replace(&mut model.comments[idx], comment.clone());
                model.comments_by_id.insert(comment.id, comment.clone());
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Comment,
                OperationKind::SingleModified,
                Some(comment_id),
            ));
        }
        WsMsg::Comment(WsMsgComment::CommentDeleted(comment_id, _count)) => {
            if let Some(idx) = model.comments.iter().position(|c| c.id == *comment_id) {
                model.comments.remove(idx);
            }
            model.comments_by_id.remove(&comment_id);
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Comment,
                OperationKind::SingleRemoved,
                Some(*comment_id),
            ));
        }
        // messages
        WsMsg::Message(WsMsgMessage::MessageUpdated(received)) => {
            if let Some(idx) = model.messages.iter().position(|m| m.id == received.id) {
                std::mem::swap(&mut model.messages[idx], received);
            }
            model.messages.sort_by(|a, b| a.id.cmp(&b.id));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Message,
                OperationKind::SingleModified,
                Some(received.id),
            ));
        }
        WsMsg::Message(WsMsgMessage::MessagesLoaded(v)) => {
            model.messages = std::mem::take(v);
            model.messages.sort_by(|a, b| a.id.cmp(&b.id));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Message,
                OperationKind::ListLoaded,
                None,
            ));
        }
        WsMsg::Message(WsMsgMessage::MessageMarkedSeen(id, _count)) => {
            if let Some(idx) = model.messages.iter().position(|m| m.id == *id) {
                model.messages.remove(idx);
            }
            model.messages.sort_by(|a, b| a.id.cmp(&b.id));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Message,
                OperationKind::SingleRemoved,
                Some(*id),
            ));
        }

        // epics
        WsMsg::Epic(WsMsgEpic::EpicsLoaded(epics)) => {
            model.epics = epics.clone();
            for epic in epics {
                model.epics_by_id.insert(epic.id, epic.clone());
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Epic,
                OperationKind::ListLoaded,
                None,
            ));
        }
        WsMsg::Epic(WsMsgEpic::EpicCreated(epic)) => {
            let id = epic.id;
            model.epics.push(epic.clone());
            model.epics.sort_by(|a, b| a.id.cmp(&b.id));
            model.epics_by_id.insert(epic.id, epic.clone());
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Epic,
                OperationKind::SingleCreated,
                Some(id),
            ));
        }
        WsMsg::Epic(WsMsgEpic::EpicUpdated(epic)) => {
            let epic_id = epic.id;
            if let Some(idx) = model.epics.iter().position(|e| e.id == epic.id) {
                let _ = std::mem::replace(&mut model.epics[idx], epic.clone());
            }
            model.epics_by_id.insert(epic.id, epic.clone());
            model.epics.sort_by(|a, b| a.id.cmp(&b.id));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Epic,
                OperationKind::SingleModified,
                Some(epic_id),
            ));
        }
        WsMsg::Epic(WsMsgEpic::EpicDeleted(id, _count)) => {
            if let Some(idx) = model.epics.iter().position(|e| e.id == *id) {
                model.epics.remove(idx);
            }
            model.epics_by_id.remove(&id);
            model.epics.sort_by(|a, b| a.id.cmp(&b.id));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Epic,
                OperationKind::SingleRemoved,
                Some(*id),
            ));
        }
        WsMsg::Session(WsMsgSession::AuthenticateSuccess) => {
            let page = crate::match_page_mut!(model, SignIn);
            page.login_success = true;
        }
        WsMsg::Session(WsMsgSession::BindTokenOk(access_token)) => {
            match write_auth_token(Some(*access_token)) {
                Ok(msg) => {
                    orders.skip().send_msg(msg);
                }
                Err(e) => {
                    log::error!("{}", e);
                }
            }
        }
        _ => {
            log::info!(
                "got web socket message but don't know what to do with it {:?}",
                msg
            );
        }
    };
}

fn init_current_project(model: &mut Model, orders: &mut impl Orders<Msg>) {
    if model.projects.is_empty() {
        return;
    }
    model.project = model.current_user_project.as_ref().and_then(|up| {
        model
            .projects
            .iter()
            .find(|p| p.id == up.project_id)
            .cloned()
    });
    orders
        .skip()
        .send_msg(Msg::ProjectChanged(model.project.as_ref().cloned()));
}

fn is_non_logged_area() -> bool {
    let pathname = seed::document().location().unwrap().pathname().unwrap();
    matches!(pathname.as_str(), "/login" | "/register" | "/invite")
}
