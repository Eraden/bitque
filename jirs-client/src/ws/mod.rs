use seed::prelude::*;

use jirs_data::WsMsg;

use crate::model::*;
use crate::shared::write_auth_token;
use crate::{Msg, WebSocketChanged};

pub mod issue;

pub fn send_ws_msg(msg: WsMsg, ws: Option<&WebSocket>) {
    let ws = match ws {
        Some(ws) => ws,
        _ => return,
    };
    let binary = bincode::serialize(&msg).unwrap();
    ws.send_bytes(binary.as_slice())
        .expect("Failed to send ws msg");
}

pub fn open_socket(model: &mut Model, orders: &mut impl Orders<Msg>) {
    if model.host_url.is_empty() {
        return;
    }
    let url = model.ws_url.as_str();

    model.ws = WebSocket::builder(url, orders)
        .on_message(|msg| Msg::WebSocketChange(WebSocketChanged::WebSocketMessage(msg)))
        .on_open(|| Msg::WebSocketChange(WebSocketChanged::WebSocketOpened))
        .on_close(|_| Msg::WebSocketChange(WebSocketChanged::WebSocketClosed))
        .on_error(|| {})
        .build_and_open()
        .ok();
}

pub async fn read_incoming(msg: WebSocketMessage) -> Msg {
    let bytes = msg.bytes().await.unwrap_or_default();
    Msg::WebSocketChange(WebSocketChanged::WebSocketMessageLoaded(bytes))
}

pub fn update(msg: &WsMsg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        // auth
        WsMsg::AuthorizeLoaded(Ok(user)) => {
            model.user = Some(user.clone());
        }
        WsMsg::AuthorizeExpired => {
            if let Ok(msg) = write_auth_token(None) {
                orders.skip().send_msg(msg);
            }
        }
        // project
        WsMsg::ProjectLoaded(project) => {
            model.project = Some(project.clone());
        }
        // issues
        WsMsg::ProjectIssuesLoaded(v) => {
            let mut v = v.clone();
            v.sort_by(|a, b| (a.list_position as i64).cmp(&(b.list_position as i64)));
            model.issues = v;
        }
        // issue statuses
        WsMsg::IssueStatusesResponse(v) => {
            model.issue_statuses = v.clone();
            model
                .issue_statuses
                .sort_by(|a, b| a.position.cmp(&b.position));
        }
        WsMsg::IssueStatusCreated(is) => {
            model.issue_statuses.push(is.clone());
            model
                .issue_statuses
                .sort_by(|a, b| a.position.cmp(&b.position));
        }
        WsMsg::IssueStatusUpdated(changed) => {
            let mut old = vec![];
            std::mem::swap(&mut model.issue_statuses, &mut old);
            for is in old {
                if is.id == changed.id {
                    model.issue_statuses.push(changed.clone());
                } else {
                    model.issue_statuses.push(is);
                }
            }
            model
                .issue_statuses
                .sort_by(|a, b| a.position.cmp(&b.position));
        }
        WsMsg::IssueDeleted(id) => {
            let mut old = vec![];
            std::mem::swap(&mut model.issue_statuses, &mut old);
            for is in old {
                if is.id == *id {
                    continue;
                }
                model.issue_statuses.push(is);
            }
            model
                .issue_statuses
                .sort_by(|a, b| a.position.cmp(&b.position));
        }
        // users
        WsMsg::ProjectUsersLoaded(v) => {
            model.users = v.clone();
        }
        // comments
        WsMsg::IssueCommentsLoaded(comments) => {
            let issue_id = match model.modals.get(0) {
                Some(ModalType::EditIssue(issue_id, _)) => *issue_id,
                _ => return,
            };
            if comments.iter().any(|c| c.issue_id != issue_id) {
                return;
            }
            let mut v = comments.clone();
            v.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));
            model.comments = v;
        }
        WsMsg::CommentDeleted(comment_id) => {
            let mut old = vec![];
            std::mem::swap(&mut model.comments, &mut old);
            for comment in old.into_iter() {
                if *comment_id != comment.id {
                    model.comments.push(comment);
                }
            }
        }
        WsMsg::AvatarUrlChanged(user_id, avatar_url) => {
            for user in model.users.iter_mut() {
                if user.id == *user_id {
                    user.avatar_url = Some(avatar_url.clone());
                }
            }
            if let Some(me) = model.user.as_mut() {
                if me.id == *user_id {
                    me.avatar_url = Some(avatar_url.clone());
                }
            }
        }
        _ => (),
    };
    orders.render();
}
