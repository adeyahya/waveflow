table! {
    configs (id) {
        id -> Nullable<Integer>,
        name -> Text,
        value -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        email -> Text,
        password -> Text,
        is_admin -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    workflows (id) {
        id -> Text,
        name -> Text,
        slug -> Text,
        secret -> Text,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    workflows_history (id) {
        id -> Text,
        workflow_id -> Text,
        content -> Nullable<Text>,
        is_success -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    configs,
    users,
    workflows,
    workflows_history,
);
