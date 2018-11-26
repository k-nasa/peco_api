-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  token VARCHAR NOT NULL UNIQUE,
  user_id INTEGER NOT NULL UNIQUE,
  password_digest VARCHAR NOT NULL
)
