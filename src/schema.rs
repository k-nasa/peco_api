table! {
    fixed_phrases (id) {
        id -> Int4,
        user_id -> Int4,
        yes_text -> Text,
        no_text -> Text,
    }
}

table! {
    send_emails (id) {
        id -> Int4,
        user_id -> Int4,
        email -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        token -> Varchar,
        user_id -> Int4,
        password_digest -> Varchar,
    }
}

joinable!(fixed_phrases -> users (user_id));
joinable!(send_emails -> users (user_id));

allow_tables_to_appear_in_same_query!(
    fixed_phrases,
    send_emails,
    users,
);
