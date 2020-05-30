use std::collections::HashMap;

use chrono::Datelike;
use seed::{prelude::*, *};

use jirs_data::Issue;

use crate::model::Model;
use crate::shared::inner_layout;
use crate::Msg;

const SVG_MARGIN_X: u32 = 5;
const SVG_DRAWABLE_HEIGHT: u32 = 300;
const SVG_HEIGHT: u32 = SVG_DRAWABLE_HEIGHT + 30;
const SVG_WIDTH: u32 = 1060;
const SVG_BAR_WIDTH: u32 = 25;
const SVG_BAR_MARGIN: u32 = 10;

pub fn view(model: &Model) -> Node<Msg> {
    let body = section![
        h1![class!["header"], "Reports"],
        div![this_month_graph(model)],
    ];

    inner_layout(model, "reports", vec![body])
}

fn this_month_graph(model: &Model) -> Node<Msg> {
    let first_day = chrono::Utc::today().with_day(1).unwrap().naive_local();
    let last_day = (first_day + chrono::Duration::days(32))
        .with_day(1)
        .unwrap()
        - chrono::Duration::days(1);

    let this_month_updated: Vec<&Issue> = model
        .issues
        .iter()
        .filter(|issue| issue.updated_at.date() >= first_day)
        .collect();

    let mut issues: HashMap<u32, Vec<&Issue>> = HashMap::new();

    let list: Vec<Node<Msg>> = this_month_updated
        .into_iter()
        .map(|issue| {
            let date = issue.updated_at.date();
            issues.entry(date.day0()).or_default().push(issue);
            let day = issue.updated_at.date().format("%Y-%m-%d").to_string();
            li![span![issue.title.as_str()], span![day.as_str()]]
        })
        .collect();

    let mut columns: Vec<Node<Msg>> = vec![];
    let x_origin: Node<Msg> = seed::rect![attrs![
        At::X => SVG_MARGIN_X,
        At::Y => SVG_HEIGHT - SVG_MARGIN_X - 20,
        At::Width => SVG_WIDTH - (SVG_MARGIN_X * 2),
        At::Height => 2,
        At::Style => "fill: var(--textDark);"
    ]];

    for day in (first_day.day0())..(last_day.day0()) {
        let x = (SVG_BAR_WIDTH * day) + (SVG_BAR_MARGIN * day) + (SVG_MARGIN_X * 2);
        let num_issues = issues.get(&day).map(|v| v.len()).unwrap_or_default() as u32;
        let height = num_issues * SVG_BAR_WIDTH;

        let day = first_day.with_day0(day).unwrap();

        columns.push(seed::rect![attrs![
            At::X => x,
            At::Y => SVG_DRAWABLE_HEIGHT - height, // reverse draw origin
            At::Width => SVG_BAR_WIDTH,
            At::Height => height,
            At::Style => "fill: rgb(255,0,0);",
            At::Title => format!("Number of issues: {}", num_issues),
        ]]);
        columns.push(seed::text![
            attrs![
                At::X => x,
                At::Y => SVG_HEIGHT,
                At::Style => "fill: var(--textDark); font-family: var(--font-regular); font-size: 10px;",
            ],
            day.format("%d/%m").to_string(),
        ]);
    }

    div![
        svg![
            attrs![At::Height => SVG_HEIGHT, At::Width => SVG_WIDTH],
            x_origin,
            columns,
        ],
        ul![list],
    ]
}
