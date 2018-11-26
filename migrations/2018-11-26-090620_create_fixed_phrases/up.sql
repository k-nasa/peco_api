-- Your SQL goes here
CREATE TABLE fixed_phrases (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id) NOT NULL,
  yes_text TEXT NOT NULL default '必要なり',
  no_text TEXT NOT NULL default '不要なり'
)
