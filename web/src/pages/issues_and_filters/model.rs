use jirs_data::{Issue, IssueId, UserId, UsernameString};
use seed::app::Orders;

use crate::components::styled_select::StyledSelectState;
use crate::{model, FieldId, IssuesAndFiltersId, Msg};

#[derive(Debug)]
pub enum IssuesAndFiltersMsg {
    RemoveFilter(usize),
}

#[derive(Debug)]
pub struct IssuesAndFiltersPage {
    pub visible_issues: Vec<IssueId>,
    pub active_id: Option<IssueId>,
    pub current_jql_part: StyledSelectState,
    pub jql: Jql,
}

impl IssuesAndFiltersPage {
    pub fn new(model: &model::Model) -> Self {
        let jql = Jql::default();
        let visible_issues = Self::visible_issues(
            model
                .issue_ids
                .iter()
                .filter_map(|id| model.issues_by_id.get(id)),
            &jql,
        );
        let active_id = model.issue_ids.first().copied();

        Self {
            visible_issues,
            active_id,
            current_jql_part: StyledSelectState::new(
                FieldId::IssuesAndFilters(IssuesAndFiltersId::Jql),
                vec![],
            ),
            jql,
        }
    }

    pub fn visible_issues<'l, IssueStream>(issues: IssueStream, jql: &Jql) -> Vec<IssueId>
    where
        IssueStream: std::iter::Iterator<Item = &'l Issue>,
    {
        issues
            .filter(|issue| jql.is_visible(issue))
            .map(|issue| issue.id)
            .collect()
    }

    pub fn update(&mut self, msg: &Msg, orders: &mut impl Orders<Msg>) {
        self.current_jql_part.update(msg, orders);
    }
}

#[derive(Debug, Default)]
pub struct Jql {
    pub parts: Vec<JqlPart>,
}

impl Jql {
    pub fn current_token(&self) -> Option<JqlPartType> {
        if self.parts.is_empty() {
            return None;
        }
        // [field, op, field, keyword]
        match self.len() % 4 {
            0 => Some(JqlPartType::Keyword),
            3 => Some(JqlPartType::Value),
            2 => Some(JqlPartType::Op),
            1 => Some(JqlPartType::Field),
            _ => None,
        }
    }

    pub fn op(&self) -> Option<&JqlPart> {
        if self.parts.is_empty() {
            return None;
        }
        match self.len() % 4 {
            // [field, op, value, keyword]
            0 => self.parts.get(self.len() - 3),
            // [field, op, value]
            3 => self.parts.get(self.len() - 2),
            // [field, op]
            2 => self.parts.last(),
            // [field]
            _ => None,
        }
    }

    pub fn value(&self) -> Option<&JqlPart> {
        if self.parts.is_empty() {
            return None;
        }

        match self.len() % 4 {
            // [field, op, value, keyword]
            0 => self.parts.get(self.len() - 2),
            // [field, op, value]
            3 => self.parts.last(),
            _ => None,
        }
    }

    pub fn field(&self) -> Option<&JqlPart> {
        if self.parts.is_empty() {
            return None;
        }
        match self.len() % 4 {
            // [field, op, value, keyword]
            0 => self.parts.get(self.len() - 3),
            // [field]
            1 => self.parts.last(),
            // [field, op]
            2 => self.parts.get(self.len() - 2),
            _ => None,
        }
    }

    pub fn keyword(&self) -> Option<&JqlPart> {
        if self.parts.is_empty() {
            return None;
        }
        match self.len() % 4 {
            // [field]
            0 => self.parts.last(),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        self.parts.len()
    }

    pub fn push(&mut self, part: JqlPart) {
        self.parts.push(part);
    }

    pub fn remove_from(&mut self, idx: usize) {
        if self.parts.is_empty() {
            return;
        }
        if self.len() <= 4 {
            self.parts.clear();
        } else {
            self.parts.drain((idx - 1 - (idx % 4))..);
        }
    }

    pub fn is_visible(&self, issue: &jirs_data::Issue) -> bool {
        if self.len() < 3 {
            return true;
        }

        let mut q = (&self.parts).iter();
        while let Some(field) = q.next() {
            let op = match q.next() {
                None => break,
                Some(op) => op,
            };
            let value = match q.next() {
                None => break,
                Some(value) => value,
            };
            let _keyword = q.next(); // skip keyword
            match (field, op, value) {
                //
                (
                    JqlPart::Field(FieldOption::Assignee),
                    JqlPart::Op(OpOption::Is | OpOption::Eq),
                    JqlPart::Value(JqlValueOption::User(id, _)),
                ) if !issue.user_ids.contains(id) => return false,
                (
                    JqlPart::Field(FieldOption::Assignee),
                    JqlPart::Op(OpOption::IsNot | OpOption::NotEq),
                    JqlPart::Value(JqlValueOption::User(id, _)),
                ) if issue.user_ids.contains(id) => return false,
                //
                (
                    JqlPart::Field(FieldOption::Type),
                    JqlPart::Op(OpOption::Is | OpOption::Eq),
                    JqlPart::Value(JqlValueOption::Type(t)),
                ) if issue.issue_type != *t => {
                    return false;
                }
                (
                    JqlPart::Field(FieldOption::Type),
                    JqlPart::Op(OpOption::IsNot | OpOption::NotEq),
                    JqlPart::Value(JqlValueOption::Type(t)),
                ) if issue.issue_type == *t => {
                    return false;
                }
                //
                (
                    JqlPart::Field(FieldOption::Priority),
                    JqlPart::Op(OpOption::Is | OpOption::Eq),
                    JqlPart::Value(JqlValueOption::Priority(p)),
                ) if issue.priority != *p => {
                    return false;
                }
                (
                    JqlPart::Field(FieldOption::Priority),
                    JqlPart::Op(OpOption::IsNot | OpOption::NotEq),
                    JqlPart::Value(JqlValueOption::Priority(p)),
                ) if issue.priority == *p => {
                    return false;
                }

                _ => {}
            };
        }
        true
    }
}

#[derive(Debug)]
pub enum JqlPartType {
    Field,
    Op,
    Value,
    Keyword,
}

#[derive(Debug)]
pub enum JqlPart {
    Field(FieldOption),
    Op(OpOption),
    Value(JqlValueOption),
    Keyword(KeywordOption),
}

impl JqlPart {
    pub fn to_label(&self) -> &str {
        match self {
            JqlPart::Field(f) => f.to_label(),
            JqlPart::Op(op) => op.to_label(),
            JqlPart::Value(v) => v.to_label(),
            JqlPart::Keyword(k) => k.to_label(),
        }
    }
}

#[derive(Debug)]
pub enum FieldOption {
    None,
    Assignee,
    Type,
    Priority,
}

impl FieldOption {
    pub fn to_label(&self) -> &'static str {
        match self {
            FieldOption::None => " ",
            FieldOption::Assignee => "Assignee",
            FieldOption::Type => "Type",
            FieldOption::Priority => "Priority",
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            FieldOption::None => "none",
            FieldOption::Assignee => "assignee",
            FieldOption::Type => "ticketType",
            FieldOption::Priority => "ticketPriority",
        }
    }

    pub fn to_value(&self) -> u32 {
        match self {
            FieldOption::None => 0,
            FieldOption::Assignee => 1,
            FieldOption::Type => 2,
            FieldOption::Priority => 3,
        }
    }
}

impl From<u32> for FieldOption {
    fn from(n: u32) -> Self {
        match n {
            1 => FieldOption::Assignee,
            2 => FieldOption::Type,
            3 => FieldOption::Priority,
            _ => FieldOption::None,
        }
    }
}

#[derive(Debug)]
pub enum OpOption {
    None,
    Eq,
    NotEq,
    Is,
    IsNot,
}

impl OpOption {
    pub fn to_label(&self) -> &'static str {
        match self {
            OpOption::None => " ",
            OpOption::Eq => "=",
            OpOption::NotEq => "!=",
            OpOption::Is => "IS",
            OpOption::IsNot => "IS NOT",
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            OpOption::None => "none",
            OpOption::Eq => "equal",
            OpOption::NotEq => "notEqual",
            OpOption::Is => "is",
            OpOption::IsNot => "isNot",
        }
    }

    pub fn to_value(&self) -> u32 {
        match self {
            OpOption::None => 0,
            OpOption::Eq => 1,
            OpOption::NotEq => 2,
            OpOption::Is => 3,
            OpOption::IsNot => 4,
        }
    }
}

impl From<u32> for OpOption {
    fn from(n: u32) -> Self {
        match n {
            1 => OpOption::Eq,
            2 => OpOption::NotEq,
            3 => OpOption::Is,
            4 => OpOption::IsNot,
            _ => OpOption::None,
        }
    }
}

#[derive(Debug)]
pub enum JqlValueOption {
    User(UserId, UsernameString),
    Priority(jirs_data::IssuePriority),
    Type(jirs_data::IssueType),
}

impl JqlValueOption {
    pub fn to_label(&self) -> &str {
        match self {
            JqlValueOption::User(_id, name) => name.as_str(),
            JqlValueOption::Priority(p) => p.to_label(),
            JqlValueOption::Type(t) => t.to_label(),
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            JqlValueOption::User(_, _) => "user",
            JqlValueOption::Priority(_) => "priority",
            JqlValueOption::Type(_) => "type",
        }
    }

    pub fn to_value(&self) -> u32 {
        match self {
            JqlValueOption::User(id, _) => (*id) as u32,
            JqlValueOption::Priority(p) => (*p).into(),
            JqlValueOption::Type(t) => (*t).into(),
        }
    }
}

#[derive(Debug)]
pub enum KeywordOption {
    None,
    And,
}

impl From<u32> for KeywordOption {
    fn from(n: u32) -> Self {
        match n {
            1 => KeywordOption::And,
            _ => KeywordOption::None,
        }
    }
}

impl KeywordOption {
    pub fn to_label(&self) -> &'static str {
        match self {
            KeywordOption::None => " ",
            KeywordOption::And => "AND",
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            KeywordOption::None => "none",
            KeywordOption::And => "and",
        }
    }

    pub fn to_value(&self) -> u32 {
        match self {
            KeywordOption::None => 0,
            KeywordOption::And => 1,
        }
    }
}
