use uuid::Uuid;

use crate::Nonebot;

pub mod revs_ws;
pub mod utils;
pub mod ws;

pub async fn load_connection_task(nonebot: &Nonebot) {
    let access_token = nonebot.config.gen_access_token();
    let mut tasks = nonebot.tasks.lock().await;

    if let Some(ws_server_config) = &nonebot.config.ws_server {
        tasks.insert(
            Uuid::new_v4(),
            Box::pin(tokio::spawn(revs_ws::run(
                ws_server_config.host,
                ws_server_config.port,
                nonebot.event_sender.clone(),
                nonebot.action_sender.clone(),
                access_token.clone(),
            ))),
        );
    }

    if let Some(bots) = &nonebot.config.bots {
        for (bot_id, bot_config) in bots {
            if !bot_config.ws_server.is_empty() {
                tasks.insert(
                    Uuid::new_v4(),
                    Box::pin(tokio::spawn(ws::run(
                        bot_config.ws_server.clone(),
                        bot_id.clone(),
                        nonebot.event_sender.clone(),
                        nonebot.action_sender.clone(),
                        access_token.clone(),
                    ))),
                );
            }
        }
    }
}
