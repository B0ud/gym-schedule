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

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "trainings"]
pub struct NewTrainings {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
}

/*impl Default for Trainings {
    fn default() -> Trainings {
        Trainings {
            id: Uuid::new_v4(),
            name: "".to_string(),
            description: None,
            image: None,
            created_at: (),
            updated_at: (),
        }
    }
}*/

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
#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize)]
pub struct ListResult<T> {
    pub offset: u32,
    pub total: u32,
    pub items: Vec<T>,
}
