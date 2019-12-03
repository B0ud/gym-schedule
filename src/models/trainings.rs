use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::trainings;

#[derive(Debug, Queryable)]
//#[table_name = "trainings"]
pub struct Trainings {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub image: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
