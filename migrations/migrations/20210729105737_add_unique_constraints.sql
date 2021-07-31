create unique index tv_shows_path
on tv_shows (path);

create unique index tv_seasons_show_id_season_number
on tv_seasons (show_id, season_number);

create unique index tv_episodes_season_id_episode_number
on tv_episodes (season_id, episode_number);

create unique index video_files_path
on video_files (path);

create unique index subtitles_video_id_path
on subtitles (video_id, path);
