create table media_items_new (
    id integer primary key,
    -- enum { movie = 1, show = 2, season = 3, episode = 4 }
    item_type integer not null,
    added_at integer default (strftime('%s', 'now')),
    updated_at integer,

    -- general metadata
    name text not null,
    overview text,
    start_date integer,
    end_date integer,

    -- images
    poster text,
    backdrop text,
    thumbnail text,

    -- external ids
    tmdb_id integer,

    -- show hierarchy
    parent_id integer,
    parent_index integer,
    grandparent_id integer,
    grandparent_index integer
);

insert into media_items_new (id, item_type, added_at, name, overview, start_date, poster, backdrop, tmdb_id)
select id, item_type, added_at, title, overview, release_date, poster, backdrop, tmdb_id
from movies
join media_items on item_id = id;

insert into media_items_new (id, item_type, added_at, name, start_date, end_date, overview, poster, backdrop, tmdb_id)
select id, item_type, added_at, name, start_date, end_date, overview, poster, backdrop, tmdb_id
from tv_shows
join media_items on item_id = id;

insert into media_items_new (id, item_type, added_at, parent_id, parent_index, name, overview, poster, tmdb_id)
select id, item_type, added_at, show_id, season_number, name, overview, poster, tmdb_id
from tv_seasons
join media_items on item_id = id;

insert into media_items_new (id, item_type, added_at, parent_id, parent_index, grandparent_id, grandparent_index, name, start_date, overview, thumbnail, tmdb_id)
select id, item_type, added_at, season_id, episode_number, tv_seasons.show_id, tv_seasons.season_number, tv_episodes.name, air_date, tv_episodes.overview, thumbnail, tv_episodes.tmdb_id
from tv_episodes
join media_items on tv_episodes.item_id = id
join tv_seasons on season_id = tv_seasons.item_id;

create table indexed_paths (
    id integer primary key,
    item_id integer not null,
    path text not null,

    foreign key (item_id) references media_items_new (id)
);

create unique index index_paths_path
on indexed_paths (path);

insert into indexed_paths (item_id, path)
select item_id, path from tv_shows;

create table user_item_data_new (
    item_id integer primary key,
    position real not null default 0,
    is_watched boolean not null default 0,
    last_watched_at integer,

    foreign key (item_id) references media_items_new (id)
);

insert into user_item_data_new
select item_id, position, is_watched, last_watched_at
from user_item_data;

create table video_files_new (
    item_id integer primary key,
    path text not null,
    duration real,
    format_name text,

    foreign key (item_id) references media_items_new (id)
);

insert into video_files_new
select item_id, path, duration, format_name
from video_files;

create table video_file_streams_new (
    id integer primary key,
    video_id integer not null,
    stream_index integer not null,
    stream_type integer not null,
    codec_name text not null,

    -- video properties
    v_width integer,
    v_height integer,

    -- audio properties
    a_language text,

    foreign key (video_id) references video_files_new (item_id)
);

insert into video_file_streams_new
select id, video_id, stream_index, stream_type, codec_name, v_width, v_height, a_language
from video_file_streams;

create table subtitles_new (
    id integer primary key,
    video_id integer not null,
    stream_index integer,
    path text,
    title text,
    language text,

    foreign key (video_id) references video_files_new (item_id)
);

insert into subtitles_new
select s.id, s.video_id, s.stream_index, s.path, s.title, s.language
from subtitles as s
join video_files_new as v on s.video_id = v.item_id;

drop table movies;
drop table tv_episodes;
drop table tv_seasons;
drop table tv_shows;
drop table subtitles;
drop table video_file_streams;
drop table video_files;
drop table user_item_data;
drop table media_items;

alter table media_items_new rename to media_items;
alter table user_item_data_new rename to user_item_data;
alter table video_files_new rename to video_files;
alter table video_file_streams_new rename to video_file_streams;
alter table subtitles_new rename to subtitles;

create unique index media_items_grandparent_id_parent_id_parent_index
on media_items (grandparent_id, parent_id, parent_index);

create unique index video_files_path
on video_files (path);

create unique index video_file_streams_video_id_index
on video_file_streams (video_id, stream_index);

create unique index subtitles_video_id_stream_index
on subtitles (video_id, stream_index);

create unique index subtitles_video_id_path
on subtitles (path);

create view movies (
    id,
    item_type,
    added_at,
    name,
    overview,
    start_date,
    end_date,
    poster,
    backdrop,
    thumbnail,
    tmdb_id
)
as select id, item_type, added_at, name, overview, start_date, end_date, poster, backdrop, thumbnail, tmdb_id
from media_items
where item_type = 1;

create view shows (
    id,
    item_type,
    added_at,
    name,
    overview,
    start_date,
    end_date,
    poster,
    backdrop,
    thumbnail,
    tmdb_id
)
as select id, item_type, added_at, name, overview, start_date, end_date, poster, backdrop, thumbnail, tmdb_id
from media_items
where item_type = 2;

create view seasons (
    id,
    item_type,
    added_at,
    name,
    overview,
    start_date,
    end_date,
    poster,
    backdrop,
    thumbnail,
    tmdb_id,
    show_id,
    season_no
)
as select id, item_type, added_at, name, overview, start_date, end_date, poster, backdrop, thumbnail, tmdb_id, parent_id, parent_index
from media_items
where item_type = 3;

create view episodes (
    id,
    item_type,
    added_at,
    name,
    overview,
    start_date,
    end_date,
    poster,
    backdrop,
    thumbnail,
    tmdb_id,
    show_id,
    season_no,
    season_id,
    episode_no
)
as select id, item_type, added_at, name, overview, start_date, end_date, poster, backdrop, thumbnail, tmdb_id, grandparent_id, grandparent_index, parent_id, parent_index
from media_items
where item_type = 4;
