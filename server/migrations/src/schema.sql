create table if not exists _migrations (
    version integer primary key,
    name text not null,
    hash blob not null,
    applied_at integer default (strftime('%s', 'now'))
);
