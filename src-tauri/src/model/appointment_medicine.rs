use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct AppointmentMedicine {
    appointment_id: i64,
    medicine_id: i64,
}

impl AppointmentMedicine {
    pub fn new(appointment_id: i64, medicine_id: i64) -> Self {
        Self {
            appointment_id,
            medicine_id,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AppointmentMedicineEntity {
    pub appointment_id: i64,
    pub medicine_id: i64,
}

impl From<AppointmentMedicineEntity> for AppointmentMedicine {
    fn from(user_entity: AppointmentMedicineEntity) -> Self {
        AppointmentMedicine {
            appointment_id: user_entity.appointment_id,
            medicine_id: user_entity.medicine_id,
        }
    }
}
