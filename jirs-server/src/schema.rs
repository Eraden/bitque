table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        user_id -> Int4,
        issue_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    issue_assignees (id) {
        id -> Int4,
        issue_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    issues (id) {
        id -> Int4,
        title -> Text,
        issue_type -> Text,
        status -> Text,
        priority -> Text,
        list_position -> Float8,
        description -> Nullable<Text>,
        description_text -> Nullable<Text>,
        estimate -> Nullable<Int4>,
        time_spent -> Nullable<Int4>,
        time_remaining -> Nullable<Int4>,
        reporter_id -> Int4,
        project_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    projects (id) {
        id -> Int4,
        name -> Text,
        url -> Text,
        description -> Text,
        category -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tokens (id) {
        id -> Int4,
        user_id -> Int4,
        access_token -> Uuid,
        refresh_token -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        avatar_url -> Nullable<Text>,
        project_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(comments -> issues (issue_id));
joinable!(comments -> users (user_id));
joinable!(issue_assignees -> issues (issue_id));
joinable!(issue_assignees -> users (user_id));
joinable!(issues -> projects (project_id));
joinable!(issues -> users (reporter_id));
joinable!(tokens -> users (user_id));
joinable!(users -> projects (project_id));

allow_tables_to_appear_in_same_query!(comments, issue_assignees, issues, projects, tokens, users,);
