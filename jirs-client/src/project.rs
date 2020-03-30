use seed::{prelude::*, *};

use crate::model::Page;
use crate::shared::{host_client, inner_layout};
use crate::Msg;

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangePage(Page::Project) => {
            orders
                .skip()
                .perform_cmd(crate::api::fetch_current_project(model.host_url.clone()));
            orders
                .skip()
                .perform_cmd(crate::api::fetch_current_user(model.host_url.clone()));
        }
        Msg::ToggleAboutTooltip => {
            model.about_tooltip_visible = !model.about_tooltip_visible;
        }
        _ => (),
    }
}

pub fn view(model: &crate::model::Model) -> Node<Msg> {
    let project_section = section![id!["project-section"],];

    inner_layout(model, project_section)
}
