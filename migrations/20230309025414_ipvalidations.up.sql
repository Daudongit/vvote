-- Add up migration script here

create table if not exists ipvalidations (
    id serial primary key,
    ip character varying(191) not null,
    election_id integer not null,
    created_at timestamp(0) without time zone not null default now(),
    updated_at timestamp(0) without time zone not null default now()
);
