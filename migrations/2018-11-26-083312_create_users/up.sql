-- Your SQL goes here
create table users (
  id serial primary key,
  token varchar not null unique,
  user_id integer not null unique,
  password_digest varchar not null
)
