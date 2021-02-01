create table if not exists users (
    id text primary key,
    username text unique
);
