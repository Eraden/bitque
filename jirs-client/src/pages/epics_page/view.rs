use chrono::NaiveDateTime;
use seed::prelude::*;
use seed::*;

use crate::model::Model;
use crate::shared::inner_layout;
use crate::Msg;

pub fn view(model: &Model) -> Node<Msg> {
    let epics: Vec<Node<Msg>> = model
        .epics
        .iter()
        .map(|epic| {
            li![
                C!["epic"],
                div![C!["epicName"], &epic.name],
                div![
                    C!["date"],
                    div![
                        C!["startsAt"],
                        span!["Stats At:"],
                        span![epic
                            .starts_at
                            .as_ref()
                            .map(|d| format!("{}", d.format("%e %B %Y")))
                            .unwrap_or_default()]
                    ],
                    div![
                        C!["endsAt"],
                        span!["Ends At: "],
                        span![epic
                            .ends_at
                            .as_ref()
                            .map(|d| format!("{}", d.format("%e %B %Y")))
                            .unwrap_or_default()]
                    ]
                ]
            ]
        })
        .collect();

    inner_layout(
        model,
        "epics",
        &[section![
            h1!["Epics"],
            p!["Epics and issues grouped in them"],
            ul![C!["epicsList"], epics]
        ]],
    )
}

fn date_field(name: &str, date: Option<&NaiveDateTime>) -> Node<Msg> {
    match date {
        _ => Node::Empty,
    }
}
