create table users (
    id serial primary key,
    username varchar(255) not null,
    avatar varchar(255) null
);
insert into users (username) values ('zbyszek');
