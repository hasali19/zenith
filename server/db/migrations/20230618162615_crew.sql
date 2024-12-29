create table crew (
    item_id integer not null,
    person_id integer not null,
    department text not null,
    job text not null,

    primary key (item_id, person_id, department, job),
    foreign key (item_id) references media_items (id),
    foreign key (person_id) references people (id)
);
