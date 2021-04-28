use jirs_data::{Issue, IssueId};
use seed::app::Orders;

use crate::components::styled_select::StyledSelectState;
use crate::{model, FieldId, IssuesAndFiltersId, Msg};

#[derive(Debug)]
pub struct IssuesAndFiltersPage {
    pub visible_issues: Vec<IssueId>,
    pub active_id: Option<IssueId>,
    pub current_jql_part: StyledSelectState,
    pub jql_parts: Vec<String>,
}

impl IssuesAndFiltersPage {
    pub fn new(model: &model::Model) -> Self {
        let visible_issues = Self::visible_issues(model.issues());
        let active_id = model.issues().first().as_ref().map(|issue| issue.id);

        Self {
            visible_issues,
            active_id,
            current_jql_part: StyledSelectState::new(
                FieldId::IssuesAndFilters(IssuesAndFiltersId::Jql),
                vec![],
            ),
            jql_parts: vec![],
        }
    }

    pub fn visible_issues(issues: &[Issue]) -> Vec<IssueId> {
        issues.iter().map(|issue| issue.id).collect()
    }

    pub fn update(&mut self, msg: &Msg, orders: &mut impl Orders<Msg>) {
        self.current_jql_part.update(msg, orders);
    }
}

pub enum FieldOption {
    None,
    Assignee,
}

impl FieldOption {
    pub fn to_label(&self) -> &'static str {
        match self {
            FieldOption::None => "none",
            FieldOption::Assignee => "Assignee",
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            FieldOption::None => " ",
            FieldOption::Assignee => "assignee",
        }
    }

    pub fn to_value(&self) -> u32 {
        match self {
            FieldOption::None => 0,
            FieldOption::Assignee => 1,
        }
    }
}

impl From<u32> for FieldOption {
    fn from(n: u32) -> Self {
        match n {
            1 => FieldOption::Assignee,
            _ => FieldOption::None,
        }
    }
}
