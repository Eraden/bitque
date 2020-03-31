use seed::{prelude::*, *};

use crate::model::{Icon, Model};
use crate::shared::styled_button::{StyledButton, Variant};
use crate::shared::{styled_tooltip, ToNode};
use crate::Msg;

pub fn render(model: &Model) -> Vec<Node<Msg>> {
    let logo_svg = Node::from_html(include_str!("../../static/logo.svg"));

    vec![
        about_tooltip_popup(model),
        aside![
            id!["navbar-left"],
            a![
                attrs![At::Class => "logoLink", At::Href => "/"],
                div![attrs![At::Class => "styledLogo"], logo_svg]
            ],
            navbar_left_item(model, "Search issues", Icon::Search),
            navbar_left_item(model, "Create Issue", Icon::Plus),
            div![
                attrs![At::Class => "bottom"],
                about_tooltip(model, navbar_left_item(model, "About", Icon::Help)),
            ],
        ],
    ]
}

fn navbar_left_item(_model: &Model, text: &str, logo: Icon) -> Node<Msg> {
    div![
        attrs![At::Class => "item"],
        i![attrs![At::Class => format!("styledIcon {}", logo)]],
        span![attrs![At::Class => "itemText"], text]
    ]
}

pub fn about_tooltip(_model: &Model, children: Node<Msg>) -> Node<Msg> {
    div![
        attrs![At::Class => "aboutTooltip"],
        ev(Ev::Click, |_| Msg::ToggleAboutTooltip),
        children
    ]
}

fn about_tooltip_popup(model: &Model) -> Node<Msg> {
    let visit_website = StyledButton {
        text: Some("Visit Website".to_string()),
        variant: Variant::Primary,
        disabled: false,
        active: false,
        icon_only: false,
        icon: None,
        on_click: None,
    }
    .into_node();
    let github_repo = StyledButton {
        text: Some("Github Repo".to_string()),
        variant: Variant::Secondary,
        disabled: false,
        active: false,
        icon_only: false,
        icon: Some(Icon::Github),
        on_click: None,
    }
    .into_node();

    styled_tooltip::StyledTooltip {
        visible: model.project_page.about_tooltip_visible,
        class_name: "aboutTooltipPopup".to_string(),
        children: div![
        ev(Ev::Click, |_| Msg::ToggleAboutTooltip),
        attrs![At::Class => "feedbackDropdown"],
        div![
            attrs![At::Class => "feedbackImageCont"],
            img![attrs![At::Src => "/feedback.png", At::Class => "feedbackImage"]]
        ],
        div![
            attrs![At::Class => "feedbackParagraph"],
            "This simplified Jira clone is built with Seed.rs on the front-end and Actix-Web on the back-end."
        ],
        div![
            attrs![At::Class => "feedbackParagraph"],
            "Read more on my website or reach out via ",
            a![
                attrs![At::Href => "mailto:adrian.wozniak@ita-prog.pl"],
                strong!["adrian.wozniak@ita-prog.pl"]
            ]
        ],
        a![
            attrs![
                At::Href => "https://gitlab.com/adrian.wozniak/jirs",
                At::Target => "_blank",
                At::Rel => "noreferrer noopener",
            ],
            visit_website,
      ],
      a![
        id!["about-github-button"],
        attrs![
            At::Href => "https://gitlab.com/adrian.wozniak/jirs",
            At::Target => "_blank",
            At::Rel => "noreferrer noopener",
        ],
        github_repo
      ]
    ],
    }.into_node()
}
