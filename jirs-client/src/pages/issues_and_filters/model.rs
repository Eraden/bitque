use jirs_data::{Issue, IssueId, UserId, UsernameString};
use seed::app::Orders;

use crate::components::styled_select::StyledSelectState;
use crate::{model, FieldId, IssuesAndFiltersId, Msg};

#[derive(Debug)]
pub struct IssuesAndFiltersPage {
    pub visible_issues: Vec<IssueId>,
    pub active_id: Option<IssueId>,
    pub current_jql_part: StyledSelectState,
    pub jql_parts: Vec<JqlPart>,
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

#[derive(Debug)]
pub enum JqlPart {
    Field(FieldOption),
    Op(OpOption),
    Value(JqlValueOption),
}

impl JqlPart {
    pub fn to_label(&self) -> &str {
        match self {
            JqlPart::Field(f) => f.to_label(),
            JqlPart::Op(op) => op.to_label(),
            JqlPart::Value(v) => v.to_label(),
        }
    }
}

#[derive(Debug)]
pub enum FieldOption {
    None,
    Assignee,
}

impl FieldOption {
    pub fn to_label(&self) -> &'static str {
        match self {
            FieldOption::None => " ",
            FieldOption::Assignee => "Assignee",
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            FieldOption::None => "none",
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

#[derive(Debug)]
pub enum OpOption {
    None,
    Eq,
    NotEq,
}

impl OpOption {
    pub fn to_label(&self) -> &'static str {
        match self {
            OpOption::None => " ",
            OpOption::Eq => "=",
            OpOption::NotEq => "!=",
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            OpOption::None => "none",
            OpOption::Eq => "equal",
            OpOption::NotEq => "notEqual",
        }
    }

    pub fn to_value(&self) -> u32 {
        match self {
            OpOption::None => 0,
            OpOption::Eq => 1,
            OpOption::NotEq => 2,
        }
    }
}

impl From<u32> for OpOption {
    fn from(n: u32) -> Self {
        match n {
            1 => OpOption::Eq,
            2 => OpOption::NotEq,
            _ => OpOption::None,
        }
    }
}

#[derive(Debug)]
pub enum JqlValueOption {
    User(UserId, UsernameString),
}

impl JqlValueOption {
    pub fn to_label(&self) -> &str {
        match self {
            JqlValueOption::User(_id, name) => name.as_str(),
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            JqlValueOption::User(_, _) => "user",
        }
    }

    pub fn to_value(&self) -> u32 {
        match self {
            JqlValueOption::User(id, _) => (*id) as u32,
        }
    }
}
