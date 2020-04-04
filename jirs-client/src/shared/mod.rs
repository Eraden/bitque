use seed::{prelude::*, *};
use wasm_bindgen::JsCast;

use jirs_data::Issue;

use crate::model::Model;
use crate::{IssueId, Msg};

pub mod aside;
pub mod navbar_left;
pub mod styled_avatar;
pub mod styled_button;
pub mod styled_confirm_modal;
pub mod styled_field;
pub mod styled_form;
pub mod styled_icon;
pub mod styled_input;
pub mod styled_modal;
pub mod styled_select;
pub mod styled_tooltip;

pub fn find_issue(model: &Model, issue_id: IssueId) -> Option<&Issue> {
    match model.project.as_ref() {
        Some(p) => p.issues.iter().find(|issue| issue.id == issue_id),
        _ => None,
    }
}

pub trait ToNode {
    fn into_node(self) -> Node<Msg>;
}

pub fn divider() -> Node<Msg> {
    div![attrs![At::Class => "divider"], ""]
}

pub fn inner_layout(
    model: &Model,
    page_name: &str,
    children: Vec<Node<Msg>>,
    modal_node: Node<Msg>,
) -> Node<Msg> {
    article![
        modal_node,
        attrs![At::Class => "inner-layout"],
        id![page_name],
        navbar_left::render(model),
        aside::render(model),
        children,
    ]
}

pub fn host_client(host_url: String, path: &str) -> Result<Request, String> {
    let url = format!("{}{}", host_url, path);
    let w = window();
    let store = match w.local_storage() {
        Ok(Some(store)) => store,
        _ => return Err("Local storage is not available".to_string()),
    };
    let token = match store.get_item("authToken") {
        Ok(Some(s)) => s,
        _ => "".to_string(),
    };
    Ok(Request::new(url).header("Authorization", format!("Bearer {}", token).as_str()))
}

pub fn update(msg: &Msg, model: &mut crate::model::Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::CurrentProjectResult(fetched) => {
            crate::api_handlers::current_project_response(fetched, model);
        }
        Msg::CurrentUserResult(fetched) => {
            crate::api_handlers::current_user_response(fetched, model);
        }
        _ => (),
    }
}

pub fn drag_ev<Ms>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::DragEvent) -> Ms + 'static + Clone,
) -> EventHandler<Ms> {
    let closure_handler = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::DragEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, closure_handler)
}
