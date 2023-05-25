use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Diagnosis {
    id: Option<i64>, // None if created manually, Some(id) if retrieved from db
    name: String,
    hospitalization: bool,
}

impl Diagnosis {
    pub fn new(name: String, hospitalization: bool) -> Self {
        Self {
            id: None,
            name,
            hospitalization,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosisEntity {
    pub id: i64,
    pub name: String,
    pub hospitalization: bool,
}

impl From<DiagnosisEntity> for Diagnosis {
    fn from(user_entity: DiagnosisEntity) -> Self {
        Diagnosis {
            id: Some(user_entity.id),
            name: user_entity.name,
            hospitalization: user_entity.hospitalization,
        }
    }
}
