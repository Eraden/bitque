use {
    crate::{
        components::{styled_icon::StyledIcon, styled_link::*},
        model::{Model, PageContent},
        pages::reports_page::model::ReportsPage,
        shared::{inner_layout, ToNode},
        Msg, PageChanged, ReportsPageChange,
    },
    chrono::Datelike,
    jirs_data::Issue,
    seed::{prelude::*, *},
    std::collections::HashMap,
};

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

    let project_name = model
        .project
        .as_ref()
        .map(|p| p.name.as_str())
        .unwrap_or_default();
    let this_month_updated = this_month_updated(model, page);
    let graph = this_month_graph(page, &this_month_updated);
    let list = issue_list(page, project_name, this_month_updated.as_slice());

    let body = section![C!["top"], h1![C!["header"], "Reports"], graph, list];

    inner_layout(model, "reports", &[body])
}

fn this_month_graph(page: &ReportsPage, this_month_updated: &[&Issue]) -> Node<Msg> {
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

        let day = page.first_day.clone().with_day0(day).unwrap();

        let on_hover = mouse_ev(Ev::MouseEnter, move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::PageChanged(PageChanged::Reports(ReportsPageChange::DayHovered(Some(
                day,
            ))))
        });

        let on_blur = mouse_ev(Ev::MouseLeave, move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::PageChanged(PageChanged::Reports(ReportsPageChange::DayHovered(None)))
        });

        let selected = page.selected_day;
        let current_date = day;
        let on_click = mouse_ev("click", move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::PageChanged(PageChanged::Reports(ReportsPageChange::DaySelected(
                match selected {
                    Some(_) => None,
                    None => Some(current_date),
                },
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
        C!["graph"],
        h5![C!["graphHeader"], "Last updated"],
        svg![
            attrs![At::Height => SVG_HEIGHT, At::Width => SVG_WIDTH],
            svg_parts,
        ],
    ]
}

#[derive(PartialEq)]
enum SelectionState {
    Inactive,
    Selected,
    NotSelected,
}

impl SelectionState {
    fn to_str(&self) -> &str {
        match self {
            SelectionState::Inactive => "",
            SelectionState::Selected => "selected",
            SelectionState::NotSelected => "nonSelected",
        }
    }
}

fn issue_list(page: &ReportsPage, project_name: &str, this_month_updated: &[&Issue]) -> Node<Msg> {
    let children: Vec<Node<Msg>> = this_month_updated
        .iter()
        .map(|issue| {
            let date = issue.updated_at.date();

            let selection_state = match (page.hovered_day.as_ref(), page.selected_day.as_ref()) {
                (Some(d), _) if *d == date => SelectionState::Selected,
                (_, Some(d)) if *d == date => SelectionState::Selected,
                (Some(_), _) | (_, Some(_)) => SelectionState::NotSelected,
                _ => SelectionState::Inactive,
            };

            let Issue {
                id,
                title,
                issue_type,
                priority,
                description,
                ..
            } = issue;
            let day = date.format("%Y-%m-%d").to_string();

            let type_icon = StyledIcon::build(issue_type.clone().into())
                .build()
                .into_node();
            let priority_icon = StyledIcon::build(priority.clone().into())
                .build()
                .into_node();
            let desc = Node::from_html(
                description
                    .as_deref()
                    .unwrap_or_default()
            );
            let link = StyledLink::build()
                .with_icon()
                .text(format!("{}-{}", project_name, id).as_str())
                .href(format!("/issues/{}", id).as_str())
                .build()
                .into_node();

            li![
                C!["issue"],
                C![selection_state.to_str()],
                div![C!["number"], link],
                div![C!["type"], type_icon],
                IF!( selection_state != SelectionState::NotSelected => div![C!["priority"], priority_icon]),
                IF!( selection_state != SelectionState::NotSelected => div![C!["name"], title.as_str()]),
                IF!( selection_state != SelectionState::NotSelected => div![C!["desc"], desc]),
                IF!( selection_state != SelectionState::NotSelected => div![C!["updatedAt"], day.as_str()]),
            ]
        })
        .collect();
    div![
        C!["issueList"],
        h5![C!["issueListHeader"], "Issues this month"],
        children
    ]
}

fn this_month_updated<'a>(model: &'a Model, page: &ReportsPage) -> Vec<&'a Issue> {
    model
        .issues()
        .iter()
        .filter(|issue| {
            issue.updated_at.date() >= page.first_day && issue.updated_at.date() <= page.last_day
        })
        .collect()
}
