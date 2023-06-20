-- Add up migration script here

create table if not exists nominees (
    id serial primary key,
    first_name character varying(60) not null,
    last_name character varying(60) not null,
    email character varying(191),
    image character varying(191),
    description text,
    created_at timestamp(0) without time zone not null default now(),
    updated_at timestamp(0) without time zone not null default now()
);
