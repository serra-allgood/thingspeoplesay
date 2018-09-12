table! {
    colors (id) {
        id -> Int8,
        hexcode -> Varchar,
    }
}

table! {
    gradients (id) {
        id -> Int8,
        message_id -> Int8,
        color_id -> Int8,
        position -> Int4,
    }
}

table! {
    messages (id) {
        id -> Int8,
        message -> Text,
        created_at -> Timestamp,
    }
}

joinable!(gradients -> colors (color_id));
joinable!(gradients -> messages (message_id));

allow_tables_to_appear_in_same_query!(
    colors,
    gradients,
    messages,
);
