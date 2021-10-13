use seed::prelude::*;
use seed::*;

use crate::components::styled_button::StyledButton;
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::model::Model;
use crate::shared::inner_layout;
use crate::Msg;

mod board;
mod filters;

pub fn view(model: &Model) -> Node<Msg> {
    let project_section = [
        breadcrumbs(model),
        header(model),
        filters::project_board_filters(model),
        board::project_board_lists(model),
    ];

    inner_layout(model, "projectPage", &project_section)
}

fn breadcrumbs(model: &Model) -> Node<Msg> {
    div![
        C!["breadcrumbsContainer"],
        span!["Projects"],
        span![C!["breadcrumbsDivider"], "/"],
        span![model.project_name()],
        span![C!["breadcrumbsDivider"], "/"],
        span!["Kanban Board"]
    ]
}

fn header(model: &Model) -> Node<Msg> {
    if !model.show_extras {
        return Node::Empty;
    }
    div![
        id!["projectBoardHeader"],
        div![id!["boardName"], C!["headerChild"], "Kanban board"],
        a![
            attrs![At::Href => "https://gitlab.com/adrian.wozniak/jirs", At::Target => "__blank", At::Rel => "noreferrer noopener"],
            StyledButton::secondary_with_text_and_icon(
                "Repository",
                StyledIcon::from(Icon::Github).render(),
            )
            .render()
        ]
    ]
}
