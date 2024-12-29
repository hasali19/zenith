drop index video_files_path;
drop index video_file_streams_video_id_index;
drop index subtitles_video_id_stream_index;
drop index subtitles_video_id_path;

create table video_files_new (
    id integer primary key,
    item_id integer not null,
    path text not null,
    duration real,
    format_name text,

    foreign key (item_id) references media_items (id)
) strict;

create table video_file_streams_new (
    id integer primary key,
    video_id integer not null,
    stream_index integer not null,
    stream_type text not null,
    codec_name text not null,

    -- video properties
    v_width integer,
    v_height integer,

    -- audio properties
    a_language text,

    foreign key (video_id) references video_files_new (id)
) strict;

create table subtitles_new (
    id integer primary key,
    video_id integer not null,
    stream_index integer,
    path text,
    title text,
    language text,
    format text,
    sdh integer not null default 0,
    forced integer not null default 0,

    foreign key (video_id) references video_files_new (id)
) strict;

create unique index video_files_path on video_files_new (path);

create unique index video_file_streams_video_id_index
on video_file_streams_new (video_id, stream_index);

create unique index subtitles_video_id_stream_index
on subtitles_new (video_id, stream_index);

create unique index subtitles_video_id_path on subtitles_new (path);

insert into video_files_new (id, item_id, path, duration, format_name)
select item_id, item_id, path, duration, format_name
from video_files;

insert into video_file_streams_new (id, video_id, stream_index, stream_type, codec_name, v_width, v_height, a_language)
select id, video_id, stream_index, stream_type, codec_name, v_width, v_height, a_language
from video_file_streams;

insert into subtitles_new (id, video_id, stream_index, path, title, language, format, sdh, forced)
select id, video_id, stream_index, path, title, language, format, sdh, forced
from subtitles;

drop table subtitles;
drop table video_file_streams;
drop table video_files;

alter table video_files_new rename to video_files;
alter table video_file_streams_new rename to video_file_streams;
alter table subtitles_new rename to subtitles;
