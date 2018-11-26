-- Your SQL goes here
CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  token VARCHAR NOT NULL,
  user_id INTEGER NOT NULL,
  password_digest VARCHAR NOT NULL
)
