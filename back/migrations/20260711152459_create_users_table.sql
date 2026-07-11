-- migrate:up
create table if not exists users (
    id uuid primary key default gen_random_uuid(),
    username varchar(50) not null unique,
    email varchar(255) not null unique,
    paswword_hash varchar(255) not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

-- migrate:down
drop table if exists users;