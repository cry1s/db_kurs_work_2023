import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api'
import { useNavigate } from 'react-router-dom';
import './Login.css';

function validateSnils(snils: string | number, error: {
  code: number;
  message: string;
}) {
	let result = false;
	if (typeof snils === 'number') {
		snils = snils.toString();
	} else if (typeof snils !== 'string') {
		snils = '';
	}
	if (!snils.length) {
		error.code = 1;
		error.message = 'СНИЛС пуст';
	} else if (/[^0-9]/.test(snils)) {
		error.code = 2;
		error.message = 'СНИЛС может состоять только из цифр';
	} else if (snils.length !== 11) {
		error.code = 3;
		error.message = 'СНИЛС может состоять только из 11 цифр';
	} else {
		let sum = 0;
		for (let i = 0; i < 9; i++) {
			sum += parseInt(snils[i]) * (9 - i);
		}
		let checkDigit = 0;
		if (sum < 100) {
			checkDigit = sum;
		} else if (sum > 101) {
			checkDigit = sum % 101;
			if (checkDigit === 100) {
				checkDigit = 0;
			}
		}
		if (checkDigit === parseInt(snils.slice(-2))) {
			result = true;
		} else {
			error.code = 4;
			error.message = 'Неправильное контрольное число';
		}
	}
	return result;
}

const Login = () => {
    const [role, setRole] = useState('');
    const [password, setPassword] = useState('');
    const [snils, setSnils] = useState('');
    const [snlError, setSnlError] = useState({ code: 0, message: '' } as { code: number; message: string; });
    const [doctorName, setDoctorName] = useState('');
    const [availableDoctors, setAvailableDoctors] = useState([]);

    const handleRoleChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
        setRole(event.target.value);
    };

    const handleDoctorNameChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
        setDoctorName(event.target.value);
    };

    const handlePasswordChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setPassword(event.target.value);
    };

    const handleSnilsChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setSnils(event.target.value);
        const error = { code: 0, message: '' };
        if (validateSnils(event.target.value, error)) {
            setSnlError({ code: 0, message: '' });
        } else {
            setSnlError(error);
        }
    };
    const navigate = useNavigate();
    useEffect(() => {
        invoke('get_current_authorization')
            .then((auth: any) => {
                console.log(auth);
            }).catch((error: any) => {
                console.error(error);
            });
        invoke('get_all_doctors')
            .then((doctors: any) => {
                console.log(doctors);
                setAvailableDoctors(doctors);
            })
    }, []);


    const handleSubmit = (event: React.FormEvent) => {
        event.preventDefault();

        if (role === 'patient') {
            // Обработка входа для пациента
            console.log('Вход для пациента:', snils);
            invoke('try_login_patient', { insuranceNumberOrNumberSnils: snils })
                .then((result: any) => {
                    console.log(result);
                    // go to patient page
                    navigate('/patient');
                }).catch((error: any) => {
                    console.error(error);
                });
            // Добавьте код для обработки входа пациента
        } else if (role === 'doctor') {
            // Обработка входа для врача
            console.log('Вход для врача:', doctorName, password);
            // Добавьте код для обработки входа врача
        } else if (role === 'admin') {
            // Обработка входа для администратора
            console.log('Вход для администратора:', password);
            invoke('try_login_admin', { password: password })
                .then((result: any) => {
                    console.log(result);
                    // go to admin page
                    navigate('/admin');
                }).catch((error: any) => {
                    console.error(error);
                });
        } else {
            // Некорректная роль
            console.log('Выберите роль');
        }
    };

    return (
        <div className='login-container'>
            <h1 className="login-heading">Добро пожаловать!</h1>
            <form onSubmit={handleSubmit} className="login-form">
                <label>
                    Роль:
                    <select value={role} onChange={handleRoleChange}>
                        <option value="" disabled>Выберите роль</option>
                        <option value="patient">Пациент</option>
                        <option value="doctor">Врач</option>
                        <option value="admin">Администратор</option>
                    </select>
                </label>
                {role === 'patient' && (
                    <label>
                        СНИЛС:
                        <input type="text" value={snils} onChange={handleSnilsChange} />
                        {snlError.code !== 0 && (
                            <div className="error-message">{snlError.message}</div>
                        )}
                    </label>
                )}
                {role === 'doctor' && (
                    <div>
                        <label>
                            Имя:
                            <select value={doctorName} onChange={handleDoctorNameChange}>
                                <option value="">Выберите имя</option>
                                {availableDoctors.map((doctor: any) => (
                                    <option value={doctor.doctor_id} key={doctor.doctor_id}>{doctor.full_name} квалификация {doctor.qualification}</option>
                                ))}
                            </select>
                        </label>
                        <label>
                            Пароль:
                            <input type="password" value={password} onChange={handlePasswordChange} />
                        </label>
                    </div>
                )}
                {role === 'admin' && (
                    <div>
                        <label>
                            Пароль:
                            <input type="password" value={password} onChange={handlePasswordChange} />
                        </label>
                    </div>
                )}
                {role && (
                    <button type="submit">Войти</button>
                )}
            </form>
        </div>
    );
};

export default Login;
