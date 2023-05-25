use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
// create table public.Specialty
// (
//     specialty_id        serial
//         primary key,
//     name                varchar(255),
//     education_duration integer
// );
#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Speciality {
    specialty_id: Option<i32>, // None if created manually, Some(id) if retrieved from db
    name: String,
    education_duration: i32,
}

impl Speciality {
    pub fn new(name: String, education_duration: i32) -> Self {
        Self {
            specialty_id: None,
            name,
            education_duration,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SpecialityEntity {
    pub specialty_id: i32,
    pub name: String,
    pub education_duration: i32,
}

impl From<SpecialityEntity> for Speciality {
    fn from(user_entity: SpecialityEntity) -> Self {
        Speciality {
            specialty_id: Some(user_entity.specialty_id),
            name: user_entity.name,
            education_duration: user_entity.education_duration,
        }
    }
}
