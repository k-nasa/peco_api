-- Your SQL goes here
create table fixed_phrases (
  id serial not null primary key,
  user_id integer references users(id) not null,
  yes_text text not null default '必要なり',
  no_text text not null default '不要なり'
)
