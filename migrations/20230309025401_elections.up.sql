-- Add up migration script here

create table if not exists elections (
    id serial primary key,
    title character varying(191),
    status integer default 0 not null,
    start timestamp(0) without time zone not null,
    "end" timestamp(0) without time zone,
    can_see_result boolean default false not null,
    created_at timestamp(0) without time zone not null default now(),
    updated_at timestamp(0) without time zone not null default now()
);
