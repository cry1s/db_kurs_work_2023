use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Medicine {
    id: Option<i64>, // None if created manually, Some(id) if retrieved from db
    name: String,
    course: String,
    dosage: String,
}

impl Medicine {
    pub fn new(name: String, course: String, dosage: String) -> Self {
        Self {
            id: None,
            name,
            course,
            dosage,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MedicineEntity {
    pub id: i64,
    pub name: String,
    pub course: String,
    pub dosage: String,
}

impl From<MedicineEntity> for Medicine {
    fn from(user_entity: MedicineEntity) -> Self {
        Medicine {
            id: Some(user_entity.id),
            name: user_entity.name,
            course: user_entity.course,
            dosage: user_entity.dosage,
        }
    }
}
