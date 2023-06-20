-- Add up migration script here

create table if not exists settings (
    id serial primary key,
    key character varying(25),
    value character varying(191),
    created_at timestamp(0) without time zone not null default now(),
    updated_at timestamp(0) without time zone not null default now()
);