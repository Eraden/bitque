use {
    crate::{
        components::{
            styled_avatar::StyledAvatar,
            styled_button::StyledButton,
            styled_icon::{Icon, StyledIcon},
            styled_tooltip,
        },
        model::Model,
        shared::{divider, ToNode},
        ws::send_ws_msg,
        Msg, Page,
    },
    jirs_data::{InvitationToken, Message, MessageType, WsMsg},
    seed::{prelude::*, *},
};

use crate::components::styled_button::ButtonVariant;
use crate::components::styled_tooltip::{StyledTooltip, TooltipVariant};

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
        StyledIcon {
            icon: self,
            size: Some(21),
            ..Default::default()
        }
        .into_node()
    }
}

pub fn update(msg: &Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let m = match msg {
        Msg::MessageInvitationApproved(token) => WsMsg::InvitationAcceptRequest(*token),
        Msg::MessageInvitationDismiss(token) => WsMsg::InvitationRejectRequest(*token),
        Msg::MessageSeen(id) => WsMsg::MessageMarkSeen(*id),
        _ => return,
    };
    send_ws_msg(m, model.ws.as_ref(), orders);
}

pub fn render(model: &Model) -> Vec<Node<Msg>> {
    let logo_svg = img![
        attrs![At::Src => "/logo2.svg"; At::Style => "background: rgba(244,244,244,.8); border-radius: 24px;"]
    ];

    let user_icon = match model.user.as_ref() {
        Some(user) => {
            let avatar = StyledAvatar {
                avatar_url: user.avatar_url.as_deref(),
                size: 27,
                name: &user.name,
                ..StyledAvatar::default()
            }
            .into_node();
            i![C!["styledIcon"], avatar]
        }
        _ => StyledIcon {
            icon: Icon::User,
            size: Some(21),
            ..Default::default()
        }
        .into_node(),
    };

    let messages = if model.messages.is_empty() {
        empty![]
    } else {
        navbar_left_item(
            "Messages",
            Icon::Message,
            None,
            Some(mouse_ev(Ev::Click, |ev| {
                ev.prevent_default();
                Msg::ToggleTooltip(styled_tooltip::TooltipVariant::Messages)
            })),
        )
    };

    let issue_nav = if model.issue_statuses.is_empty() {
        vec![]
    } else {
        vec![
            navbar_left_item("Search issues", Icon::Search, None, None),
            navbar_left_item(
                "Create Issue",
                Icon::Plus,
                Some("/add-issue"),
                Some(mouse_ev("click", |ev| {
                    ev.stop_propagation();
                    ev.prevent_default();
                    Msg::ChangePage(Page::AddIssue)
                })),
            ),
        ]
    };
    let go_to_profile = mouse_ev("click", move |ev| {
        ev.stop_propagation();
        ev.prevent_default();
        seed::Url::new().add_path_part("profile").go_and_push();
        Msg::ChangePage(Page::Profile)
    });

    vec![
        about_tooltip_popup(model),
        messages_tooltip_popup(model),
        aside![
            id!["navbar-left"],
            a![
                C!["logoLink"],
                attrs![At::Href => "/"],
                div![C!["styledLogo"], logo_svg]
            ],
            issue_nav,
            div![
                C!["bottom"],
                navbar_left_item("Profile", user_icon, Some("/profile"), Some(go_to_profile)),
                messages,
                IF![model.show_extras => about_tooltip(
                    model,
                    navbar_left_item("About", Icon::Help, None, None)
                )],
            ],
        ],
    ]
}

#[inline]
fn navbar_left_item<I>(
    text: &str,
    icon: I,
    href: Option<&str>,
    on_click: Option<EventHandler<Msg>>,
) -> Node<Msg>
where
    I: IntoNavItemIcon,
{
    let styled_icon = icon.into_nav_item_icon();

    a![
        C!["item"],
        attrs![At::Href => href.unwrap_or("#")],
        styled_icon,
        span![C!["itemText"], text],
        on_click,
    ]
}

pub fn about_tooltip(_model: &Model, children: Node<Msg>) -> Node<Msg> {
    let on_click: EventHandler<Msg> = ev(Ev::Click, move |_| {
        Some(Msg::ToggleTooltip(styled_tooltip::TooltipVariant::About))
    });
    div![C!["aboutTooltip"], on_click, children]
}

fn messages_tooltip_popup(model: &Model) -> Node<Msg> {
    let on_click: EventHandler<Msg> = ev(Ev::Click, move |_| {
        Some(Msg::ToggleTooltip(styled_tooltip::TooltipVariant::Messages))
    });
    let mut messages: Vec<Node<Msg>> = vec![];
    for (idx, message) in model.messages.iter().enumerate() {
        if let Some(message_ui) = message_ui(model, message) {
            messages.push(message_ui);
            if idx != model.messages.len() - 1 {
                messages.push(divider());
            }
        };
    }
    let body = div![on_click, C!["messagesList"], messages];
    styled_tooltip::StyledTooltip {
        visible: model.messages_tooltip_visible,
        class_list: "messagesPopup",
        children: vec![body],
        variant: TooltipVariant::Messages,
    }
    .into_node()
}

fn message_ui(model: &Model, message: &Message) -> Option<Node<Msg>> {
    let Message {
        id,
        summary,
        description,
        message_type,
        hyper_link,
        ..
    } = message;
    let message_id = *id;

    let hyperlink = if hyper_link.is_empty() && !hyper_link.starts_with('#') {
        empty![]
    } else {
        let link_icon = StyledIcon::from(Icon::Link).into_node();
        div![
            C!["hyperlink"],
            a![
                C!["styledLink"],
                attrs![At::Href => hyper_link],
                link_icon,
                hyper_link
            ]
        ]
    };

    let message_description = parse_description(model, description.as_str());
    let close_button = StyledButton {
        variant: ButtonVariant::Empty,
        icon: Some(Icon::Close.into_node()),
        on_click: Some(mouse_ev(Ev::Click, move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Some(Msg::MessageSeen(message_id))
        })),
        ..Default::default()
    }
    .into_node();
    let top = div![
        C!["top"],
        div![C!["summary"], summary],
        div![C!["action"], close_button],
    ];

    let node = match message_type {
        MessageType::ReceivedInvitation => {
            let token: InvitationToken = hyper_link.trim_start_matches('#').parse().ok()?;
            let accept = StyledButton {
                variant: ButtonVariant::Primary,
                active: true,
                text: Some("Accept"),
                icon: Some(Icon::Check.into_node()),
                on_click: Some(mouse_ev(Ev::Click, move |ev| {
                    ev.stop_propagation();
                    ev.prevent_default();
                    Some(Msg::MessageInvitationApproved(token))
                })),
                ..Default::default()
            }
            .into_node();
            let reject = StyledButton {
                variant: ButtonVariant::Danger,
                active: true,
                text: Some("Dismiss"),
                icon: Some(Icon::Close.into_node()),
                on_click: Some(mouse_ev(Ev::Click, move |ev| {
                    ev.stop_propagation();
                    ev.prevent_default();
                    Some(Msg::MessageInvitationDismiss(token))
                })),
                ..Default::default()
            }
            .into_node();
            div![
                C!["message"],
                attrs![At::Class => format!("{}", message_type)],
                top,
                div![C!["description"], message_description],
                div![C!["actions"], accept, reject],
            ]
        }
        MessageType::AssignedToIssue => div![
            C!["message assignedToIssue"],
            top,
            div![C!["description"], message_description],
            hyperlink,
        ],
        MessageType::Mention => div![
            C!["message mention"],
            top,
            div![C!["description"], message_description],
            hyperlink,
        ],
    };
    Some(node)
}

fn about_tooltip_popup(model: &Model) -> Node<Msg> {
    let visit_website = StyledButton {
        variant: ButtonVariant::Primary,
        text: Some("Visit Website"),
        ..Default::default()
    }
    .into_node();
    let github_repo = StyledButton {
        variant: ButtonVariant::Secondary,
        text: Some("Github Repo"),
        icon: Some(Icon::Github.into_node()),
        ..Default::default()
    }
    .into_node();

    let on_click = mouse_ev(Ev::Click, |_| {
        Msg::ToggleTooltip(styled_tooltip::TooltipVariant::About)
    });
    let body = div![
        on_click,
        C!["feedbackDropdown"],
        div![
            C!["feedbackImageCont feedbackImage"],
            img![attrs![At::Src => "/feedback.png"]],
        ],
        div![
            C!["feedbackParagraph"],
            "This simplified Jira clone is built with Seed.rs on the front-end and Actix-Web on the back-end."
        ],
        div![
            C!["feedbackParagraph"],
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
    ];

    StyledTooltip {
        visible: model.about_tooltip_visible,
        class_list: "aboutTooltipPopup",
        children: vec![body],
        variant: TooltipVariant::About,
    }
    .into_node()
}

fn parse_description(model: &Model, desc: &str) -> Node<Msg> {
    let mut container: Node<Msg> = div![];
    for word in desc.split(' ') {
        let child = parse_email(word)
            .and_then(|email| {
                model
                    .users
                    .iter()
                    .enumerate()
                    .find(|(_, user)| user.email == email)
            })
            .map(|(index, user)| {
                let avatar = StyledAvatar {
                    avatar_url: user.avatar_url.as_deref(),
                    size: 16,
                    user_index: index,
                    ..StyledAvatar::default()
                }
                .into_node();
                span![C!["mention"], avatar, user.name.as_str()]
            })
            .unwrap_or_else(|| span![word]);
        container.add_child(child).add_text(" ");
    }
    container
}

fn parse_email(word: &str) -> Option<&str> {
    if word.starts_with("@<") && word.ends_with('>') {
        Some(&word[2..(word.len() - 1)])
    } else {
        None
    }
}
