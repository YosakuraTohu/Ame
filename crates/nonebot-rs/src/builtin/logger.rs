use crate::{
    event::{Event, MessageEvent, MetaEvent},
    BotGetter, EventReceiver,
};
use colored::*;
use tokio::task::JoinHandle;
use tracing::{event, Level};
use uuid::{uuid, Uuid};

pub const PLUGIN_NAME: &str = "Logger";
pub const PLUGIN_AUTHER: &str = "abrahum";
pub const PLUGIN_VERSION: &str = "v0.0.0";
pub const PLUGIN_DESC: &str = "logger";
pub const PLUGIN_ID: Uuid = uuid!("b86ad211-5bd5-42e8-8a74-4a40f37b78c1");

/// Message Event Logger
pub fn message_logger(event: &MessageEvent) {
    match &event {
        MessageEvent::Private(p) => {
            let mut user_id = p.user_id.to_string();
            while user_id.len() < 10 {
                user_id.insert(0, ' ');
            }
            event!(
                Level::INFO,
                "{} [{}] -> {} from {}({})",
                user_id.green(),
                p.self_id.to_string().red(),
                p.raw_message,
                p.sender.nickname.to_string().blue(),
                p.user_id.to_string().green(),
            )
        }
        MessageEvent::Group(g) => {
            let mut group_id = g.group_id.to_string();
            while group_id.len() < 10 {
                group_id.insert(0, ' ');
            }
            event!(
                Level::INFO,
                "{} [{}] -> {} from {}({})",
                group_id.magenta(),
                g.self_id.to_string().red(),
                g.raw_message,
                g.sender.nickname.to_string().blue(),
                g.user_id.to_string().green(),
            )
        }
    }
}

/// Meta Event Logger
pub fn meta_logger(event: &MetaEvent) {
    if &event.meta_event_type == "heartbeat" {
        event!(Level::TRACE, "Recive HeartBeat")
    }
}

#[derive(Debug, Clone)]
pub struct Logger;

impl Logger {
    async fn event_recv(self, mut event_receiver: EventReceiver) {
        while let Ok(event) = event_receiver.recv().await {
            match &event {
                Event::Message(m) => message_logger(m),
                Event::Meta(m) => meta_logger(m),
                _ => {}
            }
        }
    }
}

impl crate::Plugin for Logger {
    fn load(&self, event_receiver: EventReceiver, _: BotGetter) -> JoinHandle<()> {
        let logger = self.clone();
        tokio::spawn(logger.event_recv(event_receiver))
    }

    fn plugin_info(&self) -> crate::plugin::PluginInfo {
        crate::plugin::PluginInfo {
            name: PLUGIN_NAME,
            author: PLUGIN_AUTHER,
            version: PLUGIN_VERSION,
            desc: PLUGIN_DESC,
            id: PLUGIN_ID,
        }
    }
}
