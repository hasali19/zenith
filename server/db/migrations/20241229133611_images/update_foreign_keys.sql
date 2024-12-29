alter table media_items add column poster_id integer references images (id);
alter table media_items add column backdrop_id integer references images (id);
alter table media_items add column thumbnail_id integer references images (id);

update media_items set
    poster_id = poster,
    backdrop_id = backdrop,
    thumbnail_id = thumbnail;

drop view movies;
drop view shows;
drop view seasons;
drop view episodes;

alter table media_items drop column poster;
alter table media_items drop column backdrop;
alter table media_items drop column thumbnail;

alter table media_items rename column poster_id to poster;
alter table media_items rename column backdrop_id to backdrop;
alter table media_items rename column thumbnail_id to thumbnail;

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

alter table people add column profile_id text references images (id);

update people set
    profile_id = profile;

alter table people drop column profile;

alter table people rename column profile_id to profile;
