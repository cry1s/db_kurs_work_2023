use sqlx::PgPool;

use crate::model::diagnosis::{Diagnosis, DiagnosisEntity};
// create table public.diagnosis
// (
//     diagnosis_id    serial
//         primary key,
//     disease_name    varchar(255) not null,
//     hospitalization boolean      not null
// );
pub async fn get_all_diagnoses(pool: &PgPool) -> Result<Vec<Diagnosis>, sqlx::Error> {
    let diagnoses_entities = sqlx::query_as!(
        DiagnosisEntity,
        r#"
        SELECT t.*
        FROM diagnosis t
        ORDER BY t.disease_name
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(diagnoses_entities.into_iter().map(Diagnosis::from).collect())
}

pub async fn get_diagnosis_by_id(
    pool: &PgPool,
    diagnosis_id: i32,
) -> Result<Diagnosis, sqlx::Error> {
    let diagnosis_entity = sqlx::query_as!(
        DiagnosisEntity,
        r#"
        SELECT t.*
        FROM diagnosis t
        WHERE t.diagnosis_id = $1
        "#,
        diagnosis_id
    )
    .fetch_one(pool)
    .await?;
    Ok(Diagnosis::from(diagnosis_entity))
}

pub async fn add_diagnosis(
    pool: &PgPool,
    diagnosis: Diagnosis,
) -> Result<i32, sqlx::Error> {
    let diagnosis_entity = sqlx::query_as!(
        DiagnosisEntity,
        r#"
        INSERT INTO diagnosis (disease_name, hospitalization)
        VALUES ($1, $2)
        RETURNING diagnosis_id, disease_name, hospitalization
        "#,
        diagnosis.disease_name(),
        diagnosis.hospitalization()
    )
    .fetch_one(pool)
    .await?;

    Ok(diagnosis_entity.diagnosis_id)
}

pub async fn update_diagnosis(
    pool: &PgPool,
    diagnosis: Diagnosis,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE diagnosis
        SET disease_name = $1, hospitalization = $2
        WHERE diagnosis_id = $3
        "#,
        diagnosis.disease_name(),
        diagnosis.hospitalization(),
        diagnosis.diagnosis_id().unwrap()
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_diagnosis(
    pool: &PgPool,
    diagnosis_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM diagnosis
        WHERE diagnosis_id = $1
        "#,
        diagnosis_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

//using table appointment_diagnosis
pub async fn get_diagnosises_by_appointment_id(
    pool: &PgPool,
    appointment_id: i32,
) -> Result<Vec<Diagnosis>, sqlx::Error> {
    let diagnoses_entities = sqlx::query_as!(
        DiagnosisEntity,
        r#"
        SELECT d.*
        FROM diagnosis d
        INNER JOIN appointment_diagnosis ad ON ad.diagnosis_id = d.diagnosis_id
        WHERE ad.appointment_id = $1
        "#,
        appointment_id
    ).fetch_all(pool).await?;
    Ok(diagnoses_entities.into_iter().map(Diagnosis::from).collect())
}

pub async fn find_diagnosises_by_disease_name(
    pool: &PgPool,
    disease_name: &str,
) -> Result<Vec<Diagnosis>, sqlx::Error> {
    let diagnosis_entities = sqlx::query_as!(
        DiagnosisEntity,
        r#"
        SELECT d.*
        FROM diagnosis d
        WHERE d.disease_name LIKE $1
        "#,
        disease_name
    ).fetch_all(pool).await?;
    Ok(diagnosis_entities.into_iter().map(Diagnosis::from).collect())
}