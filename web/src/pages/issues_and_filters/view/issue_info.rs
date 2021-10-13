use seed::*;
use seed::prelude::*;

use jirs_data::{Issue, IssueId};

use crate::components::styled_icon::*;
use crate::components::styled_link::*;
use crate::model::Model;
use crate::Msg;

pub fn issue_details(model: &Model, issue: &Issue, project_name: &str) -> Node<Msg> {
    let Issue {
        id,
        title,
        issue_type,
        priority,
        description,
        issue_status_id,
        epic_id,
        created_at: _,
        updated_at: _,
        user_ids: _,
        ..
    } = issue;

    let issue_link = StyledLink {
        href: format!("/issues/{}", id).as_str(),
        children: vec![div![
            C!["text"],
            format!("{}-{}", project_name, id).as_str()
        ]],
        disabled: true,
        ..Default::default()
    }
    .render();

    let project_link = StyledLink {
        disabled: true,
        href: "/board",
        children: vec![span![model
            .project
            .as_ref()
            .map(|p| p.name.as_str())
            .unwrap_or_default()]],
        ..Default::default()
    }
    .render();

    let details = {
        let issue_type = {
            let type_name = issue_type.to_string();
            let type_icon = StyledIcon::from(Icon::from(*issue_type)).render();
            li![
                C!["line"],
                div![C!["detailsTitle"], "Type:"],
                div![C!["type detailsValue"], type_icon, type_name],
            ]
        };
        let priority = {
            let name = priority.to_string();
            let icon = StyledIcon::from(Icon::from(*priority)).render();
            li![
                C!["line"],
                div![C!["detailsTitle"], "Priority:"],
                div![C!["priority detailsValue"], icon, name],
            ]
        };

        let epic = li![
            C!["line"],
            div![C!["detailsTitle"], "Epic link:"],
            match epic_id.and_then(|id| model.epics_by_id.get(&id)) {
                Some(epic) => div![C!["detailsValue epic"], a![epic.name.as_str()]],
                _ => div![C!["detailsValue epic"], "None"],
            },
        ];

        let status = li![
            C!["line"],
            div![C!["detailsTitle"], "Status:"],
            div![C!["detailsValue status"], {
                match model.issue_statuses_by_id.get(issue_status_id) {
                    Some(status) => status.name.as_str(),
                    _ => "",
                }
            }]
        ];

        ul![C!["details"], issue_type, priority, epic, status]
    };

    let right_column = div![];

    let project_icon = crate::images::project_avatar::render();

    div![
        C!["issueInfo"],
        div![
            C!["header"],
            div![C!["logo"], project_icon],
            div![C!["title"], title.as_str()],
            div![C!["path"], project_link, span!["/"], issue_link]
        ],
        div![C!["issueBody"], details, right_column],
        div![
            C!["description"],
            raw! { description.as_deref().unwrap_or_default() }
        ]
    ]
}

pub fn issues_list(
    model: &Model,
    page: &super::super::IssuesAndFiltersPage,
    project_name: &str,
) -> Node<Msg> {
    let issues: Vec<Node<Msg>> = page
        .visible_issues
        .iter()
        .filter_map(|id| model.issues_by_id.get(id))
        .map(|issue| issue_entry(page, issue, project_name))
        .collect();
    ul![C!["issuesList"], issues]
}

fn issue_entry(
    page: &super::super::IssuesAndFiltersPage,
    issue: &Issue,
    project_name: &str,
) -> Node<Msg> {
    let link = issue_link(issue.id, project_name);
    let ty = { StyledIcon::from(Icon::from(issue.issue_type)).render() };
    let priority = { StyledIcon::from(Icon::from(issue.priority)).render() };

    let on_click = {
        let id = issue.id;
        mouse_ev("click", move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::SetActiveIssue(Some(id))
        })
    };

    li![
        C!["listItem"],
        a![
            on_click,
            C!["issue"],
            IF![page.active_id == Some(issue.id) => C!["active"]],
            div![C!["number"], link],
            div![C!["name"], issue.title.as_str()],
            div![C!["type"], ty],
            div![C!["priority"], priority]
        ]
    ]
}

fn issue_link(id: IssueId, project_name: &str) -> Node<Msg> {
    StyledLink {
        href: format!("/issues/{}", id).as_str(),
        children: vec![
            StyledIcon::from(Icon::Link).render(),
            span![format!("{}-{}", project_name, id)],
        ],
        disabled: true,
        class_list: "withIcon issueLink",
    }
    .render()
}
