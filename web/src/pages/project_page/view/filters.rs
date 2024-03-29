use seed::prelude::*;
use seed::*;

use super::super::events;
use crate::components::styled_avatar::*;
use crate::components::styled_button::*;
use crate::components::styled_icon::*;
use crate::components::styled_input::*;
use crate::model::PageContent;
use crate::{FieldId, Model, Msg};

pub fn project_board_filters(model: &Model) -> Node<Msg> {
    let project_page = match &model.page_content {
        PageContent::Project(page_content) => page_content,
        _ => return empty![],
    };

    let search_input = StyledInput {
        value: project_page.text_filter.as_str(),
        valid: true,
        id: Some(FieldId::TextFilterBoard),
        icon: Some(Icon::Search),
        ..Default::default()
    }
    .render();

    let only_my = StyledButton {
        variant: ButtonVariant::Empty,
        active: project_page.only_my_filter,
        text: Some("Only My Issues"),
        class_list: "filterChild",
        on_click: Some(events::on_click_toggle_only_my()),
        ..Default::default()
    }
    .render();

    let recently_updated = StyledButton {
        variant: ButtonVariant::Empty,
        text: Some("Recently Updated"),
        class_list: "filterChild",
        on_click: Some(events::on_click_toggle_recent()),
        ..Default::default()
    }
    .render();

    let clear_all = if project_page.only_my_filter
        || project_page.recently_updated_filter
        || !project_page.active_avatar_filters.is_empty()
    {
        seed::button![
            id!["clearAllFilters"],
            C!["filterChild"],
            "Clear all",
            events::on_click_clear_filters()
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
    let avatars = model
        .user_ids
        .iter()
        .filter_map(|id| model.users_by_id.get(id))
        .enumerate()
        .map(|(idx, user)| {
            let user_id = user.id;
            let active = active_avatar_filters.contains(&user_id);
            let styled_avatar = StyledAvatar {
                avatar_url: user.avatar_url.as_deref(),
                name: &user.name,
                on_click: Some(events::on_click_filter_by_user(user_id, active)),
                user_index: idx,
                ..StyledAvatar::default()
            }
            .render();
            div![
                IF![active => C!["isActive"]],
                C!["avatarIsActiveBorder"],
                styled_avatar
            ]
        });

    div![id!["avatars"], C!["filterChild"], avatars]
}
