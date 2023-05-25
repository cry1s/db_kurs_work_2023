use sqlx::PgPool;

use crate::model::appointment::{Appointment, AppointmentEntity};
// create table public.appointment
// (
//     appointment_id                 serial
//         primary key,
//     doctor_id                      integer
//         references public.doctor
//             on update cascade on delete restrict,
//     patient_outpatient_card_number integer
//         references public.patient
//             on update cascade on delete restrict,
//     appointment_time               timestamp not null,
//     complaints                     text      not null
// );
pub async fn get_all_appointments(pool: &PgPool) -> Result<Vec<Appointment>, sqlx::Error> {
    let appointments_entities = sqlx::query_as!(
        AppointmentEntity,
        r#"
        SELECT t.*
        FROM appointment t
        ORDER BY t.appointment_time
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(appointments_entities.into_iter().map(Appointment::from).collect())
}

pub async fn get_appointment_by_id(
    pool: &PgPool,
    appointment_id: i32,
) -> Result<Appointment, sqlx::Error> {
    let appointment_entity = sqlx::query_as!(
        AppointmentEntity,
        r#"
        SELECT t.*
        FROM appointment t
        WHERE t.appointment_id = $1
        "#,
        appointment_id
    )
    .fetch_one(pool)
    .await?;
    Ok(Appointment::from(appointment_entity))
}

pub async fn add_appointment(
    pool: &PgPool,
    appointment: Appointment,
) -> Result<i32, sqlx::Error> {
    let appointment_entity = sqlx::query_as!(
        AppointmentEntity,
        r#"
        INSERT INTO appointment (doctor_id, patient_outpatient_card_number, appointment_time, complaints)
        VALUES ($1, $2, $3, $4)
        RETURNING appointment_id, doctor_id, patient_outpatient_card_number, appointment_time, complaints
        "#,
        appointment.doctor_id(),
        appointment.patient_outpatient_card_number(),
        appointment.appointment_time(),
        appointment.complaints()
    )
    .fetch_one(pool)
    .await?;
    Ok(appointment_entity.appointment_id)
}

pub async fn update_appointment(
    pool: &PgPool,
    appointment: Appointment,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE appointment
        SET doctor_id = $1, patient_outpatient_card_number = $2, appointment_time = $3, complaints = $4
        WHERE appointment_id = $5
        "#,
        appointment.doctor_id(),
        appointment.patient_outpatient_card_number(),
        appointment.appointment_time(),
        appointment.complaints(),
        appointment.appointment_id().unwrap()
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_appointment(
    pool: &PgPool,
    appointment_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM appointment
        WHERE appointment_id = $1
        "#,
        appointment_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_appointments_by_doctor_id(
    pool: &PgPool,
    doctor_id: i32,
) -> Result<Vec<Appointment>, sqlx::Error> {
    let appointments_entities = sqlx::query_as!(
        AppointmentEntity,
        r#"
        SELECT t.*
        FROM appointment t
        WHERE t.doctor_id = $1
        ORDER BY t.appointment_time
        "#,
        doctor_id
    )
    .fetch_all(pool)
    .await?;
    Ok(appointments_entities.into_iter().map(Appointment::from).collect())
}

pub async fn get_appointments_by_patient_outpatient_card_number(
    pool: &PgPool,
    patient_outpatient_card_number: i32,
) -> Result<Vec<Appointment>, sqlx::Error> {
    let appointments_entities = sqlx::query_as!(
        AppointmentEntity,
        r#"
        SELECT t.*
        FROM appointment t
        WHERE t.patient_outpatient_card_number = $1
        ORDER BY t.appointment_time
        "#,
        patient_outpatient_card_number
    )
    .fetch_all(pool)
    .await?;
    Ok(appointments_entities.into_iter().map(Appointment::from).collect())
}

// using table appointment_diagnosis
pub async fn get_appointments_by_diagnosis_id(
    pool: &PgPool,
    diagnosis_id: i32,
) -> Result<Vec<Appointment>, sqlx::Error> {
    let appointments_entities = sqlx::query_as!(
        AppointmentEntity,
        r#"
        SELECT t.*
        FROM appointment t
        INNER JOIN appointment_diagnosis ad ON t.appointment_id = ad.appointment_id
        WHERE ad.diagnosis_id = $1
        ORDER BY t.appointment_time
        "#,
        diagnosis_id
    )
    .fetch_all(pool)
    .await?;
    Ok(appointments_entities.into_iter().map(Appointment::from).collect())
}

// using table appointment_medication
pub async fn get_appointments_by_medication_id(
    pool: &PgPool,
    medication_id: i32,
) -> Result<Vec<Appointment>, sqlx::Error> {
    let appointments_entities = sqlx::query_as!(
        AppointmentEntity,
        r#"
        SELECT t.*
        FROM appointment t
        INNER JOIN appointment_medication am ON t.appointment_id = am.appointment_id
        WHERE am.medication_id = $1
        ORDER BY t.appointment_time
        "#,
        medication_id
    )
    .fetch_all(pool)
    .await?;
    Ok(appointments_entities.into_iter().map(Appointment::from).collect())
}
