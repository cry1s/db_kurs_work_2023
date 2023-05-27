// Файл с типами данных для моделей
// --------------------------------------------------
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
// --------------------------------------------------

export interface Appointment {
    appointment_id: number | null;
    doctor_id: number;
    patient_outpatient_card_number: number;
    appointment_time: string;
    complaints: string;
}

// --------------------------------------------------
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
// --------------------------------------------------

export interface Cabinet {
    working_doctor_id: number | null;
    phone: string;
    working_hours: string;
    cabinet_number: number;
}

// --------------------------------------------------
// create table public.diagnosis
// (
//     diagnosis_id    serial
//         primary key,
//     disease_name    varchar(255) not null,
//     hospitalization boolean      not null
// );
// --------------------------------------------------

export interface Diagnosis {
    diagnosis_id: number | null;
    disease_name: string;
    hospitalization: boolean;
}

// --------------------------------------------------
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
// --------------------------------------------------

export interface Doctor {
    doctor_id: number | null;
    full_name: string;
    specialty_id: number;
    qualification: string;
    password: string | undefined;
}

// --------------------------------------------------
// create table public.medication
// (
//     medication_id  serial
//         primary key,
//     name           varchar(255) not null,
//     treatment_plan varchar(255) not null,
//     dosage         varchar(255) not null
// );
// --------------------------------------------------

export interface Medication {
    medication_id: number | null;
    name: string;
    treatment_plan: string;
    dosage: string;
}

// --------------------------------------------------
// create table public.patient
// (
//     outpatient_card_number serial
//         primary key,
//     full_name              varchar(255) not null,
//     insurance_number       varchar(255) not null,
//     snils_number           varchar(255) not null,
//     contacts               varchar(255) not null,
//     address                varchar(255) not null,
//     gender                 varchar(255) not null,
//     date_of_birth          date         not null
// );
// --------------------------------------------------

export interface Patient {
    outpatient_card_number: number | null;
    full_name: string;
    insurance_number: string;
    snils_number: string;
    contacts: string;
    address: string;
    gender: string;
    date_of_birth: string;
}

// --------------------------------------------------
// create table public.Specialty
// (
//     specialty_id        serial
//         primary key,
//     name                varchar(255),
//     education_duration integer
// );
// --------------------------------------------------

export interface Specialty {
    specialty_id: number | null;
    name: string;
    education_duration: number;
}