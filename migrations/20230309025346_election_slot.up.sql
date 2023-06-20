-- Add up migration script here

create table if not exists election_slot (
    id serial primary key,
    slot_id integer not null,
    election_id integer not null,
    created_at timestamp(0) without time zone not null default now(),
    updated_at timestamp(0) without time zone not null default now()
);
