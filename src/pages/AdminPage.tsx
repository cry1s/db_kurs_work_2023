/* eslint-disable no-case-declarations */
import React, { useEffect, useState } from 'react';
import './AdminPage.css';
import { Appointment, Cabinet, Diagnosis, Doctor, Medication, Patient, Specialty } from '../models';
import { invoke } from '@tauri-apps/api';

type SomeThing = Appointment | Cabinet | Diagnosis | Doctor | Medication | Patient | Specialty | object;
// Страница администратора, на которой отображаются таблицы БД:
// - Записи
// - Кабинеты
// - Врачи
// - Пациенты
// - Лекарства
// - Диагнозы
// - Специальности
const AdminPage = () => {
  const [selectedTable, setSelectedTable] = useState('appointments'); // Изначально выбрана таблица 'Записи'
  // cabinets, diagnoses, doctors, medications, patients, specialties
  const [formData, setFormData] = useState<SomeThing>({
    appointment_id: 0,
    appointment_time: '',
    complaints: '',
    doctor_id: 0,
    patient_outpatient_card_number: 0,
  } as Appointment); // Состояние для хранения данных формы

  useEffect(() => {
    // Запрос на получение данных таблицы
    invoke('get_all_from_table', { tableName: selectedTable })
      .then((result: any) => {
        console.log(result);
        // Установка данных таблицы в состояние
        switch (selectedTable) {
          case 'appointments':
            setAppointments(result as Appointment[]);
            break;
          case 'cabinets':
            setCabinets(result as Cabinet[]);
            break;
          case 'diagnoses':
            setDiagnoses(result as Diagnosis[]);
            break;
          case 'doctors':
            setDoctors(result as Doctor[]);
            break;
          case 'medications':
            setMedications(result as Medication[]);
            break;
          case 'patients':
            setPatients(result as Patient[]);
            break;
          case 'specialties':
            setSpecialties(result as Specialty[]);
            break;
          default:
            break;
        }
      })
      .catch((error: any) => {
        console.error(error);
      });
  }, [selectedTable]);

  const handleTableSelect = (table: string) => {
    setSelectedTable(table);
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData((prevFormData) => ({ ...prevFormData, [name]: value }));
  };

  // Ваши данные таблиц БД
  const [appointments, setAppointments] = useState<Appointment[]>([]);
  const [cabinets, setCabinets] = useState<Cabinet[]>([]);
  const [diagnoses, setDiagnoses] = useState<Diagnosis[]>([]);
  const [doctors, setDoctors] = useState<Doctor[]>([]);
  const [medications, setMedications] = useState<Medication[]>([]);
  const [patients, setPatients] = useState<Patient[]>([]);
  const [specialties, setSpecialties] = useState<Specialty[]>([]);

  const fillForm = (rowData: SomeThing) => {
    setFormData(rowData);
  };

  // Функция для добавления строки в таблицу
  const handleRowAdd = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    // Ваша логика для добавления строки в таблицу
    // Используйте данные из formData

    // Очистка данных формы после добавления
    setFormData({});
  };

  const handleRowEdit = (rowData: SomeThing) => {
    switch (selectedTable) {
      case 'appointments':
        const appointment = formData as Appointment;
        appointment.appointment_id = (rowData as Appointment).appointment_id;
        // parse all fields to their types
        appointment.appointment_id = Number(appointment.appointment_id);
        appointment.doctor_id = Number(appointment.doctor_id);
        appointment.patient_outpatient_card_number = Number(appointment.patient_outpatient_card_number);
        invoke('update_table_row' , {tableName: selectedTable, row: appointment}).then((result: any) => {
          console.log(result);
        }).catch((error: any) => {
          console.error(error);
        });
        break;
      case 'cabinets':
        const cabinet = formData as Cabinet;
        cabinet.cabinet_number = (rowData as Cabinet).cabinet_number;
        // parse all fields to their types
        cabinet.cabinet_number = Number(cabinet.cabinet_number);
        cabinet.working_doctor_id = Number(cabinet.working_doctor_id);

        invoke('update_table_row' , {tableName: selectedTable, row: cabinet}).then((result: any) => {
          console.log(result);
        }
        ).catch((error: any) => {
          console.error(error);
        }
        );
        break;
      case 'diagnoses':
        const diagnosis = formData as Diagnosis;
        diagnosis.diagnosis_id = (rowData as Diagnosis).diagnosis_id;
        // parse all fields to their types
        diagnosis.diagnosis_id = Number(diagnosis.diagnosis_id);
        diagnosis.hospitalization = Boolean(diagnosis.hospitalization);        
        invoke('update_table_row' , {tableName: selectedTable, row: diagnosis}).then((result: any) => {
          console.log(result);
        }).catch((error: any) => {
          console.error(error);
        }
        );
        break;
      case 'doctors':
        const doctor = formData as Doctor;
        // parse all number fields
        doctor.specialty_id = Number(doctor.specialty_id);
        doctor.doctor_id = (rowData as Doctor).doctor_id;
        console.log(doctor);
        invoke('update_table_row' , {tableName: selectedTable, row: doctor}).then((result: any) => {
          console.log(result);
        }
        ).catch((error: any) => {
          console.error(error);
        }
        );
        break;
      case 'medications':
        const medication = formData as Medication;
        medication.medication_id = (rowData as Medication).medication_id;
        // parse all fields to their types
        medication.medication_id = Number(medication.medication_id);
        invoke('update_table_row' , {tableName: selectedTable, row: medication}).then((result: any) => {
          console.log(result);
        }
        ).catch((error: any) => {
          console.error(error);
        }
        );
        break;
      case 'patients':
        const patient = formData as Patient;
        patient.outpatient_card_number = (rowData as Patient).outpatient_card_number;
        // parse all fields to their types
        patient.outpatient_card_number = Number(patient.outpatient_card_number);
        invoke('update_table_row' , {tableName: selectedTable, row: patient}).then((result: any) => {
          console.log(result);
        }
        ).catch((error: any) => {
          console.error(error);
        }
        );
        break;
      case 'specialties':
        const specialty = formData as Specialty;
        specialty.specialty_id = (rowData as Specialty).specialty_id;
        specialty.specialty_id = Number(specialty.specialty_id);
        specialty.education_duration = Number(specialty.education_duration);
        invoke('update_table_row' , {tableName: selectedTable, row: specialty}).then((result: any) => {
          console.log(result);
        }
        ).catch((error: any) => {
          console.error(error);
        }
        );
        break;
      default:
        break;
    }
    // Обновить таблицу
    invoke('get_all_from_table', {tableName: selectedTable}).then((result: any) => {
      switch (selectedTable) {
        case 'appointments':
          setAppointments(result as Appointment[]);
          break;
        case 'cabinets':
          setCabinets(result as Cabinet[]);
          break;
        case 'diagnoses':
          setDiagnoses(result as Diagnosis[]);
          break;
        case 'doctors':
          setDoctors(result as Doctor[]);
          break;
        case 'medications':
          setMedications(result as Medication[]);
          break;
        case 'patients':
          setPatients(result as Patient[]);
          break;
        case 'specialties':
          setSpecialties(result as Specialty[]);
          break;
        default:
          break;
      }
    }).catch((error: any) => {
      console.error(error);
    }
    );
  };

  const handleRowDelete = (rowData: SomeThing) => {
    let id: number;
    switch (selectedTable) {
      case 'appointments':
        id = Number((rowData as Appointment).appointment_id);
        break;
      case 'cabinets':
        id = Number((rowData as Cabinet).cabinet_number);
        break;
      case 'diagnoses':
        id = Number((rowData as Diagnosis).diagnosis_id);
        break;
      case 'doctors':
        id = Number((rowData as Doctor).doctor_id);
        break;
      case 'medications':
        id = Number((rowData as Medication).medication_id);
        break;
      case 'patients':
        id = Number((rowData as Patient).outpatient_card_number);
        break;
      case 'specialties':
        id = Number((rowData as Specialty).specialty_id);
        break;
      default:
        id = 0;
        break;
    }
    if (id === 0) {
      return;
    }
    invoke('delete_table_row', {tableName: selectedTable, id}).then((result: any) => {
      console.log(result);
    }
    ).catch((error: any) => {
      console.error(error);
    }
    );
    // Обновить таблицу
    invoke('get_all_from_table', {tableName: selectedTable}).then((result: any) => {
      switch (selectedTable) {
        case 'appointments':
          setAppointments(result as Appointment[]);
          break;
        case 'cabinets':
          setCabinets(result as Cabinet[]);
          break;
        case 'diagnoses':
          setDiagnoses(result as Diagnosis[]);
          break;
        case 'doctors':
          setDoctors(result as Doctor[]);
          break;
        case 'medications':
          setMedications(result as Medication[]);
          break;
        case 'patients':
          setPatients(result as Patient[]);
          break;
        case 'specialties':
          setSpecialties(result as Specialty[]);
          break;
        default:
          break;
      }
    }).catch((error: any) => {
      console.error(error);
    }
    );
  };

  // Генерация таблицы в зависимости от выбранной таблицы БД
  const renderTable = () => {
    switch (selectedTable) {
      case 'appointments':
        return (
          <table>
            {/* Используя поля из интерфейса */}
            <thead>
              <tr>
                <th>ID</th>
                <th>ID доктора</th>
                <th>ID пациента</th>
                <th>Дата и время</th>
                <th>Жалобы</th>
              </tr>
            </thead>
            {/* Отобразите данные из appointments */}
            <tbody>
            {appointments.map((appointment, index) => (
              <tr key={index} onClick={() => fillForm(appointment)}>
                <td>{appointment.appointment_id}</td>
                <td>{appointment.doctor_id}</td>
                <td>{appointment.patient_outpatient_card_number}</td>
                <td>{appointment.appointment_time}</td>
                <td>{appointment.complaints}</td>
                <td>
                  <button onClick={() => handleRowEdit(appointment)}>Редактировать</button>
                  <button onClick={() => handleRowDelete(appointment)}>Удалить</button>
                </td>
              </tr>
            ))}
            </tbody>
          </table>
        );
      case 'cabinets':
        return (
          <table>
            {/* Таблица для таблицы 'Кабинеты' */}
            <thead>
              <tr>
                <th>Номер кабинета</th>
                <th>ID Врача</th>
                <th>Телефон</th>
                <th>Рабочие часы</th>
              </tr>
            </thead>
            {/* Отобразите данные из cabinets */}
            <tbody>
            {cabinets.map((cabinet, index) => (
              <tr key={index} onClick={() => fillForm(cabinet)}>
                <td>{cabinet.cabinet_number}</td>
                <td>{cabinet.working_doctor_id}</td>
                <td>{cabinet.phone}</td>
                <td>{cabinet.working_hours}</td>
                <td>
                  <button onClick={() => handleRowEdit(cabinet)}>Редактировать</button>
                  <button onClick={() => handleRowDelete(cabinet)}>Удалить</button>
                </td>
              </tr>
            ))}
            </tbody>
          </table>
        );
      case 'diagnoses':
        return (
          <table>
            {/* Таблица для таблицы 'Диагнозы' */}
            <thead>
              <tr>
                <th>ID</th>
                <th>Название</th>
                <th>Госпитализация</th>
              </tr>
            </thead>
            {/* Отобразите данные из diagnoses */}
            <tbody>
            {diagnoses.map((diagnosis, index) => (
              <tr key={index} onClick={() => fillForm(diagnosis)}>
                <td>{diagnosis.diagnosis_id}</td>
                <td>{diagnosis.disease_name}</td>
                <td>{diagnosis.hospitalization}</td>
                <td>
                  <button onClick={() => handleRowEdit(diagnosis)}>Редактировать</button>
                  <button onClick={() => handleRowDelete(diagnosis)}>Удалить</button>
                </td>
              </tr>
            ))}
            </tbody>
          </table>
        );
      case 'doctors':
        return (
          <table>
            {/* Таблица для таблицы 'Врачи' */}
            <thead>
              <tr>
                <th>ID</th>
                <th>ФИО</th>
                <th>ID Специальности</th>
                <th>Квалификация</th>
                <th>Пароль</th>
              </tr>
            </thead>
            {/* Отобразите данные из doctors */}
            <tbody>
            {doctors.map((doctor, index) => (
              <tr key={index} onClick={() => fillForm(doctor)}>
                <td>{doctor.doctor_id}</td>
                <td>{doctor.full_name}</td>
                <td>{doctor.specialty_id}</td>
                <td>{doctor.qualification}</td>
                <td>{doctor.password || ''}</td>
                <td>
                  <button onClick={() => handleRowEdit(doctor)}>Редактировать</button>
                  <button onClick={() => handleRowDelete(doctor)}>Удалить</button>
                </td>
              </tr>
            ))}
            </tbody>
          </table>
        );
      case 'medications':
        return (
          <table>
            {/* Таблица для таблицы 'Лекарства' */}
            <thead>
              <tr>
                <th>ID</th>
                <th>Название</th>
                <th>Курс</th>
                <th>Дозировка</th>
              </tr>
            </thead>
            {/* Отобразите данные из medications */}
            <tbody>
            {medications.map((medication, index) => (
              <tr key={index} onClick={() => fillForm(medication)}>
                <td>{medication.medication_id}</td>
                <td>{medication.name}</td>
                <td>{medication.treatment_plan}</td>
                <td>{medication.dosage}</td>
                <td>
                  <button onClick={() => handleRowEdit(medication)}>Редактировать</button>
                  <button onClick={() => handleRowDelete(medication)}>Удалить</button>
                </td>
              </tr>
            ))}
            </tbody>
          </table>
        );
      case 'patients':
        return (
          <table>
            {/* Таблица для таблицы 'Пациенты' */}
            <thead>
              <tr>
                <th>Номер амбулаторной карты</th>
                <th>ФИО</th>
                <th>Дата рождения</th>
                <th>Номер полиса</th>
                <th>Номер СНИЛСа</th>
                <th>Пол</th>
                <th>Контакты</th>
                <th>Адрес</th>
              </tr>
            </thead>
            {/* Отобразите данные из patients */}
            <tbody>
            {patients.map((patient, index) => (
              <tr key={index}>
                <td>{patient.outpatient_card_number}</td>
                <td>{patient.full_name}</td>
                <td>{patient.date_of_birth}</td>
                <td>{patient.insurance_number}</td>
                <td>{patient.snils_number}</td>
                <td>{patient.gender}</td>
                <td>{patient.contacts}</td>
                <td>{patient.address}</td>
                <td>
                  <button onClick={() => handleRowEdit(patient)}>Редактировать</button>
                  <button onClick={() => handleRowDelete(patient)}>Удалить</button>
                </td>
              </tr>
            ))}
            </tbody>
          </table>
        );
      case 'specialties':
        return (
          <table>
            {/* Таблица для таблицы 'Специальности' */}
            <thead>
              <tr>
                <th>ID</th>
                <th>Название</th>
                <th>Время обучения</th>
              </tr>
            </thead>
            {/* Отобразите данные из specialties */}
            <tbody>
            {specialties.map((specialty, index) => (
              <tr key={index}>
                <td>{specialty.specialty_id}</td>
                <td>{specialty.name}</td>
                <td>{specialty.education_duration}</td>
                <td>
                  <button onClick={() => handleRowEdit(specialty)}>Редактировать</button>
                  <button onClick={() => handleRowDelete(specialty)}>Удалить</button>
                </td>
              </tr>
            ))}
            </tbody>
          </table>
        );
      default:
        return null;
    }
  };

  const renderAddEditFields = () => {
    switch (selectedTable) {
      case 'appointments':
      return <div className="add-edit-row">
      {/* Форма для добавления строк */}
      <form onSubmit={handleRowAdd}>
        {/* Поля для ввода данных */}
        <input
          type="number"
          value={(formData as Appointment).doctor_id || ''}
          name="doctor_id"
          placeholder="ID врача"
          onChange={handleInputChange}
        />
        <input
          type="number"
          value={(formData as Appointment).patient_outpatient_card_number || ''}
          name="patient_outpatient_card_number"
          placeholder="Номер амбулаторной карты"
          onChange={handleInputChange}
        />
        <input
          type='datetime-local'
          value={(formData as Appointment).appointment_time || ''}
          name="appointment_time"
          placeholder="Дата"
          onChange={handleInputChange}
        />
        <input
          type="text"
          value={(formData as Appointment).complaints || ''}
          name="complaints"
          placeholder="Жалобы"
          onChange={handleInputChange}
        />
        <button type="submit">Добавить</button>
      </form>
    </div>
      case 'cabinets':
      return <div className="add-edit-row">
      {/* Форма для добавления строк */}
      <form onSubmit={handleRowAdd}>
        {/* Поля для ввода данных */}
        <input
          type="number"
          value={(formData as Cabinet).cabinet_number || ''}
          name="cabinet_number"
          placeholder="Номер кабинета"
          onChange={handleInputChange}
        />
        <input
          type="text"
          value={(formData as Cabinet).phone || ''}
          name="phone"
          placeholder="Телефон"
          onChange={handleInputChange}
        />
        <input
          type="number"
          value={(formData as Cabinet).working_doctor_id || ''}
          name="working_doctor_id"
          placeholder="ID врача"
          onChange={handleInputChange}
        />
        <button type="submit">Добавить</button>
      </form>
    </div>
      case 'diagnoses':
      return <div className="add-edit-row">
      {/* Форма для добавления строк */}
      <form onSubmit={handleRowAdd}>
        {/* Поля для ввода данных */}
        <input
          type="number"
          value={(formData as Diagnosis).diagnosis_id || ''}
          name="diagnosis_id"
          placeholder="ID диагноза"
          onChange={handleInputChange}
        />
        <input
          type="text"
          value={(formData as Diagnosis).disease_name || ''}
          name="name"
          placeholder="Название"
          onChange={handleInputChange}
        />
        <label htmlFor="hospitalization">Госпитализация</label>
        <input
          type="checkbox"
          checked={(formData as Diagnosis).hospitalization || false}
          name="hospitalization"
          placeholder="Госпитализация"
          onChange={handleInputChange}
        />
        <button type="submit">Добавить</button>
      </form>
    </div>
      case 'doctors':
      return <div className="add-edit-row">
      {/* Форма для добавления строк */}
      <form onSubmit={handleRowAdd}>
        {/* Поля для ввода данных */}
        <input
          type="number"
          value={(formData as Doctor).doctor_id || ''}
          name="doctor_id"
          placeholder="ID врача"
          onChange={handleInputChange}
        />
        <input
          type="text"
          value={(formData as Doctor).full_name || ''}
          name="full_name"
          placeholder="ФИО"
          onChange={handleInputChange}
        />
        <input
          type='number'
          value={(formData as Doctor).specialty_id || 0}
          name="specialty_id"
          placeholder="ID специальности"
          onChange={handleInputChange}
        />
        <input
          type="text"
          value={(formData as Doctor).qualification || ''}
          name="cabinet_number"
          placeholder="Номер кабинета"
          onChange={handleInputChange}
        />
        </form>
    </div>
      case 'medications':
      return <div className="add-edit-row">
      {/* Форма для добавления строк */}
      <form onSubmit={handleRowAdd}>
        {/* Поля для ввода данных */}
        <input
          type="number"
          value={(formData as Medication).medication_id || ''}
          name="medication_id"
          placeholder="ID лекарства"
          onChange={handleInputChange}
        />
        <input
          type="text"
          value={(formData as Medication).name || ''} 
          name="name"
          placeholder="Название"
          onChange={handleInputChange}
        />
        <input
          type="text"
          value={(formData as Medication).treatment_plan || ''}
          name="treatment_plan" 
          placeholder="План лечения"
          onChange={handleInputChange}
        />
        <input
          type="text"
          value={(formData as Medication).dosage || ''}
          name="dosage"
          placeholder="Дозировка"
          onChange={handleInputChange}
        />
        <button type="submit">Добавить</button>
      </form>
    </div>
      case 'patients':
      return <div className="add-edit-row">
      {/* Форма для добавления строк */}
      <form onSubmit={handleRowAdd}>
        {/* Поля для ввода данных */}
        <input
          type="text"
          value={(formData as Patient).full_name || ''}
          name="full_name"
          placeholder="ФИО"
          onChange={handleInputChange}
        />
        <input
          type='datetime-local'
          value={(formData as Patient).date_of_birth || ''}
          name="date_of_birth"
          placeholder="Дата рождения"
          onChange={handleInputChange}
        />
        <input
          type="text"
          value={(formData as Patient).address || ''}
          name="address"
          placeholder="Адрес"
          onChange={handleInputChange}
        />
        <input
          type="text"
          value={(formData as Patient).contacts || ''}
          name="contacts"
          placeholder="Контакты"
          onChange={handleInputChange}
        />
        <input
          type="text"
          value={(formData as Patient).insurance_number || ''}
          name="insurance_policy_number"
          placeholder="Номер страхового полиса"
          onChange={handleInputChange}
        />
        <input
          type="text"
          value={(formData as Patient).snils_number || ''}
          name="snils_number"
          placeholder="Номер СНИЛС"
          onChange={handleInputChange}
        />
        <button type="submit">Добавить</button>
      </form>
    </div>
      case 'specialties':
      return <div className="add-edit-row">
      {/* Форма для добавления строк */}
      <form onSubmit={handleRowAdd}>
        {/* Поля для ввода данных */}
        <input
          type="text"
          value={(formData as Specialty).name || ''}
          name="name"
          placeholder="Название"
          onChange={handleInputChange}
        />
        <input
          type="number"
          value={(formData as Specialty).education_duration || ''}
          name="education_duration"
          placeholder="Длительность обучения"
          onChange={handleInputChange}
        />
        <button type="submit">Добавить</button>
      </form>
    </div>
    default:
      return null;
    }
  };

  return (
    <div>
      <div className="table-selection">
        {/* cabinets, diagnoses, doctors, medications, patients, specialties */}
        <button onClick={() => handleTableSelect('appointments')}>Записи</button>
        <button onClick={() => handleTableSelect('cabinets')}>Кабинеты</button>
        <button onClick={() => handleTableSelect('diagnoses')}>Диагнозы</button>
        <button onClick={() => handleTableSelect('doctors')}>Врачи</button>
        <button onClick={() => handleTableSelect('medications')}>Лекарства</button>
        <button onClick={() => handleTableSelect('patients')}>Пациенты</button>
        <button onClick={() => handleTableSelect('specialties')}>Специальности</button>
        {/* Кнопки для остальных таблиц */}
      </div>

      {renderTable()}

      {renderAddEditFields()}
    </div>
  );
};

export default AdminPage;