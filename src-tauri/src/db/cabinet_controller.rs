use sqlx::PgPool;

use crate::model::cabinet::{Cabinet, CabinetEntity};
// create table public.cabinet
// (
//     working_doctor_id integer
//                                    references public.doctor
//                                        on update cascade on delete set null,
//     phone             varchar(255) not null,
//     working_hours     varchar(255) not null,
//     cabinet_number    integer      not null
//         primary key
// );

pub async fn get_all_cabinets(pool: &PgPool) -> Result<Vec<Cabinet>, sqlx::Error> {
    let cabinets_entities = sqlx::query_as!(
        CabinetEntity,
        r#"
        SELECT t.*
        FROM cabinet t
        ORDER BY t.cabinet_number
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(cabinets_entities.into_iter().map(Cabinet::from).collect())
}

pub async fn get_cabinet_by_id(
    pool: &PgPool,
    cabinet_number: i32,
) -> Result<Cabinet, sqlx::Error> {
    let cabinet_entity = sqlx::query_as!(
        CabinetEntity,
        r#"
        SELECT t.*
        FROM cabinet t
        WHERE t.cabinet_number = $1
        "#,
        cabinet_number
    )
    .fetch_one(pool)
    .await?;
    Ok(Cabinet::from(cabinet_entity))
}

pub async fn add_cabinet(
    pool: &PgPool,
    cabinet: Cabinet,
) -> Result<i32, sqlx::Error> {
    let cabinet_entity = sqlx::query_as!(
        CabinetEntity,
        r#"
        INSERT INTO cabinet (working_doctor_id, phone, working_hours, cabinet_number)
        VALUES ($1, $2, $3, $4)
        RETURNING working_doctor_id, phone, working_hours, cabinet_number
        "#,
        cabinet.working_doctor_id().unwrap(),
        cabinet.phone(),
        cabinet.working_hours(),
        cabinet.cabinet_number()
    )
    .fetch_one(pool)
    .await?;
    Ok(cabinet_entity.cabinet_number)
}

pub async fn update_cabinet(
    pool: &PgPool,
    cabinet: Cabinet,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE cabinet
        SET working_doctor_id = $1, phone = $2, working_hours = $3, cabinet_number = $4
        WHERE cabinet_number = $4
        "#,
        cabinet.working_doctor_id().unwrap(),
        cabinet.phone(),
        cabinet.working_hours(),
        cabinet.cabinet_number()
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_cabinet(
    pool: &PgPool,
    cabinet_number: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM cabinet
        WHERE cabinet_number = $1
        "#,
        cabinet_number
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_cabinet_by_doctor_id(
    pool: &PgPool,
    doctor_id: i32,
) -> Result<Cabinet, sqlx::Error> {
    let cabinet_entity = sqlx::query_as!(
        CabinetEntity,
        r#"
        SELECT t.*
        FROM cabinet t
        WHERE t.working_doctor_id = $1
        "#,
        doctor_id
    )
    .fetch_one(pool)
    .await?;
    Ok(Cabinet::from(cabinet_entity))
}

pub async fn get_cabinet_by_appointment_id(
    pool: &PgPool,
    appointment_id: i32,
) -> Result<Cabinet, sqlx::Error> {
    let cabinet_entity = sqlx::query_as!(
        CabinetEntity,
        r#"
        SELECT t.*
        FROM cabinet t
        INNER JOIN doctor d ON t.working_doctor_id = d.doctor_id
        INNER JOIN appointment a ON d.doctor_id = a.doctor_id
        WHERE a.appointment_id = $1
        "#,
        appointment_id
    )
    .fetch_one(pool)
    .await?;
    Ok(Cabinet::from(cabinet_entity))
}
