table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        email -> Text,
        password -> Text,
        is_admin -> Bool,
    }
}
