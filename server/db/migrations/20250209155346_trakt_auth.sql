create table trakt_user_auth(
    user_id integer primary key,
    refresh_token text,
    access_token text,
    expires_at text,

    foreign key (user_id) references users (id)
) strict;
