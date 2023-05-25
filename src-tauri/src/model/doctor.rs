use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Doctor {
    id: Option<i64>, // None if created manually, Some(id) if retrieved from db
    fio: String,
    id_speciality: i64,
    qualification: String,
}

impl Doctor {
    pub fn new(fio: String, id_speciality: i64, qualification: String) -> Self {
        Self {
            id: None,
            fio,
            id_speciality,
            qualification,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DoctorEntity {
    pub id: i64,
    pub fio: String,
    pub id_speciality: i64,
    pub qualification: String,
}

impl From<DoctorEntity> for Doctor {
    fn from(user_entity: DoctorEntity) -> Self {
        Doctor {
            id: Some(user_entity.id),
            fio: user_entity.fio,
            id_speciality: user_entity.id_speciality,
            qualification: user_entity.qualification,
        }
    }
}
