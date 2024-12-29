alter table video_files
add path_stem text generated always as
    (replace(path, ltrim(path, replace(path, '.', '')), '')) virtual;

create index video_files_stem on video_files (path_stem);
