use chrono::NaiveDateTime;
use jirs_data::{Issue, IssueStatus};
use seed::prelude::*;
use seed::*;

use crate::components::styled_icon::{Icon, StyledIcon};
use crate::model::Model;
use crate::shared::inner_layout;
use crate::Msg;

pub fn view(model: &Model) -> Node<Msg> {
    let page = crate::match_page!(model, Epics; Empty);

    let epics = model
        .epic_ids
        .iter()
        .filter_map(|id| model.epics_by_id.get(id))
        .map(|epic| {
            let issues = page.issues(epic.id).map(|v| {
                v.iter()
                    .filter_map(|i| model.issues_by_id.get(i))
                    .map(|issue| {
                        render_issue(
                            issue,
                            model.issue_statuses_by_id.get(&issue.issue_status_id),
                        )
                    })
                    .collect::<Vec<Node<Msg>>>()
            });

            li![
                C!["epic"],
                div![
                    C!["firstRow"],
                    div![C!["epicName"], &epic.name],
                    div![
                        C!["date"],
                        date_field("Starts at:", "startsAt", epic.starts_at.as_ref()),
                        date_field("Ends at:", "endsAt", epic.ends_at.as_ref()),
                    ],
                    div![
                        C!["counter"],
                        "Number of issues:",
                        issues.as_ref().map(Vec::len).unwrap_or(0)
                    ],
                ],
                div![C!["secondRow"], div![C!["issues"], issues]]
            ]
        });

    inner_layout(
        model,
        "epics",
        &[section![
            h1!["Epics"],
            p![C!["description"], "Epics and issues grouped in them"],
            ul![C!["epicsList"], epics]
        ]],
    )
}

fn date_field(
    name: &'static str,
    class_name: &'static str,
    date: Option<&NaiveDateTime>,
) -> Node<Msg> {
    match date {
        None => Node::Empty,
        Some(d) => div![
            C![class_name],
            span![name],
            span![format!("{}", d.format("%e %B %Y"))]
        ],
    }
}

fn render_issue(issue: &Issue, status: Option<&IssueStatus>) -> Node<Msg> {
    div![
        C!["issue"],
        div![C!["name"], issue.title.as_str()],
        div![C!["status"], status.map(|s| s.name.as_str())],
        div![
            C!["flags"],
            div![
                C!["type"],
                StyledIcon::from(Icon::from(issue.issue_type)).render()
            ],
            div![
                C!["priority"],
                StyledIcon::from(Icon::from(issue.priority)).render()
            ],
        ]
    ]
}
