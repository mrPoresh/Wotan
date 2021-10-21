--CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table users (
    id uuid not null,
    username text not null unique,
    email text not null unique,
    password_hash text not null,
    full_name text null,
    --bio varchar null,
    --image varchar null,
    --email_verified boolean not null default false,
    active boolean not null default true,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);