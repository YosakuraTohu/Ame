#![allow(unused)]

use std::{collections::HashMap, str::FromStr};

use nonebot_rs::builtin::matcher::prelude::*;
use reqwest::Url;
use serde::Deserialize;

const API_URL: &str = "https://api.lolicon.app/setu/v2";

#[derive(Clone)]
pub struct Lolicon;

impl Lolicon {
    async fn get_api() -> Result<LoliconApi, reqwest::Error> {
        let mut url = Url::from_str(API_URL).unwrap();
        url.set_query(Some("num=1"));
        reqwest::get(url).await?.json::<LoliconApi>().await
    }

    async fn make_message() -> Result<Vec<Vec<Message>>, reqwest::Error> {
        let maker = |api: &LoliconData| {
            let img = Message::Image {
                file: api.urls.get("original").unwrap().to_owned(),
                ty: None,
                url: None,
                cache: None,
                proxy: None,
                timeout: Some(60),
            };
            let title = Message::Text {
                text: format!("{}\n", api.title),
            };
            let author = Message::Text {
                text: format!("作者: {}\n", api.author),
            };
            let uid = Message::Text {
                text: format!("uid: {}", api.uid),
            };
            vec![img, title, author, uid]
        };

        let msg: Vec<Vec<Message>> = Self::get_api().await?.data.iter().map(maker).collect();

        Ok(msg)
    }
}

#[async_trait]
impl Handler<MessageEvent> for Lolicon {
    on_command!(
        MessageEvent,
        "lolicon",
        "Lolicon",
        "loli",
        "Loli",
        "色图",
        "涩图",
        "涩涩",
        "色色",
        "萝莉",
        "来点色图",
        "来点涩图"
    );
    async fn handle(&self, _event: MessageEvent, matcher: Matcher<MessageEvent>) {
        matcher.send_text("正在装填弹药...").await;
        match Self::make_message().await {
            Ok(msgs) => {
                for msg in msgs {
                    matcher.send(msg).await
                }
            }
            _ => {
                matcher.send_text("装填失败").await;
            }
        }
    }
}

pub fn lolicon() -> Matcher<MessageEvent> {
    Matcher::new("Lolicon", Lolicon).add_pre_matcher(prematchers::option_command_start())
}

#[derive(Deserialize, Debug)]
struct LoliconApi {
    error: String,
    data: Vec<LoliconData>,
}

#[derive(Deserialize, Debug)]
struct LoliconData {
    pid: u32,
    p: u8,
    uid: u32,
    title: String,
    author: String,
    r18: bool,
    width: u16,
    height: u16,
    tags: Vec<String>,
    ext: String,
    #[serde(rename = "aiType")]
    ai_type: u8,
    #[serde(rename = "uploadDate")]
    upload_date: u64,
    urls: HashMap<String, String>,
}

#[test]
fn test_get_api() {
    tokio_test::block_on(async {
        let api = Lolicon::get_api().await.unwrap();
        let msg = Lolicon::make_message().await.unwrap();
        println!("{:#?}", api);
        println!("{:#?}", msg);
    });
}
