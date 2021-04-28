use seed::prelude::*;
use seed::*;

use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_select::{SelectVariant, StyledSelect};
use crate::components::styled_select_child::StyledSelectOption;
use crate::model::Model;
use crate::pages::issues_and_filters::{FieldOption, IssuesAndFiltersPage};
use crate::Msg;

pub fn filters(_model: &Model, page: &IssuesAndFiltersPage) -> Node<Msg> {
    let search_input = pseudo_input(page);
    div![C!["filters"], search_input]
}

fn pseudo_input(page: &IssuesAndFiltersPage) -> Node<Msg> {
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
            .map(|n| field_select_option(*n))
            .collect(),
        options: Some(vec![field_select_option(1)].into_iter()),
        is_multi: false,
        opened: page.current_jql_part.opened,
    }
    .render();
    div![
        C!["pseudoInput"],
        StyledIcon::from(Icon::Search).render(),
        input
    ]
}

fn field_select_option<'l>(n: u32) -> StyledSelectOption<'l> {
    let v = FieldOption::from(n);
    StyledSelectOption {
        name: Some(v.to_name()),
        icon: None,
        text: Some(v.to_label()),
        value: v.to_value(),
        class_list: "",
        variant: SelectVariant::Empty,
    }
}
