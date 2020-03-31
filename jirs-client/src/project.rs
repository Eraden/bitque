use seed::{prelude::*, *};

use crate::model::{Icon, Model, Page};
use crate::shared::styled_button::{StyledButton, Variant};
use crate::shared::styled_input::StyledInput;
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
            model.project_page.about_tooltip_visible = !model.project_page.about_tooltip_visible;
        }
        Msg::ProjectTextFilterChanged(text) => {
            model.project_page.text_filter = text;
        }
        _ => (),
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let project_section = vec![breadcrumbs(model), header(), project_board_filters(model)];

    inner_layout(model, "projectPage", project_section)
}

fn breadcrumbs(model: &Model) -> Node<Msg> {
    let project_name = model
        .project
        .as_ref()
        .map(|p| p.name.clone())
        .unwrap_or_default();
    div![
        attrs![At::Class => "breadcrumbsContainer"],
        span!["Projects"],
        span![attrs![At::Class => "breadcrumbsDivider"], "/"],
        span![project_name],
        span![attrs![At::Class => "breadcrumbsDivider"], "/"],
        span!["Kanban Board"]
    ]
}

fn header() -> Node<Msg> {
    let button = StyledButton {
        variant: Variant::Secondary,
        icon_only: false,
        disabled: false,
        active: false,
        text: Some("Github Repo".to_string()),
        icon: Some(Icon::Github),
    }
    .into_node();
    div![
        id!["projectBoardHeader"],
        div![id!["boardName"], "Kanban board"],
        a![
            attrs![At::Href => "https://gitlab.com/adrian.wozniak/jirs", At::Target => "__blank", At::Rel => "noreferrer noopener"],
            button
        ]
    ]
}

fn project_board_filters(_model: &Model) -> Node<Msg> {
    let search_input = StyledInput {
        icon: Some(Icon::Search),
        id: Some("searchInput".to_string()),
        valid: true,
        on_change: input_ev(Ev::Change, |value| Msg::ProjectTextFilterChanged(value)),
    }
    .into_node();
    div![id!["projectBoardFilters"], search_input]
}
