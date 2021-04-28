use seed::prelude::*;
use seed::*;

use crate::components::styled_button::ButtonVariant;
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_select::{SelectVariant, StyledSelect};
use crate::components::styled_select_child::StyledSelectOption;
use crate::model::Model;
use crate::pages::issues_and_filters::{FieldOption, IssuesAndFiltersPage, JqlPart};
use crate::styled_button::StyledButton;
use crate::Msg;

pub fn filters(model: &Model, page: &IssuesAndFiltersPage) -> Node<Msg> {
    let search_input = pseudo_input(model, page);
    div![C!["filters"], search_input]
}

fn pseudo_input(model: &Model, page: &IssuesAndFiltersPage) -> Node<Msg> {
    let input = StyledSelect {
        id: page.current_jql_part.field_id(),
        text_filter: page.current_jql_part.text_filter.as_str(),
        variant: SelectVariant::Empty,
        dropdown_width: None,
        name: "jqlPart",
        valid: true,
        clearable: true,
        selected: page
            .current_jql_part
            .values
            .iter()
            .enumerate()
            .map(|(idx, n)| field_select_option(model, idx, *n, &page.jql_parts))
            .collect(),
        options: Some(options(model, &page.jql_parts).into_iter()),
        is_multi: false,
        opened: page.current_jql_part.opened,
    }
    .render();
    div![
        C!["pseudoInput"],
        StyledIcon::from(Icon::Search).render(),
        page.jql_parts
            .iter()
            .map(|s| s.to_label())
            .map(removable_part)
            .collect::<Vec<Node<Msg>>>(),
        input
    ]
}

fn options<'l, 'm: 'l>(model: &'m Model, parts: &[JqlPart]) -> Vec<StyledSelectOption<'l>> {
    log::info!("{} {:?}", parts.len(), parts);
    if parts.is_empty() {
        vec![field_select_option(model, 0, 1, &parts)]
    } else if parts.len() % 3 == 2
        && matches!(
            parts.get(parts.len() - 2),
            Some(JqlPart::Field(FieldOption::Assignee))
        )
    {
        model
            .users
            .iter()
            .map(|u| StyledSelectOption {
                name: Some("user"),
                icon: None,
                text: Some(u.name.as_str()),
                value: u.id as u32,
                class_list: "",
                variant: SelectVariant::Empty,
            })
            .collect()
    } else {
        vec![field_select_option(model, 0, 1, &parts)]
    }
}

fn field_select_option<'l, 'm: 'l>(
    model: &'m Model,
    idx: usize,
    option_value: u32,
    parts: &[JqlPart],
) -> StyledSelectOption<'l> {
    if parts.is_empty() {
        return build_field_select_option(option_value);
    }
    match (parts.len(), parts.len() % 3) {
        (_, 1) => {
            return build_field_select_option(option_value);
        }
        (_, 2)
            if matches!(
                parts.get(idx - 1),
                Some(JqlPart::Field(FieldOption::Assignee))
            ) =>
        {
            let user = model
                .users_by_id
                .get(&(option_value as i32))
                .map(|u| (u.id, u.name.as_str()))
                .unwrap_or_default();
            StyledSelectOption {
                name: Some("user"),
                icon: None,
                text: Some(user.1),
                value: user.0 as u32,
                class_list: "",
                variant: SelectVariant::Empty,
            }
        }
        _ => build_field_select_option(option_value),
    }
}

fn build_field_select_option<'l>(option_value: u32) -> StyledSelectOption<'l> {
    let v = FieldOption::from(option_value);
    StyledSelectOption {
        name: Some(v.to_name()),
        icon: None,
        text: Some(v.to_label()),
        value: v.to_value(),
        class_list: "",
        variant: SelectVariant::Empty,
    }
}

fn removable_part(part: &str) -> Node<Msg> {
    let remove = StyledButton {
        variant: ButtonVariant::Empty,
        icon: Some(StyledIcon::from(Icon::Trash).render()),
        on_click: None,
        ..Default::default()
    }
    .render();
    div![C!["part"], span![part], remove]
}
