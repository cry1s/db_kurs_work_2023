import React from 'react';
import { Link } from 'react-router-dom';

const PatientPage = () => {
  // Здесь можно добавить логику для получения текущих записей к врачу

  // Пример данных таблицы текущих записей
  const appointments = [
    { doctorName: 'Иванов', specialization: 'Терапевт', time: '10:00', roomNumber: '101' },
    { doctorName: 'Петров', specialization: 'Офтальмолог', time: '14:30', roomNumber: '202' },
    { doctorName: 'Сидоров', specialization: 'Стоматолог', time: '16:15', roomNumber: '305' },
  ];

  return (
    <div>
      <h1>Текущие записи</h1>
      <table>
        <thead>
          <tr>
            <th>Имя доктора</th>
            <th>Специализация</th>
            <th>Время</th>
            <th>Номер кабинета</th>
          </tr>
        </thead>
        <tbody>
          {appointments.map((appointment, index) => (
            <tr key={index}>
              <td>{appointment.doctorName}</td>
              <td>{appointment.specialization}</td>
              <td>{appointment.time}</td>
              <td>{appointment.roomNumber}</td>
            </tr>
          ))}
        </tbody>
      </table>
      <div>
        <Link to="/create-appointment">Создать новую запись</Link>
        <Link to="/appointment-history">История записей</Link>
        <Link to="/edit-profile">Изменить личные данные</Link>
      </div>
    </div>
  );
};

export default PatientPage;
