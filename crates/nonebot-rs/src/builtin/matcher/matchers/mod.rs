use crate::builtin::matcher::Matcher;
use crate::event::NoneBotEvent::{BotConnect, BotDisconnect};
use crate::event::{Event, MessageEvent, MetaEvent, NoticeEvent, RequestEvent, SelfId};
use crate::{BotGetter, EventReceiver, Plugin};
use colored::*;
use std::collections::{BTreeMap, HashMap};
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tracing::{event, Level};
use uuid::{uuid, Uuid};

mod action;

/// 按 `priority` 依序存储 `MatchersHashMap`
pub type MatchersBTreeMap<E> = BTreeMap<i8, MatchersHashMap<E>>;
/// 使用唯一名字存储 `Matcher`
pub type MatchersHashMap<E> = HashMap<String, Matcher<E>>;
/// Matchers Action Sender
pub type ActionSender = broadcast::Sender<super::action::MatchersAction>;

pub const PLUGIN_NAME: &str = "Matcher";
pub const PLUGIN_AUTHER: &str = "abrahum";
pub const PLUGIN_VERSION: &str = "v0.0.0";
pub const PLUGIN_DESC: &str = "";
pub const PLUGIN_ID: Uuid = uuid!("17f00592-64f7-4c9d-9285-3cbab3d8b7b3");

/// 根据 `Event` 类型分类存储对应的 `Matcher`
#[derive(Clone, Debug)]
pub struct Matchers {
    /// MessageEvent 对应 MatcherBTreeMap
    pub message: MatchersBTreeMap<MessageEvent>,
    /// NoticeEvent 对应 MatcherBTreeMap
    pub notice: MatchersBTreeMap<NoticeEvent>,
    /// RequestEvent 对应 MatcherBTreeMap
    pub request: MatchersBTreeMap<RequestEvent>,
    /// MetaEvent 对应 MatcherBTreeMap
    pub meta: MatchersBTreeMap<MetaEvent>,
    /// Bot Watch channel Receiver
    bot_getter: Option<BotGetter>,
    /// Matchers Action Sender
    action_sender: ActionSender,
}

impl Matchers {
    async fn handle_events(&mut self, event: Event, bot: &crate::bot::Bot) {
        match event {
            Event::Message(e) => {
                self.handle_event(self.message.clone(), e, bot.clone())
                    .await;
            }
            Event::Notice(e) => {
                self.handle_event(self.notice.clone(), e, bot.clone()).await;
            }
            Event::Request(e) => {
                self.handle_event(self.request.clone(), e, bot.clone())
                    .await;
            }
            Event::Meta(e) => {
                self.handle_event(self.meta.clone(), e, bot.clone()).await;
            }
            Event::Nonebot(e) => match e {
                BotConnect { bot } => {
                    log_load_matchers(self);
                    self.run_on_connect(bot, false).await;
                }
                BotDisconnect { bot } => {
                    self.run_on_connect(bot, true).await;
                }
            },
        }
    }

    /// 接收按类型分发后的 Event 逐级匹配 Matcher
    async fn handle_event<E>(
        &mut self,
        mut matcherb: MatchersBTreeMap<E>,
        event: E,
        bot: crate::bot::Bot,
    ) where
        E: Clone + Send + 'static + std::fmt::Debug + SelfId,
    {
        event!(Level::TRACE, "handling event {:?}", event);
        // 根据不同 Event 类型，逐级匹配，判定是否 Block
        for (_, matcherh) in matcherb.iter_mut() {
            if self
                ._handler_event(matcherh, event.clone(), bot.clone())
                .await
            {
                break;
            };
        }
    }

    #[doc(hidden)]
    async fn _handler_event<E>(
        &mut self,
        matcherh: &mut MatchersHashMap<E>,
        e: E,
        bot: crate::bot::Bot,
    ) -> bool
    where
        E: Clone + Send + 'static + std::fmt::Debug + SelfId,
    {
        event!(Level::TRACE, "handling event_ {:?}", e);
        // 每级 Matcher 匹配，返回是否 block
        let mut get_block = false;
        let config = bot.config.clone();
        for (name, matcher) in matcherh.iter_mut() {
            let matched = matcher
                .build(bot.clone())
                .match_(e.clone(), config.clone(), self)
                .await;
            if matched {
                event!(Level::INFO, "Matched {}", name.blue());
                if matcher.is_block() {
                    get_block = true;
                }
                if matcher.is_temp() {
                    event!(Level::INFO, "Remove matched temp matcher {}", name.blue());
                    self.remove_matcher(name);
                }
            }
        }
        get_block
    }

    async fn event_recv(mut self, mut event_receiver: EventReceiver) {
        let mut receiver = self.action_sender.subscribe();
        while let Ok(event) = event_receiver.recv().await {
            if let Ok(action) = receiver.try_recv() {
                self.handle_action(action)
            }

            let bots = self.bot_getter.clone().unwrap().borrow().clone();
            if let Some(bot) = bots.get(&event.get_self_id()) {
                self.handle_events(event, bot).await;
            }
        }
    }
}

impl Plugin for Matchers {
    fn load(&self, event_receiver: EventReceiver, bot_getter: BotGetter) -> JoinHandle<()> {
        let mut matchers = self.clone();
        matchers.bot_getter = Some(bot_getter.clone());
        init_matchers(&matchers);
        tokio::spawn(matchers.event_recv(event_receiver))
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

fn log_load_matchers(matchers: &Matchers) {
    log_matcherb(&matchers.message);
    log_matcherb(&matchers.notice);
    log_matcherb(&matchers.request);
    log_matcherb(&matchers.meta);
}

fn log_matcherb<E>(matcherb: &MatchersBTreeMap<E>)
where
    E: Clone,
{
    if matcherb.is_empty() {
        return;
    }
    for matcherh in matcherb.values() {
        for name in matcherh.keys() {
            event!(Level::INFO, "Matcher {} is Loaded", name.blue());
        }
    }
}

fn init_matchers(matchers: &Matchers) {
    init_matcherb(matchers, &matchers.message);
    init_matcherb(matchers, &matchers.notice);
    init_matcherb(matchers, &matchers.request);
    init_matcherb(matchers, &matchers.meta);
}

fn init_matcherb<E>(matchers: &Matchers, matcherb: &MatchersBTreeMap<E>)
where
    E: Clone,
{
    if matcherb.is_empty() {
        return;
    }
    for matcherh in matcherb.values() {
        for matcher in matcherh.values() {
            if matcher.init(&matchers.get_plugin_data_path()).is_err() {
                event!(Level::ERROR, "Matcher {} init error.", matcher.name.red());
            }
        }
    }
}
