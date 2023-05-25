use sqlx::PgPool;

use crate::model::speciality::{Speciality, SpecialityEntity};
// create table public.Specialty
// (
//     specialty_id        serial
//         primary key,
//     name                varchar(255),
//     education_duration integer
// );
pub async fn get_all_specialities(pool: &PgPool) -> Result<Vec<Speciality>, sqlx::Error> {
    let specialities_entities = sqlx::query_as!(
        SpecialityEntity,
        r#"
        SELECT t.*
        FROM specialty t
        ORDER BY t.name
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(specialities_entities.into_iter().map(Speciality::from).collect())
}

pub async fn get_speciality_by_id(
    pool: &PgPool,
    specialty_id: i32,
) -> Result<Speciality, sqlx::Error> {
    let speciality_entity = sqlx::query_as!(
        SpecialityEntity,
        r#"
        SELECT t.*
        FROM specialty t
        WHERE t.specialty_id = $1
        "#,
        specialty_id
    )
    .fetch_one(pool)
    .await?;
    Ok(Speciality::from(speciality_entity))
}

pub async fn add_speciality(
    pool: &PgPool,
    speciality: Speciality,
) -> Result<i32, sqlx::Error> {
    let speciality_entity = sqlx::query_as!(
        SpecialityEntity,
        r#"
        INSERT INTO specialty (name, education_duration)
        VALUES ($1, $2)
        RETURNING specialty_id, name, education_duration
        "#,
        speciality.name(),
        speciality.education_duration()
    )
    .fetch_one(pool)
    .await?;

    Ok(speciality_entity.specialty_id)
}

pub async fn update_speciality(
    pool: &PgPool,
    speciality: Speciality,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE specialty
        SET name = $1, education_duration = $2
        WHERE specialty_id = $3
        "#,
        speciality.name(),
        speciality.education_duration(),
        speciality.specialty_id().unwrap(),
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_speciality(
    pool: &PgPool,
    specialty_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM specialty
        WHERE specialty_id = $1
        "#,
        specialty_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_specialities_by_name(
    pool: &PgPool,
    name: &str,
) -> Result<Vec<Speciality>, sqlx::Error> {
    let specialities_entities = sqlx::query_as!(
        SpecialityEntity,
        r#"
        SELECT t.*
        FROM specialty t
        WHERE t.name ILIKE $1
        ORDER BY t.name
        "#,
        format!("%{}%", name)
    )
    .fetch_all(pool)
    .await?;
    Ok(specialities_entities.into_iter().map(Speciality::from).collect())
}