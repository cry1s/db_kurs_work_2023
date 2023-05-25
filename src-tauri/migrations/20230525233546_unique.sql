-- Add migration script here
alter table patient
    add constraint patient_pk
        unique (snils_number);

alter table patient
    add constraint patient_pk2
        unique (insurance_number);

