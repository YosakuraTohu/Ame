mod action;
/// Onebot Api
pub mod api;
/// Onebot Api Response
pub mod api_resp;
mod bot;
/// 内建组件
pub mod builtin;
/// nbrs 设置项
pub mod config;
pub mod connection;
/// Onebot 事件
pub mod event;
#[doc(hidden)]
pub mod message;
mod nonebot;
#[doc(hidden)]
pub mod plugin;
// /// scheduler Plugin
// pub mod scheduler;
mod utils;

use std::{collections::HashMap, pin::Pin};
use tokio::{
    sync::{broadcast, mpsc, watch, Mutex},
    task::JoinHandle,
};

#[doc(inline)]
pub use action::Action;
#[doc(inline)]
pub use api_resp::{ApiResp, RespData};
pub use async_trait::async_trait;
#[doc(inline)]
pub use bot::Bot;
#[doc(inline)]
#[doc(inline)]
pub use message::Message;
#[doc(inline)]
pub use plugin::Plugin;

// pub use scheduler::Scheduler;

pub use builtin::logger::Logger;
pub use builtin::matcher::matchers::Matchers;

/// Onebot Api mpsc channel Bot 发送 WebSocket 接收
pub type ApiSender = mpsc::Sender<ApiChannelItem>;
/// Bot 监视 Onebot ApiResp Watch channel
pub type ApiRespWatcher = watch::Receiver<ApiResp>;
/// Event broadcast channel sender 所有 WebSocket Plugin 共享，
/// WebSocket 发送，Plugin 接收
pub type EventSender = broadcast::Sender<event::Event>;
/// Event broadcast channel receiver 所有 WebSocket Plugin 共享，
/// WebSocket 发送，Plugin 接收
pub type EventReceiver = broadcast::Receiver<event::Event>;
/// Nonebot Action Sender，Bot 发送，Nonebot 接收
pub type ActionSender = mpsc::Sender<Action>;
/// Nonebot Action Sender，Bot 发送，Nonebot 接收
pub type ActionReceiver = mpsc::Receiver<Action>;
/// 广播所有可用的 Bot
pub type BotSender = watch::Sender<HashMap<String, Bot>>;
/// 接收广播的所有可用 Bot
pub type BotGetter = watch::Receiver<HashMap<String, Bot>>;
/// nbrs 本体
///
/// 用于注册 `Matcher`，暂存配置项，以及启动实例
pub struct Nonebot {
    /// Nonebot 设置
    pub config: config::NoneBotConfig,
    /// 储存 Nonebot 下连接的 Bot
    pub bots: HashMap<String, Bot>,
    /// 暂存 Events Sender 由 WebSocket 广播 Event
    event_sender: EventSender,
    /// Nonebot Action Sender
    action_sender: ActionSender,
    /// Nonebot Action Receiver
    action_receiver: ActionReceiver,
    /// Bot Sender
    pub bot_sender: BotSender,
    /// Bot Getter
    pub bot_getter: BotGetter,
    /// event handler
    plugins: HashMap<uuid::Uuid, Box<dyn Plugin + Send + Sync>>,
    /// Bot tasks
    tasks: Mutex<HashMap<uuid::Uuid, Pin<Box<JoinHandle<()>>>>>,
}

/// api channel 传递项
#[derive(Debug)]
pub enum ApiChannelItem {
    /// Onebot Api
    Api(api::Api),
    /// Event 用于临时 Matcher 与原 Matcher 传递事件 todo
    MessageEvent(event::MessageEvent),
    /// Time out 通知T
    TimeOut,
}
