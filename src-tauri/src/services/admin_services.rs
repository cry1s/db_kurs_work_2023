use serde::{Serialize, Deserialize};
use tauri::State;
use tokio::sync::Mutex;

use crate::{db::{connection::DbConnectionPool, *, doctor_controller::DoctorEntityWithPassword}, model::doctor::DoctorEntity};


#[tauri::command]
pub async fn get_all_from_table(
    table_name: String, // appointments, cabinets, diagnoses, medications, patients, specialties
    connection: State<'_, DbConnectionPool>,
) -> Result<Vec<serde_json::Value>, String> {
    let pool = &*connection.connection.lock().await;
    let result = match table_name.as_str() {
        "appointments" => appointment_controller::get_all_appointments(pool).await.map(|appointments| appointments.into_iter().map(|appointment| serde_json::to_value(appointment).unwrap()).collect()),
        "cabinets" => cabinet_controller::get_all_cabinets(pool).await.map(|cabinets| cabinets.into_iter().map(|cabinet| serde_json::to_value(cabinet).unwrap()).collect()),
        "diagnoses" => diagnosis_controller::get_all_diagnoses(pool).await.map(|diagnoses| diagnoses.into_iter().map(|diagnosis| serde_json::to_value(diagnosis).unwrap()).collect()),
        "medications" => medication_controller::get_all_medications(pool).await.map(|medications| medications.into_iter().map(|medication| serde_json::to_value(medication).unwrap()).collect()),
        "patients" => patient_controller::get_all_patients(pool).await.map(|patients| patients.into_iter().map(|patient| serde_json::to_value(patient).unwrap()).collect()),
        "specialties" => speciality_controller::get_all_specialities(pool).await.map(|specialties| specialties.into_iter().map(|specialty| serde_json::to_value(specialty).unwrap()).collect()),
        "doctors" => doctor_controller::get_all_doctors_with_passwords(pool).await.map(|doctors| doctors.into_iter().map(|doctor| serde_json::to_value(doctor).unwrap()).collect()),
        _ => Ok(vec![]),
    }.map_err(|e| format!("Error: {}", e))?;
    Ok(result)
}

#[tauri::command]
pub async fn update_table_row(
    table_name: String, // appointments, cabinets, diagnoses, medications, patients, specialties
    row: serde_json::Value,
    connection: State<'_, DbConnectionPool>,
) -> Result<serde_json::Value, String> {
    let pool = &*connection.connection.lock().await;
    let result = match table_name.as_str() {
        "appointments" => appointment_controller::update_appointment(pool, serde_json::from_value(row).unwrap()).await.map(|appointment| serde_json::to_value(appointment).unwrap()),
        "cabinets" => cabinet_controller::update_cabinet(pool, serde_json::from_value(row).unwrap()).await.map(|cabinet| serde_json::to_value(cabinet).unwrap()),
        "diagnoses" => diagnosis_controller::update_diagnosis(pool, serde_json::from_value(row).unwrap()).await.map(|diagnosis| serde_json::to_value(diagnosis).unwrap()),
        "medications" => medication_controller::update_medication(pool, serde_json::from_value(row).unwrap()).await.map(|medication| serde_json::to_value(medication).unwrap()),
        "patients" => patient_controller::update_patient(pool, serde_json::from_value(row).unwrap()).await.map(|patient| serde_json::to_value(patient).unwrap()),
        "specialties" => speciality_controller::update_speciality(pool, serde_json::from_value(row).unwrap()).await.map(|specialty| serde_json::to_value(specialty).unwrap()),
        "doctors" => doctor_controller::update_doctor(pool, serde_json::from_value(row).unwrap()).await.map(|doctor| serde_json::to_value(doctor).unwrap()),
        _ => Ok(serde_json::Value::Null),
    }.map_err(|e| format!("Error: {}", e))?;
    Ok(result)
}

#[tauri::command]
pub async fn delete_table_row(
    table_name: String, // appointments, cabinets, diagnoses, medications, patients, specialties
    id: i32,
    connection: State<'_, DbConnectionPool>,
) -> Result<serde_json::Value, String> {
    let pool = &*connection.connection.lock().await;
    let result = match table_name.as_str() {
        "appointments" => appointment_controller::delete_appointment(pool, id).await.map(|appointment| serde_json::to_value(appointment).unwrap()),
        "cabinets" => cabinet_controller::delete_cabinet(pool, id).await.map(|cabinet| serde_json::to_value(cabinet).unwrap()),
        "diagnoses" => diagnosis_controller::delete_diagnosis(pool, id).await.map(|diagnosis| serde_json::to_value(diagnosis).unwrap()),
        "medications" => medication_controller::delete_medication(pool, id).await.map(|medication| serde_json::to_value(medication).unwrap()),
        "patients" => patient_controller::delete_patient(pool, id).await.map(|patient| serde_json::to_value(patient).unwrap()),
        "specialties" => speciality_controller::delete_speciality(pool, id).await.map(|specialty| serde_json::to_value(specialty).unwrap()),
        "doctors" => doctor_controller::delete_doctor(pool, id).await.map(|doctor| serde_json::to_value(doctor).unwrap()),
        _ => Ok(serde_json::Value::Null),
    }.map_err(|e| format!("Error: {}", e))?;
    Ok(result)
}

#[tauri::command]
pub async fn add_table_row(
    table_name: String, // appointments, cabinets, diagnoses, medications, patients, specialties
    row: serde_json::Value,
    connection: State<'_, DbConnectionPool>,
) -> Result<serde_json::Value, String> {
    let pool = &*connection.connection.lock().await;
    let result = match table_name.as_str() {
        "appointments"  => appointment_controller::add_appointment(pool, serde_json::from_value(row).unwrap()).await.map(|appointment| serde_json::to_value(appointment).unwrap()),
        "cabinets"      => cabinet_controller::add_cabinet(pool, serde_json::from_value(row).unwrap()).await.map(|cabinet| serde_json::to_value(cabinet).unwrap()),
        "diagnoses"     => diagnosis_controller::add_diagnosis(pool, serde_json::from_value(row).unwrap()).await.map(|diagnosis| serde_json::to_value(diagnosis).unwrap()),
        "medications"   => medication_controller::add_medication(pool, serde_json::from_value(row).unwrap()).await.map(|medication| serde_json::to_value(medication).unwrap()),
        "patients"      => patient_controller::add_patient(pool, serde_json::from_value(row).unwrap()).await.map(|patient| serde_json::to_value(patient).unwrap()),
        "specialties"   => speciality_controller::add_speciality(pool, serde_json::from_value(row).unwrap()).await.map(|specialty| serde_json::to_value(specialty).unwrap()),
        "doctors"       => doctor_controller::add_doctor(pool, serde_json::from_value(row).unwrap()).await.map(|doctor| serde_json::to_value(doctor).unwrap()),
        _ => Ok(serde_json::Value::Null),
    }.map_err(|e| format!("Error: {}", e))?;
    Ok(result)
}
