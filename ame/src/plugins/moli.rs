use std::collections::HashMap;

use moli_sdk::{Moli as MoliSdk, MoliReqestParameter};
use nonebot_rs::{
    event::{Event, MessageEvent, SelfId},
    Bot, BotGetter, EventReceiver, Message,
};
use tokio::{task::JoinHandle, time::Instant};
use uuid::{uuid, Uuid};

pub const PLUGIN_NAME: &str = "Moli";
pub const PLUGIN_AUTHER: &str = "YosakuraTohu";
pub const PLUGIN_VERSION: &str = "v0.0.0";
pub const PLUGIN_DESC: &str = "moli";
pub const PLUGIN_ID: Uuid = uuid!("467c481f-d34e-456b-8111-0eab92990f46");

static MOLI: MoliSdk = MoliSdk::new("ulpxdy2moi9ya4fm", "92kqb4ye");

#[derive(Default, Debug, Clone)]
pub struct Moli {
    bot_getter: Option<BotGetter>,
    group_timeout: HashMap<String, Instant>,
}

impl Moli {
    pub fn new() -> Self {
        Self::default()
    }

    async fn event_recv(mut self, mut event_receiver: EventReceiver) {
        while let Ok(event) = event_receiver.recv().await {
            let bots = self.bot_getter.clone().unwrap().borrow().clone();
            if let Some(bot) = bots.get(&event.get_self_id()) {
                if let Event::Message(m) = &event {
                    self.message_handler(m, bot).await;
                }
            }
        }
    }

    async fn message_handler(&mut self, event: &MessageEvent, bot: &Bot) {
        match &event {
            MessageEvent::Private(p) => {
                let msg = MoliReqestParameter::new(
                    p.raw_message.clone(),
                    1,
                    p.sender.nickname.clone(),
                    p.sender.user_id.clone(),
                    p.self_id.clone(),
                    "雨".to_string(),
                );
                if let Ok(res) = MOLI.get_response(msg).await {
                    if let Some(mss) = res.data {
                        let tm: String = mss
                            .into_iter()
                            .map(|ms| {
                                if ms.typed == 1 {
                                    return ms.content;
                                }
                                "".to_string()
                            })
                            .collect();
                        bot.send_by_message_event(event, vec![Message::Text { text: tm }])
                            .await;
                    };
                }
            }
            MessageEvent::Group(g) => {
                let mut timeout = true;
                if let Some(gt) = self.group_timeout.get(&g.group_id) {
                    if gt.elapsed().as_secs() < 30 {
                        timeout = false
                    } else {
                        self.group_timeout.remove(&g.group_id);
                    }
                }
                if g.raw_message.contains("小雨") || !timeout {
                    self.group_timeout
                        .insert(g.group_id.clone(), Instant::now());
                    let msg = MoliReqestParameter::new(
                        g.raw_message.clone(),
                        1,
                        g.sender.nickname.clone(),
                        g.sender.user_id.clone(),
                        g.self_id.clone(),
                        "雨".to_string(),
                    );
                    if let Ok(res) = MOLI.get_response(msg).await {
                        if let Some(mss) = res.data {
                            let tm: String = mss
                                .into_iter()
                                .map(|ms| {
                                    if ms.typed == 1 {
                                        return ms.content;
                                    }
                                    "".to_string()
                                })
                                .collect();
                            bot.send_by_message_event(event, vec![Message::Text { text: tm }])
                                .await;
                        };
                    };
                }
            }
        }
    }
}

impl nonebot_rs::Plugin for Moli {
    fn load(&self, event_receiver: EventReceiver, bot_getter: BotGetter) -> JoinHandle<()> {
        let mut moli = self.clone();
        moli.bot_getter = Some(bot_getter.clone());
        tokio::spawn(moli.event_recv(event_receiver))
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
