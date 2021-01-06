use jirs_data::{ProjectId, User, UsersFieldId};

use crate::{
    shared::{
        styled_image_input::StyledImageInputState, styled_input::StyledInputState,
        styled_select::StyledSelectState,
    },
    FieldId,
};

#[derive(Debug)]
pub struct ProfilePage {
    pub name: StyledInputState,
    pub email: StyledInputState,
    pub avatar: StyledImageInputState,
    pub current_project: StyledSelectState,
}

impl ProfilePage {
    pub fn new(user: &User, project_ids: Vec<ProjectId>) -> Self {
        Self {
            name: StyledInputState::new(
                FieldId::Profile(UsersFieldId::Username),
                user.name.as_str(),
            ),
            email: StyledInputState::new(
                FieldId::Profile(UsersFieldId::Email),
                user.email.as_str(),
            ),
            avatar: StyledImageInputState::new(
                FieldId::Profile(UsersFieldId::Avatar),
                user.avatar_url.as_ref().cloned(),
            ),
            current_project: StyledSelectState::new(
                FieldId::Profile(UsersFieldId::CurrentProject),
                project_ids.into_iter().map(|n| n as u32).collect(),
            ),
        }
    }
}
