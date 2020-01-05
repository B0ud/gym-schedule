table! {
    muscle_tags (id) {
        id -> Uuid,
        name -> Text,
        code -> Text,
        description -> Nullable<Text>,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    trainings (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    muscle_tags,
    trainings,
);
