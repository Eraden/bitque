use std::collections::HashMap;

use chrono::Datelike;
use seed::{prelude::*, *};

use jirs_data::Issue;

use crate::model::{Model, PageContent, ReportsPage};
use crate::shared::styled_icon::StyledIcon;
use crate::shared::{inner_layout, ToNode};
use crate::{Msg, PageChanged, ReportsPageChange};

const SVG_MARGIN_X: u32 = 10;
const SVG_DRAWABLE_HEIGHT: u32 = 300;
const SVG_HEIGHT: u32 = SVG_DRAWABLE_HEIGHT + 30;
const SVG_WIDTH: u32 = 1060;
const SVG_BAR_MARGIN: u32 = 10;

pub fn view(model: &Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::Reports(page) => page,
        _ => return empty![],
    };

    let this_month_updated = this_month_updated(model, page);
    let graph = this_month_graph(page, &this_month_updated);
    let list = issue_list(page, &this_month_updated);

    let body = section![class!["top"], h1![class!["header"], "Reports"], graph, list];

    inner_layout(model, "reports", vec![body])
}

fn this_month_graph(page: &Box<ReportsPage>, this_month_updated: &Vec<&Issue>) -> Node<Msg> {
    let mut dominant = 0;
    let mut issues: HashMap<u32, Vec<&Issue>> = HashMap::new();

    for issue in this_month_updated {
        let date = issue.updated_at.date();
        let v = issues.entry(date.day0()).or_default();
        v.push(issue);
        if dominant < v.len() {
            dominant = v.len();
        }
    }

    // take log10 from largest amount of issue to calculate how long text will be
    let legend_margin_width = (dominant as f64).log10() * SVG_MARGIN_X as f64;

    // shapes, groups and texts
    let mut svg_parts: Vec<Node<Msg>> = vec![];

    // each piece is part of column drawable view where number of parts depends on number of issues which have largest amount of issues
    let piece_height = SVG_DRAWABLE_HEIGHT as f64 / dominant as f64;
    // width reduces by legend divided by number of days
    let piece_width = (SVG_WIDTH as f64
        - legend_margin_width
        - SVG_MARGIN_X as f64
        - (SVG_BAR_MARGIN as f64 * page.last_day.day() as f64))
        / page.last_day.day() as f64;

    let resolution = 10;
    let mut legend_parts: Vec<Node<Msg>> = vec![];
    for y in 0..(resolution + 1) {
        let current = dominant as f64 * (y as f64 / resolution as f64);

        legend_parts.push(seed::text![
            attrs![
                At::X => 0,
                At::Y => SVG_DRAWABLE_HEIGHT as f64 - (current as f64 * piece_height)  + 12f64,
                At::Style => "fill: var(--textLight); font-family: var(--font-regular); font-size: 10px;",
            ],
            format!("{:.1}", current),
        ]);
        legend_parts.push(seed::rect![attrs![
            At::X =>  legend_margin_width + SVG_MARGIN_X as f64,
            At::Y => SVG_DRAWABLE_HEIGHT as f64 - (current as f64 * piece_height),
            At::Width => SVG_WIDTH as f64 - (legend_margin_width + SVG_MARGIN_X as f64),
            At::Height => 1,
            At::Style => "fill: var(--textLight);",
        ],]);
    }
    svg_parts.push(seed::g![legend_parts]);

    for day in (page.first_day.day0())..(page.last_day.day()) {
        let num_issues = issues.get(&day).map(|v| v.len()).unwrap_or_default() as u32;
        let x = (piece_width * day as f64)
            + (SVG_BAR_MARGIN * day) as f64
            + (legend_margin_width + SVG_MARGIN_X as f64);
        let height = num_issues as f64 * piece_height;

        let day = page.first_day.with_day0(day).unwrap();

        let on_hover: EventHandler<Msg> = mouse_ev(Ev::MouseEnter, move |_| {
            Some(Msg::PageChanged(PageChanged::Reports(
                ReportsPageChange::DayHovered(Some(day.clone())),
            )))
        });
        let on_blur: EventHandler<Msg> = mouse_ev(Ev::MouseLeave, move |_| {
            Some(Msg::PageChanged(PageChanged::Reports(
                ReportsPageChange::DayHovered(None),
            )))
        });
        let selected = page.selected_day.clone();
        let current_date = day.clone();
        let on_click: EventHandler<Msg> = mouse_ev(Ev::MouseLeave, move |_| {
            Some(Msg::PageChanged(PageChanged::Reports(
                ReportsPageChange::DaySelected(match selected {
                    Some(_) => None,
                    None => Some(current_date),
                }),
            )))
        });

        svg_parts.push(seed::g![
            seed::rect![
                on_hover,
                on_blur,
                on_click,
                attrs![
                    At::X => x,
                    At::Y => SVG_DRAWABLE_HEIGHT as f64 - height, // reverse draw origin
                    At::Width => piece_width,
                    At::Height => height,
                    At::Style => "fill: rgb(255, 0, 0);",
                    At::Title => format!("Number of issues: {}", num_issues),
                ]
            ],
            seed::text![
                attrs![
                    At::X => x,
                    At::Y => SVG_HEIGHT,
                    At::Style => "fill: var(--textLight); font-family: var(--font-regular); font-size: 10px;",
                ],
                day.format("%d/%m").to_string(),
            ]
        ]);
    }

    div![
        class!["graph"],
        h5![class!["graphHeader"], "Last updated"],
        svg![
            attrs![At::Height => SVG_HEIGHT, At::Width => SVG_WIDTH],
            svg_parts,
        ],
    ]
}

fn issue_list(page: &Box<ReportsPage>, this_month_updated: &Vec<&Issue>) -> Node<Msg> {
    let mut children: Vec<Node<Msg>> = vec![];
    for issue in this_month_updated.into_iter() {
        let date = issue.updated_at.date();
        let day = date.format("%Y-%m-%d").to_string();
        let active_class = match (page.hovered_day.as_ref(), page.selected_day.as_ref()) {
            (Some(d), _) if *d == date => "selected",
            (_, Some(d)) if *d == date => "selected",
            (Some(_), _) | (_, Some(_)) => "nonSelected",
            _ => "",
        };
        let Issue {
            title,
            issue_type,
            priority,
            description,
            issue_status_id: _,
            ..
        } = issue;
        let type_icon = StyledIcon::build(issue_type.clone().into())
            .build()
            .into_node();
        let priority_icon = StyledIcon::build(priority.clone().into())
            .build()
            .into_node();
        children.push(li![
            class!["issue"],
            class![active_class],
            span![class!["priority"], priority_icon],
            span![class!["type"], type_icon],
            span![class!["name"], title.as_str()],
            span![
                class!["desc"],
                description.as_ref().cloned().unwrap_or_default()
            ],
            span![class!["updatedAt"], day.as_str()],
        ]);
    }
    div![
        class!["issueList"],
        h5![class!["issueListHeader"], "Issues this month"],
        children
    ]
}

fn this_month_updated<'a>(model: &'a Model, page: &Box<ReportsPage>) -> Vec<&'a Issue> {
    model
        .issues
        .iter()
        .filter(|issue| {
            issue.updated_at.date() >= page.first_day && issue.updated_at.date() <= page.last_day
        })
        .collect()
}
