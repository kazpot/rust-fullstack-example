-- Add migration script here
CREATE TABLE IF NOT EXISTS products (
    id serial,
    name text,
    price integer
);

CREATE TABLE IF NOT EXISTS service_users (
    id serial,
    service_key text,
    name text
);