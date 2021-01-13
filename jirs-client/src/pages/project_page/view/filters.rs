use {
    crate::{
        model::PageContent,
        shared::{styled_avatar::*, styled_button::*, styled_icon::*, styled_input::*, ToNode},
        FieldId, Model, Msg,
    },
    seed::{prelude::*, *},
};

pub fn project_board_filters(model: &Model) -> Node<Msg> {
    let project_page = match &model.page_content {
        PageContent::Project(page_content) => page_content,
        _ => return empty![],
    };

    let search_input = StyledInput::build()
        .icon(Icon::Search)
        .valid(true)
        .value(project_page.text_filter.as_str())
        .build(FieldId::TextFilterBoard)
        .into_node();

    let only_my = StyledButton::build()
        .empty()
        .active(project_page.only_my_filter)
        .text("Only My Issues")
        .add_class("filterChild")
        .on_click(mouse_ev(Ev::Click, |_| Msg::ProjectToggleOnlyMy))
        .build()
        .into_node();

    let recently_updated = StyledButton::build()
        .empty()
        .text("Recently Updated")
        .add_class("filterChild")
        .on_click(mouse_ev(Ev::Click, |_| Msg::ProjectToggleRecentlyUpdated))
        .build()
        .into_node();

    let clear_all = if project_page.only_my_filter
        || project_page.recently_updated_filter
        || !project_page.active_avatar_filters.is_empty()
    {
        seed::button![
            id!["clearAllFilters"],
            C!["filterChild"],
            "Clear all",
            mouse_ev(Ev::Click, |_| Msg::ProjectClearFilters),
        ]
    } else {
        empty![]
    };

    div![
        id!["projectBoardFilters"],
        search_input,
        avatars_filters(model),
        only_my,
        recently_updated,
        clear_all
    ]
}

pub fn avatars_filters(model: &Model) -> Node<Msg> {
    let project_page = match &model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return empty![],
    };
    let active_avatar_filters = &project_page.active_avatar_filters;
    let avatars: Vec<Node<Msg>> = model
        .users
        .iter()
        .enumerate()
        .map(|(idx, user)| {
            let user_id = user.id;
            let active = active_avatar_filters.contains(&user_id);
            let styled_avatar = StyledAvatar::build()
                .avatar_url(user.avatar_url.as_deref().unwrap_or_default())
                .on_click(mouse_ev(Ev::Click, move |_| {
                    Msg::ProjectAvatarFilterChanged(user_id, active)
                }))
                .name(user.name.as_str())
                .user_index(idx)
                .build()
                .into_node();
            div![
                if active { Some(C!["isActive"]) } else { None },
                C!["avatarIsActiveBorder"],
                styled_avatar
            ]
        })
        .collect();

    div![id!["avatars"], C!["filterChild"], avatars]
}
