use seed::prelude::*;

use super::IssuesAndFiltersMsg;
use crate::components::styled_select::StyledSelectChanged;
use crate::model::{Model, PageContent};
use crate::pages::issues_and_filters::{
    FieldOption, JqlPart, JqlPartType, JqlValueOption, KeywordOption, OpOption,
};
use crate::ws::board_load;
use crate::{match_page, FieldId, IssuesAndFiltersId, Msg, OperationKind, Page, ResourceKind};

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    if model.user.is_none() {
        return;
    }

    if matches!(model.page, Page::IssuesAndFilters)
        && !matches!(model.page_content, PageContent::IssuesAndFilters(..))
    {
        build_page_content(model);
    }

    match msg {
        Msg::ResourceChanged(ResourceKind::Auth, OperationKind::SingleLoaded, Some(_))
        | Msg::ChangePage(Page::IssuesAndFilters) => {
            board_load(model, orders);
            build_page_content(model);
        }
        _ => (),
    }

    crate::match_page_mut!(model, IssuesAndFilters).update(&msg, orders);

    match msg {
        Msg::IssuesAndFilters(IssuesAndFiltersMsg::RemoveFilter(idx)) => {
            crate::match_page_mut!(model, IssuesAndFilters)
                .jql
                .remove_from(idx);
            let issues = super::IssuesAndFiltersPage::visible_issues(
                model
                    .issue_ids
                    .iter()
                    .filter_map(|id| model.issues_by_id.get(id)),
                &crate::match_page!(model, IssuesAndFilters).jql,
            );
            crate::match_page_mut!(model, IssuesAndFilters).visible_issues = issues;
        }
        Msg::ResourceChanged(ResourceKind::Issue, OperationKind::ListLoaded, _) => {
            let jql = &match_page!(model, IssuesAndFilters).jql;
            let issues = super::IssuesAndFiltersPage::visible_issues(
                model
                    .issue_ids
                    .iter()
                    .filter_map(|id| model.issues_by_id.get(id)),
                jql,
            );
            let first_id = model.issue_ids.first().copied();
            let page = crate::match_page_mut!(model, IssuesAndFilters);
            if page.active_id.is_none() {
                page.active_id = first_id;
            }
            page.visible_issues = issues;
        }
        Msg::SetActiveIssue(Some(id)) => {
            crate::match_page_mut!(model, IssuesAndFilters).active_id = Some(id);
        }
        Msg::SetActiveIssue(None) => {
            let first_id = model.issue_ids.first().copied();
            crate::match_page_mut!(model, IssuesAndFilters).active_id = first_id;
        }
        Msg::StyledSelectChanged(
            FieldId::IssuesAndFilters(IssuesAndFiltersId::Jql),
            StyledSelectChanged::Changed(Some(n)),
        ) if n != 0 => {
            let page = crate::match_page_mut!(model, IssuesAndFilters);
            match page.jql.current_token() {
                None | Some(JqlPartType::Keyword) => {
                    let field = FieldOption::from(n);
                    page.jql.push(JqlPart::Field(field));
                }
                Some(JqlPartType::Field) => {
                    let field = OpOption::from(n);
                    page.jql.push(JqlPart::Op(field));
                }
                Some(JqlPartType::Op)
                    if matches!(
                        page.jql.field(),
                        Some(JqlPart::Field(FieldOption::Assignee))
                    ) =>
                {
                    let id = match model.user_ids.get(n as usize) {
                        Some(idx) => idx,
                        _ => return,
                    };
                    let u = match model.users_by_id.get(id) {
                        Some(u) => u,
                        _ => return,
                    };
                    let field = JqlValueOption::User(u.id, u.name.clone());
                    page.jql.push(JqlPart::Value(field));
                }
                Some(JqlPartType::Op)
                    if matches!(page.jql.field(), Some(JqlPart::Field(FieldOption::Type))) =>
                {
                    page.jql
                        .push(JqlPart::Value(JqlValueOption::Type(n.into())));
                }
                Some(JqlPartType::Op)
                    if matches!(
                        page.jql.field(),
                        Some(JqlPart::Field(FieldOption::Priority))
                    ) =>
                {
                    page.jql
                        .push(JqlPart::Value(JqlValueOption::Priority(n.into())));
                }
                Some(JqlPartType::Value) => {
                    let field = KeywordOption::from(n);
                    page.jql.push(JqlPart::Keyword(field));
                }
                _ => {}
            }
            page.current_jql_part.reset();
            page.current_jql_part.opened = true;
            let issues = super::IssuesAndFiltersPage::visible_issues(
                model
                    .issue_ids
                    .iter()
                    .filter_map(|id| model.issues_by_id.get(id)),
                &crate::match_page!(model, IssuesAndFilters).jql,
            );
            crate::match_page_mut!(model, IssuesAndFilters).visible_issues = issues;
        }
        _ => {}
    }
}

fn build_page_content(model: &mut Model) {
    model.page_content =
        PageContent::IssuesAndFilters(Box::new(super::IssuesAndFiltersPage::new(model)));
}
