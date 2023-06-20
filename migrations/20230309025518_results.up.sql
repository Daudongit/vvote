-- Add up migration script here

create table if not exists results (
    id serial primary key,
    voter_id integer not null,
    position_id integer not null,
    nominee_id integer not null,
    election_id integer not null,
    voter_ip character varying(191),
    voter_code character varying(191),
    created_at timestamp(0) without time zone not null default now(),
    updated_at timestamp(0) without time zone not null default now()
);