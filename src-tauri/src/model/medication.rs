use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Medication {
    medication_id: Option<i32>, // None if created manually, Some(id) if retrieved from db
    name: String,
    treatment_plan: String,
    dosage: String,
}

impl Medication {
    pub fn new(name: String, course: String, dosage: String) -> Self {
        Self {
            medication_id: None,
            name,
            treatment_plan: course,
            dosage,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MedicationEntity {
    pub medication_id: i32,
    pub name: String,
    pub treatment_plan: String,
    pub dosage: String,
}

impl From<MedicationEntity> for Medication {
    fn from(user_entity: MedicationEntity) -> Self {
        Medication {
            medication_id: Some(user_entity.medication_id),
            name: user_entity.name,
            treatment_plan: user_entity.treatment_plan,
            dosage: user_entity.dosage,
        }
    }
}