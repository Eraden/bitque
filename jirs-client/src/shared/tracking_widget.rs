use jirs_data::{TimeTracking, UpdateIssuePayload};
use seed::prelude::*;
use seed::*;

use crate::components::styled_icon::{Icon, StyledIcon};
use crate::modals::issues_edit::Model as EditIssueModal;
use crate::modals::time_tracking::value_for_time_tracking;
use crate::model::{ModalType, Model};
use crate::Msg;

#[inline]
pub fn fibonacci_values() -> Vec<u32> {
    vec![0, 1, 2, 3, 5, 8, 13, 21, 34, 55]
}

#[inline]
pub fn fibonacci_value_name<'l>(v: u32) -> &'l str {
    match v {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        5 => "5",
        8 => "8",
        13 => "13",
        21 => "21",
        34 => "34",
        55 => "55",
        _ => unreachable!(),
    }
}

pub fn tracking_link(model: &Model, modal: &crate::modals::issues_edit::Model) -> Node<Msg> {
    let crate::modals::issues_edit::Model { id, .. } = modal;

    let issue_id = *id;

    let handler = mouse_ev(Ev::Click, move |_| {
        Msg::ModalOpened(ModalType::TimeTracking(Some(issue_id)))
    });

    div![C!["trackingLink"], handler, tracking_widget(model, modal),]
}

pub fn tracking_widget(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    let time_tracking_type = model
        .project
        .as_ref()
        .map(|p| p.time_tracking)
        .unwrap_or_else(|| TimeTracking::Untracked);
    if time_tracking_type == TimeTracking::Untracked {
        return empty![];
    }
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

    let icon = StyledIcon {
        icon: Icon::Stopwatch,
        class_list: "watchIcon",
        size: Some(32),
        ..Default::default()
    }
    .render();
    let bar_width = calc_bar_width(*estimate, *time_spent, *time_remaining);

    let spent_text = match (time_spent, time_tracking_type) {
        (Some(time), TimeTracking::Hourly) => format!(
            "{}h logged",
            value_for_time_tracking(&Some(*time), &time_tracking_type)
        ),
        (Some(time), TimeTracking::Fibonacci) => format!(
            "{} point logged",
            value_for_time_tracking(&Some(*time), &time_tracking_type)
        ),
        _ => "No time logged".to_string(),
    };

    let remaining_node: Node<Msg> = remaining_node(time_remaining, estimate, time_tracking_type);

    div![
        C!["trackingWidget"],
        icon,
        div![
            C!["right"],
            div![
                C!["barCounter"],
                div![
                    C!["bar"],
                    attrs![At::Style => format!("width: {}%", bar_width)]
                ]
            ],
            div![C!["values"], div![spent_text], remaining_node,]
        ]
    ]
}

#[inline]
fn remaining_node(
    time_remaining: &Option<i32>,
    estimate: &Option<i32>,
    time_tracking_type: TimeTracking,
) -> Node<Msg> {
    let text = match (time_remaining, estimate, time_tracking_type) {
        (Some(n), _, TimeTracking::Hourly) => format!(
            "{}h remaining",
            value_for_time_tracking(&Some(*n), &time_tracking_type)
        ),
        (_, Some(n), TimeTracking::Hourly) => format!(
            "{}h estimated",
            value_for_time_tracking(&Some(*n), &time_tracking_type)
        ),
        (Some(n), _, TimeTracking::Fibonacci) => format!("{} remaining", n),
        (_, Some(n), TimeTracking::Fibonacci) => format!("{} estimated", n),
        _ => return empty![],
    };
    div![text]
}

#[inline]
fn calc_bar_width(
    estimate: Option<i32>,
    time_spent: Option<i32>,
    time_remaining: Option<i32>,
) -> f64 {
    match (estimate, time_spent, time_remaining) {
        (_, Some(spent), Some(remaining)) => {
            ((spent as f64 / (spent + remaining) as f64) * 100f64).min(100f64)
        }
        (Some(estimate), Some(spent), _) => ((spent / estimate) as f64 * 100f64).min(100f64),
        (None, None, _) => 100f64,
        (None, _, _) => 0f64,
        _ => 0f64,
    }
}
