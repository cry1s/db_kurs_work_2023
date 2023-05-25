-- Add migration script here
alter table public.appointment
    alter column doctor_id set not null;

alter table public.appointment
    alter column patient_outpatient_card_number set not null;