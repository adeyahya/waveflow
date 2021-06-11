table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        email -> Text,
        password -> Text,
        is_admin -> Bool,
    }
}

table! {
    workflows (id) {
        id -> Text,
        name -> Text,
        slug -> Text,
        secret -> Text,
        content -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    workflows,
);
