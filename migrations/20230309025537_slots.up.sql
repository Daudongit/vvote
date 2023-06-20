-- Add up migration script here

create table if not exists slots (
    id serial primary key,
    position_id integer not null,
    created_at timestamp(0) without time zone not null default now(),
    updated_at timestamp(0) without time zone not null default now()
);
