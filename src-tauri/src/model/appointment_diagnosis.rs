use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct AppointmentDiagnosis {
    appointment_id: i64,
    diagnosis_id: i64,
}

impl AppointmentDiagnosis {
    pub fn new(appointment_id: i64, diagnosis_id: i64) -> Self {
        Self {
            appointment_id,
            diagnosis_id,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AppointmentDiagnosisEntity {
    pub appointment_id: i64,
    pub diagnosis_id: i64,
}

impl From<AppointmentDiagnosisEntity> for AppointmentDiagnosis {
    fn from(user_entity: AppointmentDiagnosisEntity) -> Self {
        AppointmentDiagnosis {
            appointment_id: user_entity.appointment_id,
            diagnosis_id: user_entity.diagnosis_id,
        }
    }
}
