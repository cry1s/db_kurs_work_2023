use chrono::prelude::*;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Appointment {
    appointment_id: Option<i32>, // None if created manually, Some(appointment_id) if retrieved from db
    doctor_id: i32,
    patient_outpatient_card_number: i32,
    appointment_time: NaiveDateTime,
    complaints: String,
}

impl Appointment {
    pub fn new(
        doctor_id: i32,
        patient_outpatient_card_number: i32,
        appointment_time: NaiveDateTime,
        complaints: String,
    ) -> Self {
        Self {
            appointment_id: None,
            doctor_id,
            patient_outpatient_card_number,
            appointment_time,
            complaints,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AppointmentEntity {
    pub appointment_id: i32,
    pub doctor_id: i32,
    pub patient_outpatient_card_number: i32,
    pub appointment_time: NaiveDateTime,
    pub complaints: String,
}

impl From<AppointmentEntity> for Appointment {
    fn from(user_entity: AppointmentEntity) -> Self {
        Appointment {
            appointment_id: Some(user_entity.appointment_id),
            doctor_id: user_entity.doctor_id,
            patient_outpatient_card_number: user_entity.patient_outpatient_card_number,
            appointment_time: user_entity.appointment_time,
            complaints: user_entity.complaints,
        }
    }
}
