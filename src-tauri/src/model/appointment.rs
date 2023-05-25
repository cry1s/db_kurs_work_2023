use chrono::prelude::*;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Appointment {
    id: Option<i64>, // None if created manually, Some(id) if retrieved from db
    doctor_id: i64,
    patient_id: i64,
    date: DateTime<Utc>,
    description: String,
}

impl Appointment {
    pub fn new(
        doctor_id: i64,
        patient_id: i64,
        date: DateTime<Utc>,
        description: String,
    ) -> Self {
        Self {
            id: None,
            doctor_id,
            patient_id,
            date,
            description,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AppointmentEntity {
    pub id: i64,
    pub doctor_id: i64,
    pub patient_id: i64,
    pub date: DateTime<Utc>,
    pub description: String,
}

impl From<AppointmentEntity> for Appointment {
    fn from(user_entity: AppointmentEntity) -> Self {
        Appointment {
            id: Some(user_entity.id),
            doctor_id: user_entity.doctor_id,
            patient_id: user_entity.patient_id,
            date: user_entity.date,
            description: user_entity.description,
        }
    }
}
