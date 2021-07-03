create table media_items (
    id integer primary key,
    item_type integer not null,
    added_at integer default (strftime('%s', 'now')),
    updated_at integer
);

create table video_files (
    item_id integer not null,
    path text not null,
    duration real not null,

    foreign key (item_id) references media_items (id)
);

create table movies (
    item_id integer primary key,
    title text not null,
    release_date integer,
    overview text,
    poster text,
    backdrop text,

    foreign key (item_id) references media_items (id)
);

create table tv_shows (
    item_id integer primary key,
    path text not null,
    name text not null,
    start_date integer,
    end_date integer,
    overview text,
    poster text,
    backdrop text,
    tmdb_id integer,

    foreign key (item_id) references media_items (id)
);

create table tv_seasons (
    item_id integer primary key,
    show_id integer not null,
    season_number integer not null,
    name text,
    overview text,
    poster text,
    tmdb_id integer,

    foreign key (item_id) references media_items (id)
);

create table tv_episodes (
    item_id integer primary key,
    season_id integer not null,
    episode_number integer not null,
    name text,
    air_date integer,
    overview text,
    thumbnail text,
    tmdb_id integer,

    foreign key (item_id) references media_items (id)
);

create table user_item_data (
    item_id integer primary key,
    position real not null default 0,
    is_watched boolean not null default 0,

    foreign key (item_id) references media_items (id)
);
