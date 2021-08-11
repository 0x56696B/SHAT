table! {
    chats (id) {
        id -> Int4,
        issuer_id -> Int4,
        message_id -> Int4,
    }
}

table! {
    messages (id) {
        id -> Int4,
        message -> Text,
        issuer_id -> Int4,
    }
}

table! {
    people (id) {
        id -> Int4,
        username -> Nullable<Varchar>,
    }
}

joinable!(chats -> messages (message_id));
joinable!(chats -> people (issuer_id));
joinable!(messages -> people (issuer_id));

allow_tables_to_appear_in_same_query!(
    chats,
    messages,
    people,
);
