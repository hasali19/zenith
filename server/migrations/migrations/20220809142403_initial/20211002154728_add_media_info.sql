alter table video_files
add format_name text;

create table video_file_streams (
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

    foreign key (video_id) references video_files (item_id)
);

create unique index video_file_streams_video_id_index
on video_file_streams (video_id, stream_index);
