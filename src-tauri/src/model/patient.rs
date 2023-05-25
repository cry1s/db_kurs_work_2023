use chrono::prelude::*;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Patient {
    id: Option<i64>, // None if created manually, Some(id) if retrieved from db
    fio: String,
    nomer_polisa: String,
    nomer_snilsa: String,
    contacts: String,
    address: String,
    sex: String,
    dob: DateTime<Utc>,
}

impl Patient {
    pub fn new(
        fio: String,
        nomer_polisa: String,
        nomer_snilsa: String,
        contacts: String,
        address: String,
        sex: String,
        dob: DateTime<Utc>,
    ) -> Self {
        Self {
            id: None,
            fio,
            nomer_polisa,
            nomer_snilsa,
            contacts,
            address,
            sex,
            dob,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PatientEntity {
    pub id: i64,
    pub fio: String,
    pub nomer_polisa: String,
    pub nomer_snilsa: String,
    pub contacts: String,
    pub address: String,
    pub sex: String,
    pub dob: DateTime<Utc>,
}

impl From<PatientEntity> for Patient {
    fn from(user_entity: PatientEntity) -> Self {
        Patient {
            id: Some(user_entity.id),
            fio: user_entity.fio,
            nomer_polisa: user_entity.nomer_polisa,
            nomer_snilsa: user_entity.nomer_snilsa,
            contacts: user_entity.contacts,
            address: user_entity.address,
            sex: user_entity.sex,
            dob: user_entity.dob,
        }
    }
}
