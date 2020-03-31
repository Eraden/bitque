use seed::fetch::{FetchObject, ResponseWithDataResult};
use seed::{prelude::*, *};

use jirs_data::FullProjectResponse;

use crate::model::{Icon, Model};
use crate::Msg;

pub mod aside;
pub mod navbar_left;
pub mod styled_button;
pub mod styled_input;
pub mod styled_tooltip;

pub fn styled_icon(icon: Icon) -> Node<Msg> {
    i![attrs![At::Class => format!("styledIcon {}", icon)], ""]
}

pub fn divider() -> Node<Msg> {
    div![attrs![At::Class => "divider"], ""]
}

pub fn inner_layout(model: &Model, page_name: &str, children: Vec<Node<Msg>>) -> Node<Msg> {
    article![
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
        Msg::CurrentProjectResult(FetchObject {
            result:
                Ok(ResponseWithDataResult {
                    data: Ok(body),
                    status,
                    ..
                }),
            ..
        }) if status.is_ok() => match serde_json::from_str::<'_, FullProjectResponse>(body) {
            Ok(project_response) => {
                model.project = Some(project_response.project);
            }
            _ => (),
        },
        Msg::CurrentUserResult(FetchObject {
            result:
                Ok(ResponseWithDataResult {
                    data: Ok(body),
                    status,
                    ..
                }),
            ..
        }) if status.is_ok() => match serde_json::from_str::<'_, jirs_data::User>(body) {
            Ok(user) => {
                model.user = Some(user);
            }
            _ => (),
        },
        _ => (),
    }
}
