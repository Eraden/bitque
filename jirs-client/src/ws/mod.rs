use seed::prelude::*;

use jirs_data::WsMsg;

use crate::model::*;
use crate::shared::write_auth_token;
use crate::{Msg, APP};

pub mod issue;

pub fn handle(msg: WsMsg) {
    let app = match unsafe { APP.as_mut().unwrap() }.write() {
        Ok(app) => app,
        _ => return,
    };

    match msg {
        WsMsg::Ping | WsMsg::Pong => {}
        _ => app.update(Msg::WsMsg(msg)),
    }
}

pub fn update(msg: &Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        // auth
        Msg::WsMsg(WsMsg::AuthorizeLoaded(Ok(user))) => {
            model.user = Some(user.clone());
        }
        Msg::WsMsg(WsMsg::AuthorizeExpired) => {
            if let Ok(msg) = write_auth_token(None) {
                orders.skip().send_msg(msg);
            }
        }
        // project
        Msg::WsMsg(WsMsg::ProjectLoaded(project)) => {
            model.project = Some(project.clone());
        }
        // issues
        Msg::WsMsg(WsMsg::ProjectIssuesLoaded(v)) => {
            let mut v = v.clone();
            v.sort_by(|a, b| (a.list_position as i64).cmp(&(b.list_position as i64)));
            model.issues = v;
        }
        // users
        Msg::WsMsg(WsMsg::ProjectUsersLoaded(v)) => {
            model.users = v.clone();
        }
        // comments
        Msg::WsMsg(WsMsg::IssueCommentsLoaded(comments)) => {
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
        Msg::WsMsg(WsMsg::CommentDeleted(comment_id)) => {
            let mut old = vec![];
            std::mem::swap(&mut model.comments, &mut old);
            for comment in old.into_iter() {
                if *comment_id != comment.id {
                    model.comments.push(comment);
                }
            }
        }
        _ => (),
    };
    orders.render();
}
