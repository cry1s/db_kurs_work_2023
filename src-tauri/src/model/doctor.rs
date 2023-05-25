use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Doctor {
    doctor_id: Option<i32>, // None if created manually, Some(doctor_id) if retrieved from db
    full_name: String,
    specialty_id: i32,
    qualification: String,
}

impl Doctor {
    pub fn new(full_name: String, specialty_id: i32, qualification: String) -> Self {
        Self {
            doctor_id: None,
            full_name,
            specialty_id,
            qualification,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DoctorEntity {
    pub doctor_id: i32,
    pub full_name: String,
    pub specialty_id: i32,
    pub qualification: String,
}

impl From<DoctorEntity> for Doctor {
    fn from(user_entity: DoctorEntity) -> Self {
        Doctor {
            doctor_id: Some(user_entity.doctor_id),
            full_name: user_entity.full_name,
            specialty_id: user_entity.specialty_id,
            qualification: user_entity.qualification,
        }
    }
}
