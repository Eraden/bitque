use {
    crate::{
        model::IssueModal,
        shared::{
            styled_date_time_input::StyledDateTimeInputState,
            styled_input::StyledInputState,
            styled_select::StyledSelectState,
            styled_select_child::{StyledSelectChild, StyledSelectChildBuilder},
            ToChild, ToNode,
        },
        FieldId, Msg,
    },
    jirs_data::{IssueFieldId, IssuePriority},
    seed::prelude::*,
};

#[derive(Copy, Clone)]
pub enum Type {
    Task,
    Bug,
    Story,
    Epic,
}

impl From<u32> for Type {
    fn from(n: u32) -> Self {
        match n {
            0 => Type::Task,
            1 => Type::Bug,
            2 => Type::Story,
            3 => Type::Epic,
            _ => Type::Task,
        }
    }
}

impl Type {
    pub(crate) fn ordered<'l>() -> &'l [Type] {
        use Type::*;
        &[Task, Bug, Story, Epic]
    }

    pub(crate) fn submit_label(&self) -> &str {
        use Type::*;
        match self {
            Epic => "Create epic",
            Bug | Task | Story => "Create issue",
        }
    }

    pub(crate) fn form_label(&self) -> &str {
        use Type::*;
        match self {
            Epic => "Create epic",
            Bug | Task | Story => "Create issue",
        }
    }

    pub(crate) fn submit_action(&self) -> Msg {
        use Type::*;
        match self {
            Epic => Msg::AddEpic,
            Bug | Task | Story => Msg::AddIssue,
        }
    }
}

impl<'l> ToChild<'l> for Type {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        let name = match self {
            Type::Task => "Task",
            Type::Bug => "Bug",
            Type::Story => "Story",
            Type::Epic => "Epic",
        };
        let value = match self {
            Type::Task => 0,
            Type::Bug => 1,
            Type::Story => 2,
            Type::Epic => 3,
        };
        let icon = match self {
            Type::Task => crate::shared::styled_icon::Icon::Task,
            Type::Bug => crate::shared::styled_icon::Icon::Bug,
            Type::Story => crate::shared::styled_icon::Icon::Story,
            Type::Epic => crate::shared::styled_icon::Icon::Epic,
        };

        let type_icon = crate::shared::styled_icon::StyledIcon::build(icon)
            .add_class(name)
            .build()
            .into_node();

        StyledSelectChild::build()
            .add_class(name)
            .text(name)
            .icon(type_icon)
            .value(value)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Model {
    pub priority: IssuePriority,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub project_id: Option<jirs_data::ProjectId>,
    pub user_ids: Vec<jirs_data::UserId>,
    pub reporter_id: Option<jirs_data::UserId>,
    pub issue_status_id: jirs_data::IssueStatusId,
    pub epic_id: Option<jirs_data::UserId>,

    // modal fields
    pub title_state: StyledInputState,
    pub type_state: StyledSelectState,
    pub reporter_state: StyledSelectState,
    pub assignees_state: StyledSelectState,
    pub priority_state: StyledSelectState,

    // epic
    pub epic_name_state: StyledSelectState,
    pub epic_starts_at_state: StyledDateTimeInputState,
    pub epic_ends_at_state: StyledDateTimeInputState,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            priority: Default::default(),
            description: Default::default(),
            description_text: Default::default(),
            estimate: Default::default(),
            time_spent: Default::default(),
            time_remaining: Default::default(),
            project_id: Default::default(),
            user_ids: Default::default(),
            reporter_id: Default::default(),
            issue_status_id: Default::default(),
            epic_id: Default::default(),
            title_state: StyledInputState::new(FieldId::AddIssueModal(IssueFieldId::Title), ""),
            type_state: StyledSelectState::new(FieldId::AddIssueModal(IssueFieldId::Type), vec![]),
            reporter_state: StyledSelectState::new(
                FieldId::AddIssueModal(IssueFieldId::Reporter),
                vec![],
            ),
            assignees_state: StyledSelectState::new(
                FieldId::AddIssueModal(IssueFieldId::Assignees),
                vec![],
            ),
            priority_state: StyledSelectState::new(
                FieldId::AddIssueModal(IssueFieldId::Priority),
                vec![],
            ),
            // epic
            epic_name_state: StyledSelectState::new(
                FieldId::AddIssueModal(IssueFieldId::EpicName),
                vec![],
            ),
            epic_starts_at_state: StyledDateTimeInputState::new(
                FieldId::AddIssueModal(IssueFieldId::EpicStartsAt),
                None,
            ),
            epic_ends_at_state: StyledDateTimeInputState::new(
                FieldId::AddIssueModal(IssueFieldId::EpicEndsAt),
                None,
            ),
        }
    }
}

impl IssueModal for Model {
    fn epic_id_value(&self) -> Option<u32> {
        self.epic_name_state.values.get(0).cloned()
    }

    fn epic_state(&self) -> &StyledSelectState {
        &self.epic_name_state
    }

    fn update_states(&mut self, msg: &Msg, orders: &mut impl Orders<Msg>) {
        self.title_state.update(msg);
        self.assignees_state.update(msg, orders);
        self.reporter_state.update(msg, orders);
        self.type_state.update(msg, orders);
        self.priority_state.update(msg, orders);
        self.epic_name_state.update(msg, orders);
        self.epic_starts_at_state.update(msg, orders);
        self.epic_ends_at_state.update(msg, orders);
    }
}
