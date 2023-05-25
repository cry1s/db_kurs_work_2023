use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Speciality {
    id: Option<i64>, // None if created manually, Some(id) if retrieved from db
    name: String,
    months_to_study: i32,
}

impl Speciality {
    pub fn new(name: String, months_to_study: i32) -> Self {
        Self {
            id: None,
            name,
            months_to_study,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SpecialityEntity {
    pub id: i64,
    pub name: String,
    pub months_to_study: i32,
}

impl From<SpecialityEntity> for Speciality {
    fn from(user_entity: SpecialityEntity) -> Self {
        Speciality {
            id: Some(user_entity.id),
            name: user_entity.name,
            months_to_study: user_entity.months_to_study,
        }
    }
}
