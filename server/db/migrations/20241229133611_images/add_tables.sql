create table images (
    id text primary key,
    image_type int not null,
    source_type int not null,
    source text not null
) strict;

create unique index images_image_type_source_type_source
on images (image_type, source_type, source);
