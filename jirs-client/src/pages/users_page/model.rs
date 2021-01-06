use jirs_data::{Invitation, User, UserRole, UsersFieldId};

use crate::model::InvitationFormState;
use crate::shared::styled_select::StyledSelectState;
use crate::FieldId;

#[derive(Debug)]
pub struct UsersPage {
    pub name: String,
    pub name_touched: bool,
    pub email: String,
    pub email_touched: bool,
    pub user_role: UserRole,

    pub user_role_state: StyledSelectState,
    pub pending_invitations: Vec<String>,
    pub error: String,
    pub form_state: InvitationFormState,

    pub invited_users: Vec<User>,
    pub invitations: Vec<Invitation>,
}

impl Default for UsersPage {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            name_touched: false,
            email: "".to_string(),
            email_touched: false,
            user_role: Default::default(),
            user_role_state: StyledSelectState::new(FieldId::Users(UsersFieldId::UserRole), vec![]),
            pending_invitations: vec![],
            error: "".to_string(),
            form_state: Default::default(),
            invited_users: vec![],
            invitations: vec![],
        }
    }
}
