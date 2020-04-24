#![allow(unused_imports, dead_code)]

table! {
    use diesel::sql_types::*;
    use jirs_data::sql::*;

    /// Representation of the `comments` table.
    ///
    /// (Automatically generated by Diesel.)
    comments (id) {
        /// The `id` column of the `comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `body` column of the `comments` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        body -> Text,
        /// The `user_id` column of the `comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `issue_id` column of the `comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        issue_id -> Int4,
        /// The `created_at` column of the `comments` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `comments` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use jirs_data::sql::*;

    /// Representation of the `invitations` table.
    ///
    /// (Automatically generated by Diesel.)
    invitations (id) {
        /// The `id` column of the `invitations` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `name` column of the `invitations` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
        /// The `email` column of the `invitations` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Text,
        /// The `state` column of the `invitations` table.
        ///
        /// Its SQL type is `InvitationStateType`.
        ///
        /// (Automatically generated by Diesel.)
        state -> InvitationStateType,
        /// The `project_id` column of the `invitations` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        project_id -> Int4,
        /// The `invited_by_id` column of the `invitations` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        invited_by_id -> Int4,
        /// The `created_at` column of the `invitations` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `invitations` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `bind_token` column of the `invitations` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        bind_token -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use jirs_data::sql::*;

    /// Representation of the `issue_assignees` table.
    ///
    /// (Automatically generated by Diesel.)
    issue_assignees (id) {
        /// The `id` column of the `issue_assignees` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `issue_id` column of the `issue_assignees` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        issue_id -> Int4,
        /// The `user_id` column of the `issue_assignees` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `created_at` column of the `issue_assignees` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `issue_assignees` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use jirs_data::sql::*;

    /// Representation of the `issues` table.
    ///
    /// (Automatically generated by Diesel.)
    issues (id) {
        /// The `id` column of the `issues` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `title` column of the `issues` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Text,
        /// The `issue_type` column of the `issues` table.
        ///
        /// Its SQL type is `IssueTypeType`.
        ///
        /// (Automatically generated by Diesel.)
        issue_type -> IssueTypeType,
        /// The `status` column of the `issues` table.
        ///
        /// Its SQL type is `IssueStatusType`.
        ///
        /// (Automatically generated by Diesel.)
        status -> IssueStatusType,
        /// The `priority` column of the `issues` table.
        ///
        /// Its SQL type is `IssuePriorityType`.
        ///
        /// (Automatically generated by Diesel.)
        priority -> IssuePriorityType,
        /// The `list_position` column of the `issues` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        list_position -> Int4,
        /// The `description` column of the `issues` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Nullable<Text>,
        /// The `description_text` column of the `issues` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        description_text -> Nullable<Text>,
        /// The `estimate` column of the `issues` table.
        ///
        /// Its SQL type is `Nullable<Float8>`.
        ///
        /// (Automatically generated by Diesel.)
        estimate -> Nullable<Float8>,
        /// The `time_spent` column of the `issues` table.
        ///
        /// Its SQL type is `Nullable<Float8>`.
        ///
        /// (Automatically generated by Diesel.)
        time_spent -> Nullable<Float8>,
        /// The `time_remaining` column of the `issues` table.
        ///
        /// Its SQL type is `Nullable<Float8>`.
        ///
        /// (Automatically generated by Diesel.)
        time_remaining -> Nullable<Float8>,
        /// The `reporter_id` column of the `issues` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        reporter_id -> Int4,
        /// The `project_id` column of the `issues` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        project_id -> Int4,
        /// The `created_at` column of the `issues` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `issues` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use jirs_data::sql::*;

    /// Representation of the `projects` table.
    ///
    /// (Automatically generated by Diesel.)
    projects (id) {
        /// The `id` column of the `projects` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `name` column of the `projects` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
        /// The `url` column of the `projects` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        url -> Text,
        /// The `description` column of the `projects` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Text,
        /// The `category` column of the `projects` table.
        ///
        /// Its SQL type is `ProjectCategoryType`.
        ///
        /// (Automatically generated by Diesel.)
        category -> ProjectCategoryType,
        /// The `created_at` column of the `projects` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `projects` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `time_tracking` column of the `projects` table.
        ///
        /// Its SQL type is `TimeTrackingType`.
        ///
        /// (Automatically generated by Diesel.)
        time_tracking -> TimeTrackingType,
    }
}

table! {
    use diesel::sql_types::*;
    use jirs_data::sql::*;

    /// Representation of the `tokens` table.
    ///
    /// (Automatically generated by Diesel.)
    tokens (id) {
        /// The `id` column of the `tokens` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `tokens` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `access_token` column of the `tokens` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        access_token -> Uuid,
        /// The `refresh_token` column of the `tokens` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        refresh_token -> Uuid,
        /// The `created_at` column of the `tokens` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `tokens` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `bind_token` column of the `tokens` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        bind_token -> Nullable<Uuid>,
    }
}

table! {
    use diesel::sql_types::*;
    use jirs_data::sql::*;

    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `name` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
        /// The `email` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Text,
        /// The `avatar_url` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        avatar_url -> Nullable<Text>,
        /// The `project_id` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        project_id -> Int4,
        /// The `created_at` column of the `users` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `users` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `role` column of the `users` table.
        ///
        /// Its SQL type is `UserRoleType`.
        ///
        /// (Automatically generated by Diesel.)
        role -> UserRoleType,
    }
}

joinable!(comments -> issues (issue_id));
joinable!(comments -> users (user_id));
joinable!(invitations -> projects (project_id));
joinable!(invitations -> users (invited_by_id));
joinable!(issue_assignees -> issues (issue_id));
joinable!(issue_assignees -> users (user_id));
joinable!(issues -> projects (project_id));
joinable!(issues -> users (reporter_id));
joinable!(tokens -> users (user_id));
joinable!(users -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    comments,
    invitations,
    issue_assignees,
    issues,
    projects,
    tokens,
    users,
);
