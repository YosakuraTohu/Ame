use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::{BotGetter, EventReceiver};

/// Prelude for Plugin
pub mod prelude {
    pub use super::Plugin;
    pub use super::PluginInfo;
    pub use crate::event::{Event, MessageEvent, NoneBotEvent};
    pub use crate::event::{SelfId, UserId};
    pub use crate::message::Message;
    pub use tokio::task::JoinHandle;
    pub use toml;
    pub use uuid::{uuid, Uuid};
}

/// A trait for nbrs plugins
pub trait Plugin: std::fmt::Debug {
    /// Plugin 启动函数，在 NoneBot 启动时调用一次，不应当阻塞
    fn load(&self, event_receiver: EventReceiver, bot_getter: BotGetter) -> JoinHandle<()>;
    /// Plugin Name 用于注册 Plugin 时标识唯一性
    fn plugin_info(&self) -> PluginInfo;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PluginInfo {
    pub name: &'static str,
    pub author: &'static str,
    pub version: &'static str,
    pub desc: &'static str,
    pub id: Uuid,
}
