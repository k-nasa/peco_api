-- Your SQL goes here
create table send_emails (
  id serial primary key not null,
  user_id integer references users(id) not null,
  email text not null
)
