create table users (
    id integer primary key,
    username text not null unique,
    password_hash text not null
) strict;

drop table user_item_data;

create table media_item_user_data (
    item_id integer not null,
    user_id integer not null,
    is_watched integer not null default 0,
    position real not null default 0,
    position_updated_at integer,

    primary key (item_id, user_id),
    foreign key (item_id) references media_items (id),
    foreign key (user_id) references users (id)
) strict;
