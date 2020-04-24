use seed::{prelude::*, *};

use jirs_data::UpdateIssuePayload;

use crate::model::{EditIssueModal, ModalType, Model};
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::ToNode;
use crate::Msg;

pub fn tracking_link(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    let EditIssueModal { id, .. } = modal;

    let issue_id = *id;

    let handler = mouse_ev(Ev::Click, move |_| {
        Msg::ModalOpened(Box::new(ModalType::TimeTracking(issue_id)))
    });

    div![
        class!["trackingLink"],
        handler,
        tracking_widget(model, modal),
    ]
}

pub fn tracking_widget(_model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    let EditIssueModal {
        payload:
            UpdateIssuePayload {
                estimate,
                time_spent,
                time_remaining,
                ..
            },
        ..
    } = modal;

    let icon = StyledIcon::build(Icon::Stopwatch)
        .add_class("watchIcon")
        .size(32)
        .build()
        .into_node();
    let bar_width = calc_bar_width(*estimate, *time_spent, *time_remaining);

    let spent_text = if let Some(time) = time_spent {
        format!("{}h logged", time)
    } else {
        "No time logged".to_string()
    };

    let remaining_node: Node<Msg> = match (time_remaining, estimate) {
        (Some(n), _) => div![format!("{}h remaining", n)],
        (_, Some(n)) => div![format!("{}h estimated", n)],
        _ => empty![],
    };

    div![
        class!["trackingWidget"],
        icon,
        div![
            class!["right"],
            div![
                class!["barCounter"],
                div![
                    class!["bar"],
                    attrs![At::Style => format!("width: {}%", bar_width)]
                ]
            ],
            div![class!["values"], div![spent_text], remaining_node,]
        ]
    ]
}

#[inline]
fn calc_bar_width(
    estimate: Option<f64>,
    time_spent: Option<f64>,
    time_remaining: Option<f64>,
) -> f64 {
    match (estimate, time_spent, time_remaining) {
        (_, Some(spent), Some(remaining)) => ((spent / (spent + remaining)) * 100f64).min(100f64),
        (Some(estimate), Some(spent), _) => ((spent / estimate) * 100f64).min(100f64),
        (None, None, _) => 100f64,
        (None, _, _) => 0f64,
        _ => 0f64,
    }
}
