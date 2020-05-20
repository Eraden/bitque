use seed::{prelude::*, *};

use crate::model::Model;
use crate::shared::styled_avatar::StyledAvatar;
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::{styled_tooltip, ToNode};
use crate::Msg;

trait IntoNavItemIcon {
    fn into_nav_item_icon(self) -> Node<Msg>;
}

impl IntoNavItemIcon for Node<Msg> {
    fn into_nav_item_icon(self) -> Node<Msg> {
        self
    }
}

impl IntoNavItemIcon for Icon {
    fn into_nav_item_icon(self) -> Node<Msg> {
        StyledIcon::build(self).size(21).build().into_node()
    }
}

pub fn render(model: &Model) -> Vec<Node<Msg>> {
    let logo_svg = Node::from_html(include_str!("../../static/logo.svg"));

    let user_icon = match model.user.as_ref() {
        Some(user) => i![
            class!["styledIcon"],
            StyledAvatar::build()
                .size(27)
                .name(user.name.as_str())
                .avatar_url(user.avatar_url.as_ref().cloned().unwrap_or_default())
                .build()
                .into_node()
        ],
        _ => StyledIcon::build(Icon::User).size(21).build().into_node(),
    };

    let messages = if model.messages.is_empty() {
        empty![]
    } else {
        navbar_left_item("Messages", Icon::Message, None)
    };

    vec![
        about_tooltip_popup(model),
        aside![
            id!["navbar-left"],
            a![
                attrs![At::Class => "logoLink", At::Href => "/"],
                div![attrs![At::Class => "styledLogo"], logo_svg]
            ],
            navbar_left_item("Search issues", Icon::Search, None),
            navbar_left_item("Create Issue", Icon::Plus, Some("/add-issue")),
            div![
                class!["bottom"],
                navbar_left_item("Profile", user_icon, Some("/profile")),
                messages,
                about_tooltip(model, navbar_left_item("About", Icon::Help, None)),
            ],
        ],
    ]
}

fn navbar_left_item<I>(text: &str, icon: I, href: Option<&str>) -> Node<Msg>
where
    I: IntoNavItemIcon,
{
    let styled_icon = icon.into_nav_item_icon();
    let href = href.unwrap_or_else(|| "#");
    a![
        class!["item"],
        attrs![At::Href => href],
        styled_icon,
        span![class!["itemText"], text]
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
    let visit_website = StyledButton::build()
        .text("Visit Website".to_string())
        .primary()
        .build()
        .into_node();
    let github_repo = StyledButton::build()
        .text("Github Repo".to_string())
        .secondary()
        .icon(Icon::Github)
        .build()
        .into_node();

    styled_tooltip::StyledTooltip {
        visible: model.about_tooltip_visible,
        class_name: "aboutTooltipPopup".to_string(),
        children: div![
        ev(Ev::Click, |_| Msg::ToggleAboutTooltip),
        attrs![At::Class => "feedbackDropdown"],
        div![
            attrs![At::Class => "feedbackImageCont"],
            img![attrs![At::Src => "/feedback.png"]],
            class!["feedbackImage"],
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
