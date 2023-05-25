-- Add migration script here
create table public.Patient
(
    outpatient_card_number serial
        primary key,
    full_name             varchar(255) not null,
    insurance_number      varchar(255) not null,
    snils_number          varchar(255) not null,
    contacts              varchar(255) not null,
    address               varchar(255) not null,
    gender                varchar(255) not null,
    date_of_birth         date not null
);

create table public.Diagnosis
(
    diagnosis_id     serial
        primary key,
    disease_name     varchar(255) not null,
    hospitalization  boolean not null
);

create table public.Medication
(
    medication_id  serial
        primary key,
    name           varchar(255) not null,
    treatment_plan varchar(255) not null,
    dosage         varchar(255) not null
);

create table public.Specialty
(
    specialty_id        serial
        primary key,
    name                varchar(255) not null,
    education_duration integer not null
);

create table public.Doctor
(
    doctor_id           serial
        primary key,
    full_name           varchar(255) not null,
    specialty_id        integer default 1 not null
        references public.Specialty
            on update cascade on delete set default,
    qualification      varchar(255) not null
);

create table public.Cabinet
(
    working_doctor_id  integer
                                    references public.Doctor
                                        on update cascade on delete set null,
    phone              varchar(255) not null,
    working_hours      varchar(255) not null,
    cabinet_number     integer not null
        primary key
);

create table public.Appointment
(
    appointment_id                    serial
        primary key,
    doctor_id                         integer
        references public.Doctor
            on update cascade on delete RESTRICT,
    patient_outpatient_card_number     integer
            references public.Patient
                on update cascade on delete RESTRICT,
    appointment_time                  timestamp not null,
    complaints                        text not null
);

create table public.Appointment_Medication
(
    appointment_id  integer not null
        references public.Appointment
            on update cascade on delete cascade,
    medication_id   integer not null
        references public.Medication
            on update cascade on delete cascade,
    primary key (appointment_id, medication_id)
);

create table public.Appointment_Diagnosis
(
    appointment_id  integer not null
        references public.Appointment
            on update cascade on delete cascade,
    diagnosis_id    integer not null
        references public.Diagnosis
            on update cascade on delete cascade,
    primary key (appointment_id, diagnosis_id)
);

create view public.Record (appointment_id, doctor_id, patient_outpatient_card_number, appointment_time, complaints) as
SELECT Appointment.appointment_id,
       Appointment.doctor_id,
       Appointment.patient_outpatient_card_number,
       Appointment.appointment_time,
       Appointment.complaints
FROM Appointment
WHERE Appointment.appointment_time > now()
with cascaded check option;