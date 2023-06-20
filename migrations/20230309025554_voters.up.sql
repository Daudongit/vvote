-- Add up migration script here

create table if not exists voters (
    id serial primary key,
    email character varying(191),
    ip character varying(191),
    confirmation_token character varying(25),
    phone character varying(191),
    name character varying(191),
    member_id character varying(191),
    remember_token character varying(100),
    created_at timestamp(0) without time zone not null default now(),
    updated_at timestamp(0) without time zone not null default now()
);
