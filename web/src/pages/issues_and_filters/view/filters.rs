use seed::prelude::*;
use seed::*;

use super::super::IssuesAndFiltersMsg;
use crate::components::styled_button::ButtonVariant;
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_select::{SelectVariant, StyledSelect};
use crate::components::styled_select_child::StyledSelectOption;
use crate::model::Model;
use crate::pages::issues_and_filters::{
    FieldOption, IssuesAndFiltersPage, Jql, JqlPart, JqlPartType, KeywordOption, OpOption,
};
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
            .map(|n| field_select_option(model, *n, &page.jql))
            .collect(),
        options: Some(options(model, &page.jql).into_iter()),
        is_multi: false,
        opened: page.current_jql_part.opened,
    }
    .render();
    div![
        C!["pseudoInput"],
        StyledIcon::from(Icon::Search).render(),
        page.jql
            .parts
            .iter()
            .map(|s| s.to_label())
            .enumerate()
            .map(|(idx, m)| removable_part(idx, m))
            .collect::<Vec<Node<Msg>>>(),
        input
    ]
}

fn options<'l, 'm: 'l>(model: &'m Model, jql: &Jql) -> Vec<StyledSelectOption<'l>> {
    let ty = jql.current_token();
    match ty {
        Some(JqlPartType::Value) => {
            vec![build_keyword_select_option(1)]
        }
        Some(JqlPartType::Field) => {
            vec![
                field_select_option(model, 1, jql),
                field_select_option(model, 2, jql),
                field_select_option(model, 3, jql),
                field_select_option(model, 4, jql),
            ]
        }
        Some(JqlPartType::Op)
            if matches!(jql.field(), Some(JqlPart::Field(FieldOption::Assignee))) =>
        {
            model
                .user_ids
                .iter()
                .filter_map(|id| model.users_by_id.get(id))
                .map(|u| StyledSelectOption {
                    name: Some("user"),
                    icon: None,
                    text: Some(u.name.as_str()),
                    value: u.id as u32,
                    class_list: "",
                    variant: SelectVariant::Empty,
                })
                .collect()
        }
        Some(JqlPartType::Op)
            if matches!(jql.field(), Some(JqlPart::Field(FieldOption::Priority))) =>
        {
            vec![
                jirs_data::IssuePriority::Lowest,
                jirs_data::IssuePriority::Low,
                jirs_data::IssuePriority::Medium,
                jirs_data::IssuePriority::High,
                jirs_data::IssuePriority::Highest,
            ]
            .into_iter()
            .map(|u| StyledSelectOption {
                name: Some("priority"),
                icon: None,
                text: Some(u.to_label()),
                value: u.into(),
                class_list: "",
                variant: SelectVariant::Empty,
            })
            .collect()
        }
        Some(JqlPartType::Op) if matches!(jql.field(), Some(JqlPart::Field(FieldOption::Type))) => {
            vec![
                jirs_data::IssueType::Task,
                jirs_data::IssueType::Bug,
                jirs_data::IssueType::Story,
            ]
            .into_iter()
            .map(|u| StyledSelectOption {
                name: Some("type"),
                icon: None,
                text: Some(u.to_label()),
                value: u.into(),
                class_list: "",
                variant: SelectVariant::Empty,
            })
            .collect()
        }
        Some(JqlPartType::Op) => {
            vec![field_select_option(model, 1, jql)]
        }
        None | Some(JqlPartType::Keyword) => vec![
            field_select_option(model, 1, jql),
            field_select_option(model, 2, jql),
            field_select_option(model, 3, jql),
        ],
    }
}

fn field_select_option<'l, 'm: 'l>(
    model: &'m Model,
    option_value: u32,
    jql: &Jql,
) -> StyledSelectOption<'l> {
    let ty = jql.current_token();
    if ty.is_none() {
        return build_field_select_option(option_value);
    }
    match jql.current_token() {
        Some(JqlPartType::Op)
            if matches!(jql.field(), Some(JqlPart::Field(FieldOption::Assignee))) =>
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
        Some(JqlPartType::Op)
            if matches!(jql.field(), Some(JqlPart::Field(FieldOption::Priority))) =>
        {
            let p: jirs_data::IssuePriority = option_value.into();
            StyledSelectOption {
                name: Some("priority"),
                icon: None,
                text: Some(p.to_label()),
                value: p.into(),
                class_list: "",
                variant: SelectVariant::Empty,
            }
        }
        Some(JqlPartType::Op) if matches!(jql.field(), Some(JqlPart::Field(FieldOption::Type))) => {
            let p: jirs_data::IssueType = option_value.into();
            StyledSelectOption {
                name: Some("type"),
                icon: None,
                text: Some(p.to_label()),
                value: p.into(),
                class_list: "",
                variant: SelectVariant::Empty,
            }
        }
        Some(JqlPartType::Field) => build_op_select_option(option_value),
        _ => build_field_select_option(option_value),
    }
}

fn build_op_select_option<'l>(option_value: u32) -> StyledSelectOption<'l> {
    let v = OpOption::from(option_value);
    StyledSelectOption {
        name: Some(v.to_name()),
        icon: None,
        text: Some(v.to_label()),
        value: v.to_value(),
        class_list: "",
        variant: SelectVariant::Empty,
    }
}

fn build_keyword_select_option<'l>(option_value: u32) -> StyledSelectOption<'l> {
    let v = KeywordOption::from(option_value);
    StyledSelectOption {
        name: Some(v.to_name()),
        icon: None,
        text: Some(v.to_label()),
        value: v.to_value(),
        class_list: "",
        variant: SelectVariant::Empty,
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

fn removable_part(idx: usize, part: &str) -> Node<Msg> {
    let remove = StyledButton {
        variant: ButtonVariant::Empty,
        icon: Some(StyledIcon::from(Icon::Trash).render()),
        on_click: Some(mouse_ev("click", move |_| {
            Msg::IssuesAndFilters(IssuesAndFiltersMsg::RemoveFilter(idx))
        })),
        ..Default::default()
    }
    .render();
    div![C!["part"], span![part], remove]
}
