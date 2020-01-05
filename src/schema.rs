table! {
    asso_muscle_tags_exercises (exercise_id, muscle_tags) {
        exercise_id -> Uuid,
        muscle_tags -> Uuid,
    }
}

table! {
    exercises (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        category -> Text,
        category_icon -> Nullable<Text>,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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

joinable!(asso_muscle_tags_exercises -> exercises (exercise_id));
joinable!(asso_muscle_tags_exercises -> muscle_tags (muscle_tags));

allow_tables_to_appear_in_same_query!(
    asso_muscle_tags_exercises,
    exercises,
    muscle_tags,
    trainings,
);
