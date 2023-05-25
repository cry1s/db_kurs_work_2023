use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Diagnosis {
    diagnosis_id: Option<i32>, // None if created manually, Some(id) if retrieved from db
    disease_name: String,
    hospitalization: bool,
}

impl Diagnosis {
    pub fn new(disease_name: String, hospitalization: bool) -> Self {
        Self {
            diagnosis_id: None,
            disease_name,
            hospitalization,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosisEntity {
    pub diagnosis_id: i32,
    pub disease_name: String,
    pub hospitalization: bool,
}

impl From<DiagnosisEntity> for Diagnosis {
    fn from(user_entity: DiagnosisEntity) -> Self {
        Diagnosis {
            diagnosis_id: Some(user_entity.diagnosis_id),
            disease_name: user_entity.disease_name,
            hospitalization: user_entity.hospitalization,
        }
    }
}
