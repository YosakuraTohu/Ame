use super::utils::handler_web_socket;
use crate::{builtin::matcher::prelude::SelfId, event::Event, ActionSender, EventSender};
use async_recursion::async_recursion;
use colored::*;
use futures_util::StreamExt;
use http::{header::USER_AGENT, Uri};
use tokio::{
    net::TcpStream,
    sync::{mpsc, watch},
};
use tracing::{event, Level};

use tokio_tungstenite::{
    client_async,
    tungstenite::handshake::client::{generate_key, Request},
};

#[async_recursion]
pub async fn run(
    url: String,
    bot_id: String,
    event_sender: EventSender,
    action_sender: ActionSender,
    access_token: crate::config::AccessToken,
) {
    single_socket(
        &url,
        &bot_id,
        event_sender.clone(),
        action_sender.clone(),
        access_token.clone(),
    )
    .await;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    run(url, bot_id, event_sender, action_sender, access_token).await;
}

pub async fn single_socket(
    url: &str,
    bot_id: &str,
    event_sender: EventSender,
    action_sender: ActionSender,
    access_token: crate::config::AccessToken,
) {
    let uri: Uri = url.parse().unwrap();
    let addr = format!("{}:{}", uri.host().unwrap(), uri.port().unwrap());
    let authority = match uri.authority() {
        Some(authority) => authority.as_str(),
        None => {
            return;
        }
    };
    let host = authority
        .find('@')
        .map(|idx| authority.split_at(idx + 1).1)
        .unwrap_or_else(|| authority);
    let req = Request::builder()
        .method("GET")
        .header("Host", host)
        .header("Authorization", access_token.get(bot_id))
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", generate_key())
        .header(USER_AGENT, format!("OneBot/11 Ame/NoneBot-rs/{}", "0.4.0"))
        .uri(uri)
        .body(())
        .unwrap();

    event!(Level::INFO, "Connecting to {}", url);

    let tcp_stream = match TcpStream::connect(addr).await {
        Ok(s) => s,
        Err(_) => return,
    };

    // build channel
    let (sender, receiver) = mpsc::channel(32);
    let (apiresp_watch_sender, api_resp_watcher) = watch::channel(crate::api_resp::ApiResp {
        status: "init".to_string(),
        retcode: 0,
        data: crate::api_resp::RespData::None,
        echo: "".to_string(),
    });

    let ws_stream = client_async(req, tcp_stream).await;
    let Ok((mut stream, _)) = ws_stream else {
        return;
    };
    // let headers = resp.headers();
    // println!("{:?}", headers);
    if let Some(Ok(msg)) = stream.next().await {
        let msg = msg.to_text().unwrap();
        let event: Event = serde_json::from_str(msg).unwrap();
        let bot_id = event.get_self_id();

        event!(Level::INFO, "Connectted to Bot {} Server", bot_id.red());

        // add bot to Nonebot
        action_sender
            .send(crate::Action::AddBot {
                bot_id: bot_id.clone(),
                api_sender: sender,
                action_sender: action_sender.clone(),
                api_resp_watcher,
            })
            .await
            .unwrap();

        // handle WebSocketStream
        handler_web_socket(
            stream,
            event_sender,
            action_sender,
            apiresp_watch_sender,
            receiver,
            bot_id,
        )
        .await;
    }
}
