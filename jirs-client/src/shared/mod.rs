use seed::{prelude::*, *};

use jirs_data::*;

use crate::{
    model::{Model, Page},
    resolve_page, Msg,
};

pub mod aside;
pub mod drag;
pub mod navbar_left;
pub mod styled_avatar;
pub mod styled_button;
pub mod styled_checkbox;
pub mod styled_confirm_modal;
pub mod styled_date_time_input;
pub mod styled_editor;
pub mod styled_field;
pub mod styled_form;
pub mod styled_icon;
pub mod styled_image_input;
pub mod styled_input;
pub mod styled_link;
pub mod styled_modal;
// pub mod styled_rte;
pub mod styled_select;
pub mod styled_select_child;
pub mod styled_textarea;
pub mod styled_tooltip;
pub mod tracking_widget;

pub trait ToChild<'l> {
    type Builder: 'l;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder;
}

#[inline]
pub fn go_to_board(orders: &mut impl Orders<Msg>) {
    go_to("board", orders);
    orders.skip().send_msg(Msg::ChangePage(Page::Project));
}

#[inline]
pub fn go_to_login(orders: &mut impl Orders<Msg>) {
    go_to("login", orders);
    orders.skip().send_msg(Msg::ChangePage(Page::SignIn));
}

#[inline]
pub fn go_to(url: &str, orders: &mut impl Orders<Msg>) {
    let url = seed::Url::new().add_path_part(url);
    url.go_and_push();
    if let Some(page) = resolve_page(url) {
        orders.skip().send_msg(Msg::ChangePage(page));
    }
}

pub fn find_issue(model: &'_ Model, issue_id: IssueId) -> Option<&'_ Issue> {
    model.issues.iter().find(|issue| issue.id == issue_id)
}

pub trait ToNode {
    fn into_node(self) -> Node<Msg>;
}

pub fn divider() -> Node<Msg> {
    div![C!["divider"], ""]
}

pub fn inner_layout(model: &Model, page_name: &str, children: Vec<Node<Msg>>) -> Node<Msg> {
    let modal_node = crate::modal::view(model);
    article![
        modal_node,
        C!["inner-layout", "innerPage"],
        id![page_name],
        navbar_left::render(model),
        aside::render(model),
        children,
    ]
}

pub fn outer_layout(model: &Model, page_name: &str, children: Vec<Node<Msg>>) -> Node<Msg> {
    let modal = crate::modal::view(model);
    article![
        C!["outer-layout", "outerPage"],
        id![page_name],
        modal,
        children
    ]
}

pub fn write_auth_token(token: Option<uuid::Uuid>) -> Result<Msg, String> {
    let w = window();
    let store = match w.local_storage() {
        Ok(Some(store)) => store,
        _ => return Err("Local storage is not available".to_string()),
    };
    match token {
        Some(token) => {
            store
                .set_item("authToken", format!("{}", token).as_str())
                .map_err(|e| format!("Failed to read auth token. {:?}", e))?;
        }
        _ => {
            store
                .remove_item("authToken")
                .map_err(|e| format!("Failed to read auth token. {:?}", e))?;
        }
    }

    Ok(match token {
        Some(_) => Msg::AuthTokenStored,
        None => Msg::AuthTokenErased,
    })
}

pub fn read_auth_token() -> Result<uuid::Uuid, String> {
    let w = window();
    let store = match w.local_storage() {
        Ok(Some(store)) => store,
        _ => return Err("Local storage is not available".to_string()),
    };
    store
        .get_item("authToken")
        .map_err(|_e| "Failed to read auth token".to_string())?
        .ok_or_else(|| "Auth token not found".to_string())?
        .parse()
        .map_err(|_| "Bad token format".to_string())
}
