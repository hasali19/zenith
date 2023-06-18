create table people (
    id integer primary key,
    tmdb_id integer,
    name text not null,
    profile text
);

create index person_tmdb_id on people (tmdb_id);

create table cast (
    item_id integer not null,
    person_id integer not null,
    idx integer not null,
    character text,

    primary key (item_id, person_id),
    foreign key (item_id) references media_items (id),
    foreign key (person_id) references people (id)
);

create index cast_idx on cast (idx);
