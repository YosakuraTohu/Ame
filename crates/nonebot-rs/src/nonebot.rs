use crate::{ActionSender, ApiChannelItem, ApiResp, Bot, Nonebot, Plugin};
use std::collections::HashMap;
use tokio::sync::{broadcast, mpsc, watch};
use tracing::{event, Level};
use uuid::Uuid;

impl Nonebot {
    /// 当 WenSocket 收到配置中未配置的 Bot 时，调用该方法新建 Bot 配置信息
    pub fn add_bot(
        &mut self,
        bot_id: String,
        api_sender: mpsc::Sender<ApiChannelItem>,
        action_sender: ActionSender,
        api_resp_watcher: watch::Receiver<ApiResp>,
    ) -> Bot {
        let bot = Bot::new(
            bot_id.clone(),
            self.config.gen_bot_config(&bot_id),
            api_sender,
            action_sender,
            api_resp_watcher,
        );
        self.bots.insert(bot_id.to_string(), bot.clone());
        self.bot_sender.send(self.bots.clone()).unwrap();
        bot
    }

    /// 移除 Bot，移除成功则返回移除的 Bot
    pub fn remove_bot(&mut self, bot_id: String) -> Option<Bot> {
        let bot_id = bot_id.to_string();
        let bot = self.bots.remove(&bot_id);
        self.bot_sender.send(self.bots.clone()).unwrap();
        bot
    }

    /// 新建一个 Matchers 为空的 Nonebot 结构体
    pub fn new() -> Self {
        Default::default()
    }

    /// 添加 Plugin
    pub fn add_plugin<T>(&mut self, plugin: T) -> &mut Self
    where
        T: Plugin + Send + Sync + 'static,
    {
        self.plugins
            .insert(plugin.plugin_info().id, Box::new(plugin));
        self
    }

    /// 移除 Plugin
    pub fn remove_plugin(&mut self, id: &Uuid) {
        self.plugins.remove(id);
    }

    #[doc(hidden)]
    pub async fn load_plugins_task(&self) {
        use colored::*;
        event!(Level::INFO, "Loaded Config Successful...");
        event!(Level::INFO, "{}", "高性能自律実験4号機が稼働中····".red());
        let mut tasks = self.tasks.lock().await;
        for plugin in self.plugins.values() {
            let task = plugin.load(self.event_sender.subscribe(), self.bot_getter.clone());
            let plugin_info = plugin.plugin_info();
            tasks.insert(plugin_info.id, Box::pin(task));
            if plugin.init().is_err() {
                event!(
                    Level::ERROR,
                    "Plugin {} {} init error.",
                    plugin_info.name.red(),
                    plugin_info.id.to_string().blue()
                );
            }
            event!(
                Level::INFO,
                "Plugin {} {} is loaded.",
                plugin_info.name.red(),
                plugin_info.id.to_string().blue()
            );
        }
    }

    async fn task_runner(&self) {
        let mut tasks = self.tasks.lock().await;
        for task in tasks.values_mut() {
            task.await.ok();
        }
    }

    /// 运行 Nonebot 实例
    pub async fn run(&mut self) {
        crate::connection::load_connection_task(self).await;
        self.load_plugins_task().await;
        self.handle_action().await;
        self.task_runner().await;
    }
}

impl Default for Nonebot {
    fn default() -> Self {
        let nb_config = crate::config::NoneBotConfig::load();
        let (event_sender, _) = broadcast::channel(1024); // need largo cache when reconnect
        let (action_sender, action_receiver) = tokio::sync::mpsc::channel(32);
        let (bot_sender, bot_getter) = watch::channel(HashMap::new());
        Nonebot {
            bots: Default::default(),
            config: nb_config,
            event_sender,
            action_sender,
            action_receiver,
            bot_sender,
            bot_getter,
            plugins: Default::default(),
            tasks: Default::default(),
        }
    }
}
