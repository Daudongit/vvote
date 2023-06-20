-- Add up migration script here

create or replace function update_timestamp() returns trigger as $$
begin
    new.updated_at = now();
    return new;
end;
$$ language 'plpgsql';

create table if not exists users (
    id serial primary key,
    name character varying(191) not null,
    email character varying(191) not null unique,
    password character varying(191) not null,
    remember_token character varying(100),
    created_at timestamp(0) without time zone not null default now(),
    updated_at timestamp(0) without time zone not null default now()
);

create unique index users_unique_lower_email_idx on users (lower(email));

create trigger user_updated before insert or update on users
for each row execute procedure update_timestamp();

insert into users (id, name, email, password)
values 
    (1, 'Admin', 'admin@mail.com', '$2y$10$3rhkkZaioFOrAPM/J4QXQ.p6AMMdnTX7Sh/GYHmcmB8DIK85F78KO'),
    (2, 'admin', 'admin@gmail.com', 'pbkdf2_sha256$320000$XHuXWO5orSKQ$a03T/vmqdipUGPgkOQjEmSfY7GC3HpYL5PEy1rqvtAA=');
