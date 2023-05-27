use tauri::State;

use crate::{
    db::{appointment_controller, connection::DbConnectionPool, patient_controller},
    model::{appointment::Appointment, patient::Patient},
};

use super::login_service::{Authorization, Authorized};

#[tauri::command]
pub async fn get_my_appointments(
    connection: State<'_, DbConnectionPool>,
    authorized_state: State<'_, Authorization>,
) -> Result<Vec<Appointment>, String> {
    let pool = &*connection.connection.lock().await;
    let patient = match &*authorized_state.credentials.lock().await {
        Authorized::Patient(patient) => patient.clone(),
        _ => return Err("Вы не авторизованы!".to_string()),
    };
    let appointments = appointment_controller::get_appointments_by_patient_outpatient_card_number(
        pool,
        patient.outpatient_card_number().unwrap(),
    )
    .await
    .map_err(|e| format!("Error: {}", e))?;
    Ok(appointments)
}

#[tauri::command]
pub async fn change_my_data(
    connection: State<'_, DbConnectionPool>,
    authorized_state: State<'_, Authorization>,
    patient: Patient,
) -> Result<(), String> {
    let mut patient = patient;
    let pool = &*connection.connection.lock().await;
    let auth = &*authorized_state.credentials.lock().await;
    let auth = match auth {
        Authorized::Patient(patient) => patient,
        _ => return Err("Вы не авторизованы!".to_string()),
    };
    patient.set_outpatient_card_number(*auth.outpatient_card_number());
    patient_controller::update_patient(pool, patient)
        .await
        .map_err(|e| format!("Error: {}", e))?;
    Ok(())
}
