-- Add up migration script here
CREATE SCHEMA IF NOT EXISTS bouncer;

CREATE TABLE IF NOT EXISTS bouncer.users
(
    id         bigserial PRIMARY KEY,
    email      varchar(40) NOT NULL UNIQUE,
    name       varchar(40) NOT NULL,
    surname    varchar(40) NOT NULL,
    phone      varchar(15) NOT NULL,
    password   varchar     NOT NULL,
    created    timestamp   NOT NULL,
    last_login timestamp
);

INSERT INTO bouncer.users (email, name, surname, phone, password, created)
VALUES ('setupuser@mail.com', 'Setup', 'User', '0838748937', 'password', now())
