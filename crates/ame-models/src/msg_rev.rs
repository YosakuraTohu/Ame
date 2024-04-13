use chrono::{DateTime, Utc};
use sqlx::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
#[sqlx(type_name = "msg_rev")]
pub struct MsgRev {
    pub id: Uuid,
    pub mid: i32,
    pub time: DateTime<Utc>,
    pub r#type: MsgType,
    pub source: String,
    pub target: String,
    pub sender: MsgSender,
    pub msg: WrappedMsgSegment,
    pub raw_msg: String,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "msg_type", rename_all = "snake_case")]
pub enum MsgType {
    Private,
    Group,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "msg_sender")]
pub struct MsgSender {
    pub uid: String,
    pub nickname: String,
    pub sex: String,
    pub age: i32,
    pub card: String,
    pub title: String,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "msg_segment")]
pub struct MsgSegment {
    pub r#type: String,
    pub data: String,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "_msg_segment")]
pub struct WrappedMsgSegment(pub Vec<MsgSegment>);

#[derive(Debug, Clone)]
pub struct MsgBuilder {
    pub mid: i32,
    pub time: i64,
    pub r#type: MsgType,
    pub source: String,
    pub target: String,
    pub sender: MsgSender,
    pub msg: WrappedMsgSegment,
    pub raw_msg: String,
}

#[derive(Debug, Clone)]
pub struct TIU {
    pub time: DateTime<Utc>,
    pub mid: i32,
    pub urls: Vec<String>,
}

pub async fn fetch_latest_msg_rev(
    pool: &sqlx::PgPool,
    len: i32,
) -> Result<Vec<MsgRev>, sqlx::error::Error> {
    let rows = sqlx::query_as_unchecked!(MsgRev, "select * from msg_rev limit $1", len)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn insert_msg_rev(
    pool: &sqlx::PgPool,
    builder: MsgBuilder,
) -> Result<(), sqlx::error::Error> {
    let dt = DateTime::from_timestamp(builder.time, 0).unwrap_or_default();
    sqlx::query!(
        "insert into msg_rev values (default, $1, $2, $3, $4, $5, $6, $7, $8)",
        builder.mid,
        dt,
        builder.r#type as _,
        builder.source,
        builder.target,
        builder.sender as _,
        builder.msg as _,
        builder.raw_msg
    )
    .fetch_all(pool)
    .await?;
    Ok(())
}

pub async fn fetch_latest_tiu(
    pool: &sqlx::PgPool,
    len: i32,
    offset: i32,
) -> Result<Vec<TIU>, sqlx::error::Error> {
    let res = sqlx::query_as_unchecked!(
        TIU,
        r#"
        select
            time, mid, array_agg(m.data) urls
        from
            msg_rev, unnest(msg) as m
        where
            m.type = 'image'
        group by
            time, mid
        order by
            time asc
        limit $1
        offset $2
        "#,
        len,
        offset
    )
    .fetch_all(pool)
    .await?;
    Ok(res)
}
