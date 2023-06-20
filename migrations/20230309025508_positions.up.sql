-- Add up migration script here

create table if not exists positions (
    id serial primary key,
    name character varying(60) not null,
    created_at timestamp(0) without time zone not null default now(),
    updated_at timestamp(0) without time zone not null default now()
);
