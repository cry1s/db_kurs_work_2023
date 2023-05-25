use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Cabinet {
    number: i64,
    id_doctor: Option<i64>,
    phone: String,
    work_time: String,
}

impl Cabinet {
    pub fn new(number: i64, id_doctor: Option<i64>, phone: String, work_time: String) -> Self {
        Self {
            number,
            id_doctor,
            phone,
            work_time,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CabinetEntity {
    pub number: i64,
    pub id_doctor: Option<i64>,
    pub phone: String,
    pub work_time: String,
}

impl From<CabinetEntity> for Cabinet {
    fn from(user_entity: CabinetEntity) -> Self {
        Cabinet {
            number: user_entity.number,
            id_doctor: user_entity.id_doctor,
            phone: user_entity.phone,
            work_time: user_entity.work_time,
        }
    }
}
