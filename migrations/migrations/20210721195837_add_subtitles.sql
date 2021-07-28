------------ Make video_files.item_id the primary key ------------

alter table video_files
rename to old__video_files;

create table video_files (
    item_id integer primary key,
    path text not null,
    duration real not null,

    foreign key (item_id) references media_items (id)
);

insert into video_files
select item_id, path, duration
from old__video_files;

drop table old__video_files;

------------ Add subtitles table ----------------------------------

create table subtitles (
    id integer primary key,
    video_id integer not null,
    path text not null,
    title text,
    language text,

    foreign key (video_id) references video_files (item_id)
);
