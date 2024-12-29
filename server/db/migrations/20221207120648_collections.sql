create table collections (
    id integer primary key,
    name text not null,
    overview text
);

create table collections_media_items (
    collection_id integer not null,
    item_id integer not null,
    idx integer not null,

    primary key (collection_id, item_id),
    foreign key (collection_id) references collections (id),
    foreign key (item_id) references media_items (id)
);

create unique index collections_media_items_collection_id_idx
on collections_media_items (collection_id, idx);
