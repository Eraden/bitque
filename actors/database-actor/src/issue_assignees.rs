use {
  crate::{db_create, db_delete, db_load, db_load_field},
  diesel::{expression::dsl::not, prelude::*},
  jirs_data::{IssueAssignee, IssueId, UserId},
};

db_create! {
    AsignMultiple,
    msg => issue_assignees => {
        use crate::models::CreateIssueAssigneeForm;
        let AsignMultiple { user_ids, issue_id: i_id } = msg;

        diesel::insert_into(issue_assignees)
            .values(user_ids.into_iter().map(|u_id| {
                CreateIssueAssigneeForm {
                    user_id: u_id,
                    issue_id: i_id
                 }
            }).collect::<Vec<CreateIssueAssigneeForm>>())
    },
    IssueAssignee,
    user_ids => Vec<UserId>,
    issue_id => IssueId
}

db_load! {
    LoadAssignees,
    msg => issue_assignees => issue_assignees
            .distinct_on(id)
            .filter(issue_id.eq(msg.issue_id)),
    IssueAssignee,
    issue_id => IssueId
}

db_load_field! {
    LoadAssigneesIds,
    UserId,
    msg => issue_assignees => issue_assignees
                .select(user_id)
                .filter(issue_id.eq(msg.issue_id)),
    IssueAssignee,
    issue_id => IssueId
}

db_delete! {
    DeleteIssueAssignees,
    msg => issue_assignees => diesel::delete(issue_assignees.filter(issue_id.eq(msg.issue_id))),
    IssueAssignee,
    issue_id => IssueId
}

db_delete! {
    DropIssueAssignees,
    msg => issue_assignees => diesel::delete(issue_assignees)
                .filter(not(user_id.eq_any(msg.user_ids)).and(issue_id.eq(msg.issue_id))),
    IssueAssignee,
    issue_id => IssueId,
    user_ids => Vec<UserId>
}
