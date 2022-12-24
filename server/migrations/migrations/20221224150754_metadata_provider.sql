alter table media_items add metadata_provider text;
alter table media_items add metadata_provider_key text;

update media_items
set metadata_provider = "tmdb";

update media_items
set metadata_provider_key = cast(tmdb_id as text)
where item_type in (1, 2);

update media_items as season
set metadata_provider_key = (select tmdb_id from media_items as show where show.id = season.parent_id) || ":" || season.parent_index
where item_type = 3;

update media_items as episode
set metadata_provider_key = (select tmdb_id from media_items as show where show.id = episode.grandparent_id) || ":" || episode.grandparent_index || ":" || episode.parent_index
where item_type = 4;
