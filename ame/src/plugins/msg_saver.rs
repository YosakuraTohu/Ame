use ame_models::prelude::*;
use nonebot_rs::{
    event::{Event, MessageEvent},
    BotGetter, EventReceiver, Message,
};
use sqlx::postgres::PgPoolOptions;
use tokio::task::JoinHandle;
use tracing::{event, Level};
use uuid::{uuid, Uuid};

pub const PLUGIN_NAME: &str = "MsgSaver";
pub const PLUGIN_AUTHER: &str = "YosakuraTohu";
pub const PLUGIN_VERSION: &str = "v0.0.0";
pub const PLUGIN_DESC: &str = "Message saver";
pub const PLUGIN_ID: Uuid = uuid!("318f9313-a605-498b-9ee3-b5c63f059a24");

#[derive(Debug, Clone)]
pub struct MsgSaver;

impl MsgSaver {
    async fn event_recv(self, mut event_receiver: EventReceiver) {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://localhost/Services")
            .await;
        while let Ok(event) = event_receiver.recv().await {
            if let Ok(p) = &pool {
                if let Event::Message(m) = &event {
                    message_handler(m, p).await;
                }
            } else {
                event!(Level::ERROR, "Database connect fault.");
                break;
            }
        }
    }
}

async fn message_handler(event: &MessageEvent, pool: &sqlx::PgPool) {
    let msg_to_sg = |msg: Message| match msg {
        Message::Text { text } => MsgSegment {
            r#type: "text".to_string(),
            data: text,
        },
        Message::Face { id } => MsgSegment {
            r#type: "face".to_string(),
            data: id,
        },
        Message::Image {
            file: _,
            ty: _,
            url,
            cache: _,
            proxy: _,
            timeout: _,
        } => MsgSegment {
            r#type: "image".to_string(),
            data: url.unwrap_or_default(),
        },
        Message::Record {
            file: _,
            magic: _,
            url,
            cache: _,
            proxy: _,
            timeout: _,
        } => MsgSegment {
            r#type: "record".to_string(),
            data: url.unwrap_or_default(),
        },
        Message::Video {
            file: _,
            url,
            cache: _,
            proxy: _,
            timeout: _,
        } => MsgSegment {
            r#type: "video".to_string(),
            data: url.unwrap_or_default(),
        },
        Message::At { qq } => MsgSegment {
            r#type: "at".to_string(),
            data: qq,
        },
        Message::Poke { ty: _, id, name: _ } => MsgSegment {
            r#type: "poke".to_string(),
            data: id,
        },
        Message::Reply { id } => MsgSegment {
            r#type: "reply".to_string(),
            data: id,
        },
        Message::Forward { id } => MsgSegment {
            r#type: "forward".to_string(),
            data: id,
        },
        Message::Xml { data } => MsgSegment {
            r#type: "xml".to_string(),
            data,
        },
        Message::Json { data } => MsgSegment {
            r#type: "json".to_string(),
            data,
        },
        _ => MsgSegment {
            r#type: "other".to_string(),
            data: "".to_string(),
        },
    };
    match &event {
        MessageEvent::Private(p) => {
            let p = p.clone();
            let sender = MsgSender {
                uid: p.sender.user_id,
                nickname: p.sender.nickname,
                sex: p.sender.sex,
                age: p.sender.age,
                card: Default::default(),
                title: Default::default(),
            };
            let ms = p.message.into_iter().map(msg_to_sg).collect();
            let msg = WrappedMsgSegment(ms);
            let builder = MsgBuilder {
                mid: p.message_id,
                time: p.time,
                r#type: MsgType::Private,
                source: p.user_id,
                target: p.self_id,
                sender,
                msg,
                raw_msg: p.raw_message,
            };
            if let Err(e) = insert_msg_rev(pool, builder).await {
                event!(Level::ERROR, "Insert error:\n{:#?}", e)
            }
        }
        MessageEvent::Group(g) => {
            let g = g.clone();
            let sender = MsgSender {
                uid: g.sender.user_id,
                nickname: g.sender.nickname,
                sex: g.sender.sex,
                age: g.sender.age,
                card: g.sender.card,
                title: g.sender.title,
            };
            let ms = g.message.into_iter().map(msg_to_sg).collect();
            let msg = WrappedMsgSegment(ms);
            let builder = MsgBuilder {
                mid: g.message_id,
                time: g.time,
                r#type: MsgType::Group,
                source: g.group_id,
                target: g.self_id,
                sender,
                msg,
                raw_msg: g.raw_message,
            };
            if let Err(e) = insert_msg_rev(pool, builder).await {
                event!(Level::ERROR, "Insert error:\n{:#?}", e)
            }
        }
    }
}

impl nonebot_rs::Plugin for MsgSaver {
    fn load(&self, event_receiver: EventReceiver, _bot_getter: BotGetter) -> JoinHandle<()> {
        tokio::spawn(self.clone().event_recv(event_receiver))
    }

    fn plugin_info(&self) -> nonebot_rs::plugin::PluginInfo {
        nonebot_rs::plugin::PluginInfo {
            name: PLUGIN_NAME,
            author: PLUGIN_AUTHER,
            version: PLUGIN_VERSION,
            desc: PLUGIN_DESC,
            id: PLUGIN_ID,
        }
    }
}
