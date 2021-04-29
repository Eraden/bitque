pub use epic_field::*;
pub use update::*;
pub use view::*;

pub mod comments_delete;
#[cfg(debug_assertions)]
pub mod debug;
pub mod epics_delete;
pub mod epics_edit;
pub mod issue_statuses_delete;
pub mod issues_create;
pub mod issues_delete;
pub mod issues_edit;
pub mod time_tracking;

mod epic_field;
mod update;
mod view;

#[macro_export]
macro_rules! match_modal {
    ($model: ident, $ty: ident) => {
        match $model.modals.iter().find_map(|modal| {
            if let crate::model::ModalType::$ty(modal) = modal {
                Some(modal)
            } else {
                None
            }
        }) {
            Some(modal) => modal,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! match_modal_mut {
    ($model: ident, $field: ident) => {
        match $model.modals.$field {
            Some(modal) => modal,
            _ => return,
        }
    };
}
