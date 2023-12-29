create table user_access_tokens (
    owner integer not null,
    name text not null,
    user_id integer not null,
    token text not null,
    created_at integer not null,
    expires_at integer,

    primary key (owner, name, user_id),
    foreign key (user_id) references users (id)
) strict;

create unique index user_access_tokens_token
on user_access_tokens (token);
