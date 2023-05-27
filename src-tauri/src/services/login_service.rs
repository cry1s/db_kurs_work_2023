use serde::{Serialize, Deserialize};
use tauri::State;
use tokio::sync::Mutex;

use crate::db::{connection::DbConnectionPool, patient_controller, doctor_controller};
use crate::model::doctor::Doctor;
use crate::model::patient::Patient;

#[tauri::command]
pub fn create_patient(
    full_name: String,
    insurance_number: String,
    snils_number: String,
    contacts: String,
    address: String,
    gender: String,
    date_of_birth: String,
) -> Patient {
    let date_of_birth = date_of_birth.parse::<chrono::NaiveDate>().unwrap();
    Patient::new(full_name, insurance_number, snils_number, contacts, address, gender, date_of_birth)
}

#[tauri::command]
pub async fn try_login_patient(
    connection: State<'_, DbConnectionPool>,
    authorized_state: State<'_, Authorization>,
    insurance_number_or_number_snils: String,
) -> Result<Patient, String> {
    let pool = &*connection.connection.lock().await;
    let patient = patient_controller::get_patient_by_insurance_number_or_number_snils(pool, &insurance_number_or_number_snils)
        .await
        .map_err(|_| format!("CНИЛС не найден"))?;
    *authorized_state.credentials.lock().await = Authorized::Patient(patient.clone());
    Ok(patient)
}

#[tauri::command]
pub async fn get_all_doctors(
    connection: State<'_, DbConnectionPool>,
) -> Result<Vec<Doctor>, String> {
    let pool = &*connection.connection.lock().await;
    let doctors = doctor_controller::get_all_doctors(pool)
        .await
        .map_err(|e| format!("Error: {}", e))?;
    Ok(doctors)
}

#[tauri::command]
pub async fn try_login_doctor(
    doctor_id: i32,
    password: String,
    connection: State<'_, DbConnectionPool>,
    authorized_state: State<'_, Authorization>,
) -> Result<Doctor, String> {
    let password = password.parse::<i32>().map_err(|_| format!("Неправильный пароль!"))?;
    let pool = &*connection.connection.lock().await;
    let doctor = doctor_controller::try_login_doctor(pool, doctor_id, password).await.map_err(|_| format!("Неправильный пароль!"))?;
    *authorized_state.credentials.lock().await = Authorized::Doctor(doctor.clone());
    Ok(doctor)
}

#[tauri::command]
pub async fn try_login_admin(
    password: String,
    authorized_state: State<'_, Authorization>,
) -> Result<(), String> {
    let password = password.parse::<i32>().map_err(|_| format!("Неправильный пароль!"))?;
    if password == 1234 {
        *authorized_state.credentials.lock().await = Authorized::Admin;
        Ok(())
    } else {
        Err(format!("Неправильный пароль!"))
    }
}

#[tauri::command]
pub async fn try_register_patient(
    connection: State<'_, DbConnectionPool>,
    authorized_state: State<'_, Authorization>,
    patient: Patient,
) -> Result<(), String> {
    let pool = &*connection.connection.lock().await;
    let patient = patient_controller::try_register_patient(pool, patient).await?;
    *authorized_state.credentials.lock().await = Authorized::Patient(patient.clone());
    Ok(())
}

#[tauri::command]
pub async fn get_current_authorization(
    authorized_state: State<'_, Authorization>,
) -> Result<Authorized, String> {
    let auth = &*authorized_state.credentials.lock().await;
    Ok(auth.clone())
}

pub struct Authorization {
    pub credentials: Mutex<Authorized>,
}


#[derive(Clone, Serialize, Deserialize)]
pub enum Authorized {
    None,
    Patient(Patient),
    Doctor(Doctor),
    Admin,
}