alter table subtitles add format text;
alter table subtitles add sdh boolean not null default 0;
alter table subtitles add forced boolean not null default 0;
