use sqlx::PgPool;

use crate::model::patient::{Patient, PatientEntity};

// create table public.patient
// (
//     outpatient_card_number serial
//         primary key,
//     full_name              varchar(255) not null,
//     insurance_number       varchar(255) not null,
//     snils_number           varchar(255) not null,
//     contacts               varchar(255) not null,
//     address                varchar(255) not null,
//     gender                 varchar(255) not null,
//     date_of_birth          date         not null
// );
pub async fn get_all_patients(pool: &PgPool) -> Result<Vec<Patient>, sqlx::Error> {
    let patients_entities = sqlx::query_as!(
        PatientEntity,
        r#"
        SELECT t.*
        FROM patient t
        ORDER BY t.full_name
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(patients_entities.into_iter().map(Patient::from).collect())
}

pub async fn get_patient_by_outpatient_card_number(
    pool: &PgPool,
    outpatient_card_number: i32,
) -> Result<Patient, sqlx::Error> {
    let patient_entity = sqlx::query_as!(
        PatientEntity,
        r#"
        SELECT t.*
        FROM patient t
        WHERE t.outpatient_card_number = $1
        "#,
        outpatient_card_number
    )
    .fetch_one(pool)
    .await?;
    Ok(Patient::from(patient_entity))
}

pub async fn get_patient_by_appointment_id (
    pool: &PgPool,
    appointment_id: i32,
) -> Result<Patient, sqlx::Error> {
    let patient_entity = sqlx::query_as!(
        PatientEntity,
        r#"
        SELECT t.*
        FROM patient t
        INNER JOIN appointment a ON a.patient_outpatient_card_number = t.outpatient_card_number
        WHERE a.appointment_id = $1
        "#,
        appointment_id
    )
    .fetch_one(pool)
    .await?;
    Ok(Patient::from(patient_entity))
}

pub async fn add_patient(
    pool: &PgPool,
    patient: Patient,
) -> Result<i32, sqlx::Error> {
    let patient_entity = sqlx::query_as!(
        PatientEntity,
        r#"
        INSERT INTO patient (full_name, insurance_number, snils_number, contacts, address, gender, date_of_birth)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING outpatient_card_number, full_name, insurance_number, snils_number, contacts, address, gender, date_of_birth
        "#,
        patient.full_name(),
        patient.insurance_number(),
        patient.snils_number(),
        patient.contacts(),
        patient.address(),
        patient.gender(),
        patient.date_of_birth()
    ).fetch_one(pool).await?;
    Ok(patient_entity.outpatient_card_number)
}

pub async fn update_patient(
    pool: &PgPool,
    patient: Patient,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE patient
        SET full_name = $1, insurance_number = $2, snils_number = $3, contacts = $4, address = $5, gender = $6, date_of_birth = $7
        WHERE outpatient_card_number = $8
        "#,
        patient.full_name(),
        patient.insurance_number(),
        patient.snils_number(),
        patient.contacts(),
        patient.address(),
        patient.gender(),
        patient.date_of_birth(),
        patient.outpatient_card_number().unwrap()
    ).execute(pool).await?;
    Ok(())
}

pub async fn delete_patient(
    pool: &PgPool,
    outpatient_card_number: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM patient
        WHERE outpatient_card_number = $1
        "#,
        outpatient_card_number
    ).execute(pool).await?;
    Ok(())
}

pub async fn find_patients_by_full_name(
    pool: &PgPool,
    full_name: &str
) -> Result<Vec<Patient>, sqlx::Error> {
    let patients_entitys = sqlx::query_as!(
        PatientEntity,
        r#"
        SELECT p.*
        FROM patient p
        WHERE p.full_name LIKE $1
        "#,
        full_name
    ).fetch_all(pool).await?;
    Ok(patients_entitys.into_iter().map(Patient::from).collect())
}