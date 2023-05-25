use sqlx::PgPool;

use crate::model::doctor::{Doctor, DoctorEntity};
// create table public.doctor
// (
//     doctor_id     serial
//         primary key,
//     full_name     varchar(255)      not null,
//     specialty_id  integer default 1 not null
//         references public.specialty
//             on update cascade on delete set default,
//     qualification varchar(255)      not null
// );

pub async fn get_all_doctors(pool: &PgPool) -> Result<Vec<Doctor>, sqlx::Error> {
    let doctors_entities = sqlx::query_as!(
        DoctorEntity,
        r#"
        SELECT t.*
        FROM doctor t
        ORDER BY t.full_name
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(doctors_entities.into_iter().map(Doctor::from).collect())
}

pub async fn get_doctor_by_id(
    pool: &PgPool,
    doctor_id: i32,
) -> Result<Doctor, sqlx::Error> {
    let doctor_entity = sqlx::query_as!(
        DoctorEntity,
        r#"
        SELECT t.*
        FROM doctor t
        WHERE t.doctor_id = $1
        "#,
        doctor_id
    )
    .fetch_one(pool)
    .await?;
    Ok(Doctor::from(doctor_entity))
}

pub async fn add_doctor(
    pool: &PgPool,
    doctor: Doctor,
) -> Result<i32, sqlx::Error> {
    let doctor_entity = sqlx::query_as!(
        DoctorEntity,
        r#"
        INSERT INTO doctor (full_name, specialty_id, qualification)
        VALUES ($1, $2, $3)
        RETURNING doctor_id, full_name, specialty_id, qualification
        "#,
        doctor.full_name(),
        doctor.specialty_id(),
        doctor.qualification(),
    ).fetch_one(pool).await?;
    Ok(doctor_entity.doctor_id)
}

pub async fn update_doctor(
    pool: &PgPool,
    doctor: Doctor,
) -> Result<i32, sqlx::Error> {
    let doctor_entity = sqlx::query_as!(
        DoctorEntity,
        r#"
        UPDATE doctor
        SET full_name = $1, specialty_id = $2, qualification = $3
        WHERE doctor_id = $4
        RETURNING doctor_id, full_name, specialty_id, qualification
        "#,
        doctor.full_name(),
        doctor.specialty_id(),
        doctor.qualification(),
        doctor.doctor_id().unwrap()
    ).fetch_one(pool).await?;
    Ok(doctor_entity.doctor_id)
}

pub async fn delete_doctor(
    pool: &PgPool,
    doctor_id: i32,
) -> Result<i32, sqlx::Error> {
    let doctor_entity = sqlx::query_as!(
        DoctorEntity,
        r#"
        DELETE FROM doctor
        WHERE doctor_id = $1
        RETURNING doctor_id, full_name, specialty_id, qualification
        "#,
        doctor_id
    )
    .fetch_one(pool)
    .await?;
    Ok(doctor_entity.doctor_id)
}

pub async fn find_doctors_by_full_name(
    pool: &PgPool,
    full_name: &str,
) -> Result<Vec<Doctor>, sqlx::Error> {
    let doctors_entitys = sqlx::query_as!(
        DoctorEntity,
        r#"
        SELECT t.*
        FROM doctor t
        WHERE t.full_name like $1
        "#,
        full_name
    )
    .fetch_all(pool)
    .await?;
    Ok(doctors_entitys.into_iter().map(Doctor::from).collect())
}

pub async fn get_doctor_by_appointment_id(
    pool: &PgPool,
    appointment_id: i32,
) -> Result<Doctor, sqlx::Error> {
    let doctor_entity = sqlx::query_as!(
        DoctorEntity,
        r#"
        SELECT t.*
        FROM doctor t
        INNER JOIN appointment a ON a.doctor_id = t.doctor_id
        WHERE a.appointment_id = $1
        "#,
        appointment_id
    )
    .fetch_one(pool)
    .await?;
    Ok(Doctor::from(doctor_entity))
}
