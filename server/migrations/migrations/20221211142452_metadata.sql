alter table media_items add metadata_updated_at integer;
alter table media_items add age_rating text;
alter table media_items add imdb_id text;

-- Set metadata_updated_at to random time in the last week. This is to spread out refreshes over
-- the week.
update media_items
set metadata_updated_at = strftime('%s') - abs(random() % (60 * 60 * 24 * 7))
where metadata_updated_at is null;

create table genres (
    id integer primary key,
    name text not null
);

create unique index genres_name on genres (name);

create table media_items_genres (
    item_id integer not null,
    genre_id integer not null,

    primary key (item_id, genre_id),
    foreign key (item_id) references media_items (id),
    foreign key (genre_id) references genres (id)
);
