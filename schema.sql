create table transfer
(
    id text not null constraint transfer_id_pk primary key,
    "timestamp" bigint,
    block_number bigint,
    log_index integer,
    tx_hash bytea,
    amount numeric,
    sender bytea,
    receiver bytea
);

create table Approval
(
    id text not null constraint approval_id_pk primary key,
    "timestamp" bigint,
    block_number bigint,
    log_index integer,
    tx_hash bytea,
    "value" numeric,
    spender bytea,
    "owner" bytea
);


create table cursors
(
    id         text not null constraint cursor_pk primary key,
    cursor     text,
    block_num  bigint,
    block_id   text
);
