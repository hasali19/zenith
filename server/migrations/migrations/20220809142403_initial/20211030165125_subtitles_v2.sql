create table subtitles_v2 (
    id integer primary key,
    video_id integer not null,
    stream_index integer,
    path text,
    title text,
    language text,

    foreign key (video_id) references video_files (item_id)
);

insert into subtitles_v2 (id, video_id, stream_index, path, title, language)
select id, video_id, cast(replace(path, 'embedded://', '') as integer), null, title, language
from subtitles
where path like "embedded://%";

insert into subtitles_v2 (id, video_id, stream_index, path, title, language)
select id, video_id, null, path, title, language
from subtitles
where path not like "embedded://%";

drop table subtitles;

alter table subtitles_v2
rename to subtitles;

create unique index subtitles_video_id_stream_index
on subtitles (video_id, stream_index);

create unique index subtitles_video_id_path
on subtitles (video_id, path);
