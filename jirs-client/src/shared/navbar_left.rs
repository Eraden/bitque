use seed::{prelude::*, *};

use jirs_data::{InvitationToken, Message, MessageType, WsMsg};

use crate::model::Model;
use crate::shared::styled_avatar::StyledAvatar;
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::{divider, styled_tooltip, ToNode};
use crate::ws::send_ws_msg;
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
    // let logo_svg = Node::from_html(include_str!("../../static/logo.svg"));

    let user_icon = match model.user.as_ref() {
        Some(user) => i![
            class!["styledIcon"],
            StyledAvatar::build()
                .size(27)
                .name(user.name.as_str())
                .avatar_url(user.avatar_url.as_deref().unwrap_or_default())
                .build()
                .into_node()
        ],
        _ => StyledIcon::build(Icon::User).size(21).build().into_node(),
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
                Msg::ToggleTooltip(styled_tooltip::Variant::Messages)
            })),
        )
    };

    let issue_nav = if model.issue_statuses.is_empty() {
        vec![]
    } else {
        vec![
            navbar_left_item("Search issues", Icon::Search, None, None),
            navbar_left_item("Create Issue", Icon::Plus, Some("/add-issue"), None),
        ]
    };

    vec![
        about_tooltip_popup(model),
        messages_tooltip_popup(model),
        aside![
            id!["navbar-left"],
            a![
                class!["logoLink"],
                attrs![At::Href => "/"],
                div![class!["styledLogo"], logo_svg]
            ],
            issue_nav,
            div![
                class!["bottom"],
                navbar_left_item("Profile", user_icon, Some("/profile"), None),
                messages,
                about_tooltip(model, navbar_left_item("About", Icon::Help, None, None)),
            ],
        ],
    ]
}

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
    let href = href.unwrap_or_else(|| "#");

    a![
        class!["item"],
        attrs![At::Href => href],
        styled_icon,
        span![class!["itemText"], text],
        on_click,
    ]
}

pub fn about_tooltip(_model: &Model, children: Node<Msg>) -> Node<Msg> {
    let on_click: EventHandler<Msg> = ev(Ev::Click, move |_| {
        Some(Msg::ToggleTooltip(styled_tooltip::Variant::About))
    });
    div![class!["aboutTooltip"], on_click, children]
}

fn messages_tooltip_popup(model: &Model) -> Node<Msg> {
    let on_click: EventHandler<Msg> = ev(Ev::Click, move |_| {
        Some(Msg::ToggleTooltip(styled_tooltip::Variant::Messages))
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
    let body = div![on_click, class!["messagesList"], messages];
    styled_tooltip::StyledTooltip::build()
        .add_class("messagesPopup")
        .visible(model.messages_tooltip_visible)
        .messages_tooltip()
        .add_child(body)
        .build()
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
        let link_icon = StyledIcon::build(Icon::Link).build().into_node();
        div![
            class!["hyperlink"],
            a![
                class!["styledLink"],
                attrs![At::Href => hyper_link],
                link_icon,
                hyper_link
            ]
        ]
    };

    let message_description = parse_description(model, description.as_str());
    let close_button = StyledButton::build()
        .icon(Icon::Close)
        .empty()
        .on_click(mouse_ev(Ev::Click, move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Some(Msg::MessageSeen(message_id))
        }))
        .build()
        .into_node();
    let top = div![
        class!["top"],
        div![class!["summary"], summary],
        div![class!["action"], close_button],
    ];

    let node = match message_type {
        MessageType::ReceivedInvitation => {
            let token: InvitationToken = hyper_link.trim_start_matches('#').parse().ok()?;
            let accept = StyledButton::build()
                .primary()
                .text("Accept")
                .active(true)
                .icon(Icon::Check)
                .on_click(mouse_ev(Ev::Click, move |ev| {
                    ev.stop_propagation();
                    ev.prevent_default();
                    Some(Msg::MessageInvitationApproved(token))
                }))
                .build()
                .into_node();
            let reject = StyledButton::build()
                .danger()
                .text("Dismiss")
                .icon(Icon::Close)
                .on_click(mouse_ev(Ev::Click, move |ev| {
                    ev.stop_propagation();
                    ev.prevent_default();
                    Some(Msg::MessageInvitationDismiss(token))
                }))
                .active(true)
                .build()
                .into_node();
            div![
                class!["message"],
                attrs![At::Class => format!("{}", message_type)],
                top,
                div![class!["description"], message_description],
                div![class!["actions"], accept, reject],
            ]
        }
        MessageType::AssignedToIssue => div![
            class!["message assignedToIssue"],
            top,
            div![class!["description"], message_description],
            hyperlink,
        ],
        MessageType::Mention => div![
            class!["message mention"],
            top,
            div![class!["description"], message_description],
            hyperlink,
        ],
    };
    Some(node)
}

fn about_tooltip_popup(model: &Model) -> Node<Msg> {
    let visit_website = StyledButton::build()
        .text("Visit Website")
        .primary()
        .build()
        .into_node();
    let github_repo = StyledButton::build()
        .text("Github Repo")
        .secondary()
        .icon(Icon::Github)
        .build()
        .into_node();

    let on_click = mouse_ev(Ev::Click, |_| {
        Msg::ToggleTooltip(styled_tooltip::Variant::About)
    });
    let body = div![
        on_click,
        class!["feedbackDropdown"],
        div![
            class!["feedbackImageCont"],
            img![attrs![At::Src => "/feedback.png"]],
            class!["feedbackImage"],
        ],
        div![
            class!["feedbackParagraph"],
            "This simplified Jira clone is built with Seed.rs on the front-end and Actix-Web on the back-end."
        ],
        div![
            class!["feedbackParagraph"],
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

    styled_tooltip::StyledTooltip::build()
        .visible(model.about_tooltip_visible)
        .about_tooltip()
        .add_class("aboutTooltipPopup")
        .add_child(body)
        .build()
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
                let avatar = StyledAvatar::build()
                    .avatar_url(user.avatar_url.as_deref().unwrap_or_default())
                    .user_index(index)
                    .size(16)
                    .build()
                    .into_node();
                span![class!["mention"], avatar, user.name.as_str()]
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
