use actix::{Handler, Message};
use diesel::prelude::*;

use jirs_data::IssueAssignee;

use crate::{
    db::{DbExecutor, DbPooledConn},
    db_pool,
    errors::ServiceErrors,
    q,
};

pub struct LoadAssignees {
    pub issue_id: i32,
}

impl LoadAssignees {
    pub fn execute(self, conn: &DbPooledConn) -> Result<Vec<IssueAssignee>, ServiceErrors> {
        use crate::schema::issue_assignees::dsl::*;

        q!(issue_assignees
            .distinct_on(id)
            .filter(issue_id.eq(self.issue_id)))
        .load::<IssueAssignee>(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::RecordNotFound("issue users".to_string())
        })
    }
}

impl Message for LoadAssignees {
    type Result = Result<Vec<IssueAssignee>, ServiceErrors>;
}

impl Handler<LoadAssignees> for DbExecutor {
    type Result = Result<Vec<IssueAssignee>, ServiceErrors>;

    fn handle(&mut self, msg: LoadAssignees, _ctx: &mut Self::Context) -> Self::Result {
        let conn = db_pool!(self);

        msg.execute(conn)
    }
}
