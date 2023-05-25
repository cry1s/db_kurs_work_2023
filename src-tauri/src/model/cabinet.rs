use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Cabinet {
    cabinet_number: i32,
    working_doctor_id: Option<i32>,
    phone: String,
    working_hours: String,
}

impl Cabinet {
    pub fn new(cabinet_number: i32, id_doctor: Option<i32>, phone: String, working_hours: String) -> Self {
        Self {
            cabinet_number,
            working_doctor_id: id_doctor,
            phone,
            working_hours,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CabinetEntity {
    pub cabinet_number: i32,
    pub working_doctor_id: Option<i32>,
    pub phone: String,
    pub working_hours: String,
}

impl From<CabinetEntity> for Cabinet {
    fn from(user_entity: CabinetEntity) -> Self {
        Cabinet {
            cabinet_number: user_entity.cabinet_number,
            working_doctor_id: user_entity.working_doctor_id,
            phone: user_entity.phone,
            working_hours: user_entity.working_hours,
        }
    }
}
