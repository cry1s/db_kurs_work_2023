use sqlx::PgPool;

use crate::model::medication::{Medication, MedicationEntity};
// create table public.medication
// (
//     medication_id  serial
//         primary key,
//     name           varchar(255) not null,
//     treatment_plan varchar(255) not null,
//     dosage         varchar(255) not null
// );
pub async fn get_all_medications(pool: &PgPool) -> Result<Vec<Medication>, sqlx::Error> {
    let medications_entities = sqlx::query_as!(
        MedicationEntity,
        r#"
        SELECT t.*
        FROM medication t
        ORDER BY t.name
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(medications_entities.into_iter().map(Medication::from).collect())
}

pub async fn get_medication_by_id(
    pool: &PgPool,
    medication_id: i32,
) -> Result<Medication, sqlx::Error> {
    let medication_entity = sqlx::query_as!(
        MedicationEntity,
        r#"
        SELECT t.*
        FROM medication t
        WHERE t.medication_id = $1
        "#,
        medication_id
    )
    .fetch_one(pool)
    .await?;
    Ok(Medication::from(medication_entity))
}

pub async fn add_medication(
    pool: &PgPool,
    medication: Medication,
) -> Result<i32, sqlx::Error> {
    let medication_entity = sqlx::query_as!(
        MedicationEntity,
        r#"
        INSERT INTO medication (name, treatment_plan, dosage)
        VALUES ($1, $2, $3)
        RETURNING medication_id, name, treatment_plan, dosage
        "#,
        medication.name(),
        medication.treatment_plan(),
        medication.dosage()
    )
    .fetch_one(pool)
    .await?;

    Ok(medication_entity.medication_id)
}

pub async fn update_medication(
    pool: &PgPool,
    medication: Medication,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE medication
        SET name = $1, treatment_plan = $2, dosage = $3
        WHERE medication_id = $4
        "#,
        medication.name(),
        medication.treatment_plan(),
        medication.dosage(),
        medication.medication_id().unwrap()
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_medication(
    pool: &PgPool,
    medication_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM medication
        WHERE medication_id = $1
        "#,
        medication_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

//using table appointment_medication
pub async fn get_medications_by_appointment_id(
    pool: &PgPool,
    appointment_id: i32,
) -> Result<Vec<Medication>, sqlx::Error> {
    let medications_entities = sqlx::query_as!(
        MedicationEntity,
        r#"
        SELECT m.*
        FROM medication m
        INNER JOIN appointment_medication am ON am.medication_id = m.medication_id
        WHERE am.appointment_id = $1
        "#,
        appointment_id
    ).fetch_all(pool).await?;
    Ok(medications_entities.into_iter().map(Medication::from).collect())
}

pub async fn find_medications_by_name(
    pool: &PgPool,
    name: &str,
) -> Result<Vec<Medication>, sqlx::Error> {
    let medications_entities = sqlx::query_as!(
        MedicationEntity,
        r#"
        SELECT m.*
        FROM medication m
        WHERE m.name LIKE $1
        "#,
        name
    ).fetch_all(pool).await?;
    Ok(medications_entities.into_iter().map(Medication::from).collect())
}