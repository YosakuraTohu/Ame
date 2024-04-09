create type msg_type as enum ('private', 'group');

create type msg_sender as (
    uid          text,
    nickname     text,
    sex          text,
    age          integer,
    card         text,
    title        text
);

create type msg_segment as (
    type     text,
    data     text
);

create table msg_rev (
    id          uuid primary key default gen_random_uuid(),
    mid         integer           not null,
    time        timestamptz       not null,
    type        msg_type          not null,
    source      text              not null,
    target      text              not null,
    sender      msg_sender        not null,
    msg         msg_segment[]     not null,
    raw_msg     text              not null
);
