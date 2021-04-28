use seed::prelude::*;
use seed::*;

use crate::model::Model;
use crate::shared::inner_layout;
use crate::Msg;

mod filters;
mod issue_info;

pub fn view(model: &Model) -> Node<Msg> {
    let page = crate::match_page!(model, IssuesAndFilters; Empty);
    let project_name = model
        .project
        .as_ref()
        .map(|p| p.name.as_str())
        .unwrap_or_default();

    let details = match page.active_id.and_then(|id| model.issues_by_id.get(&id)) {
        Some(issue) => issue_info::issue_details(model, issue, project_name),
        _ => Node::Empty,
    };
    let content = div![
        C!["container"],
        issue_info::issues_list(model, page, project_name),
        details
    ];
    inner_layout(
        model,
        "issuesAndFilters",
        &[filters::filters(model, page), content],
    )
}
