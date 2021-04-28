use seed::prelude::*;

use crate::components::styled_select::StyledSelectChanged;
use crate::model::{Model, PageContent};
use crate::pages::issues_and_filters::{FieldOption, JqlPart, JqlValueOption, OpOption};
use crate::ws::board_load;
use crate::{FieldId, IssuesAndFiltersId, Msg, OperationKind, Page, ResourceKind};

mod jql;

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
        Msg::ResourceChanged(ResourceKind::Issue, OperationKind::ListLoaded, _) => {
            let issues = super::IssuesAndFiltersPage::visible_issues(model.issues());
            let first_id = model.issues().first().as_ref().map(|issue| issue.id);
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
            let first_id = model.issues().first().as_ref().map(|issue| issue.id);
            crate::match_page_mut!(model, IssuesAndFilters).active_id = first_id;
        }
        Msg::StyledSelectChanged(
            FieldId::IssuesAndFilters(IssuesAndFiltersId::Jql),
            StyledSelectChanged::Changed(Some(n)),
        ) if n != 0 => {
            let page = crate::match_page_mut!(model, IssuesAndFilters);
            match page.jql_parts.len() % 3 {
                0 => {
                    let field = FieldOption::from(n);
                    page.jql_parts.push(JqlPart::Field(field));
                }
                1 => {
                    let field = OpOption::from(n);
                    page.jql_parts.push(JqlPart::Op(field));
                }
                2 => {
                    let u = match model.users.get(n as usize) {
                        Some(u) => u,
                        _ => return,
                    };
                    let field = JqlValueOption::User(u.id, u.name.clone());
                    page.jql_parts.push(JqlPart::Value(field));
                }
                _ => {}
            }
            page.current_jql_part.reset();
        }
        _ => {}
    }
}

fn build_page_content(model: &mut Model) {
    model.page_content =
        PageContent::IssuesAndFilters(Box::new(super::IssuesAndFiltersPage::new(model)));
}
