use chrono::prelude::*;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Patient {
    outpatient_card_number: Option<i32>, // None if created manually, Some(outpatient_card_number) if retrieved from db
    full_name: String,
    insurance_number: String,
    snils_number: String,
    contacts: String,
    address: String,
    gender: String,
    date_of_birth: NaiveDate,
}

impl Patient {
    pub fn new(
        full_name: String,
        insurance_number: String,
        snils_number: String,
        contacts: String,
        address: String,
        gender: String,
        date_of_birth: NaiveDate,
    ) -> Self {
        Self {
            outpatient_card_number: None,
            full_name,
            insurance_number,
            snils_number,
            contacts,
            address,
            gender,
            date_of_birth,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PatientEntity {
    pub outpatient_card_number: i32,
    pub full_name: String,
    pub insurance_number: String,
    pub snils_number: String,
    pub contacts: String,
    pub address: String,
    pub gender: String,
    pub date_of_birth: NaiveDate,
}

impl From<PatientEntity> for Patient {
    fn from(user_entity: PatientEntity) -> Self {
        Patient {
            outpatient_card_number: Some(user_entity.outpatient_card_number),
            full_name: user_entity.full_name,
            insurance_number: user_entity.insurance_number,
            snils_number: user_entity.snils_number,
            contacts: user_entity.contacts,
            address: user_entity.address,
            gender: user_entity.gender,
            date_of_birth: user_entity.date_of_birth,
        }
    }
}
