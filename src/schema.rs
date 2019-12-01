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
