use crate::builtin::matcher::prelude::*;

#[derive(Debug)]
struct Status;

#[async_trait]
impl Handler<MessageEvent> for Status {
    crate::on_command!(MessageEvent, "status");
    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        matcher
            .send_text(&build_status(&event, &matcher).await)
            .await;
    }
}

async fn build_status(event: &MessageEvent, matcher: &Matcher<MessageEvent>) -> String {
    let friend_count = match matcher.get_friend_list().await {
        Some(flist) => flist.len(),
        None => 0,
    };
    let group_count = match matcher.get_group_list().await {
        Some(glist) => glist.len(),
        None => 0,
    };
    let time: String = if let Some(bot) = &matcher.bot {
        let connected_time = crate::utils::timestamp() - bot.connect_time;
        format_time(connected_time)
    } else {
        "-".to_string()
    };
    format!(
        "当前BotId：{}\n已连接时间：{}\n已加载好友数量：{}\n已加载群数量：{}",
        event.get_self_id(),
        time,
        friend_count,
        group_count
    )
}

pub fn bot_status() -> Matcher<MessageEvent> {
    let status = Status;
    Matcher::new("BotStatus", status)
        .add_rule(rules::is_superuser())
        .add_pre_matcher(prematchers::command_start())
}

fn format_time(time: i64) -> String {
    fn f(time: i64, div: i64, mut rs: String, s: &str) -> (String, i64) {
        let (time, remain) = (time / div, time % div);
        rs.insert_str(0, &format!("{}{}", remain, s));
        (rs, time)
    }

    let rs = String::new();
    let (rs, time) = f(time, 60, rs, "秒");
    if time == 0 {
        return rs;
    }
    let (rs, time) = f(time, 60, rs, "分");
    if time == 0 {
        return rs;
    }
    let (mut rs, time) = f(time, 24, rs, "时");
    if time == 0 {
        return rs;
    }
    rs.insert_str(0, &format!("{}{}", time, "天"));
    rs
}
