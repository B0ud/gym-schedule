use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::trainings;

//#[table_name = "trainings"]
#[derive(Debug, Queryable)]
pub struct Trainings {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Json Response Object

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainingsResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
}
impl From<Trainings> for TrainingsResponse {
    fn from(trainings: Trainings) -> Self {
        TrainingsResponse {
            id: trainings.id,
            name: trainings.name,
            description: trainings.description,
            image: trainings.image,
        }
    }
}
