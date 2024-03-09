use nonebot_rs::builtin::matcher::prelude::*;

#[derive(Clone)]
pub struct DriftingBottle;

#[async_trait]
impl Handler<MessageEvent> for DriftingBottle {
    on_command!(MessageEvent, "漂流瓶");
    async fn handle(&self, _event: MessageEvent, matcher: Matcher<MessageEvent>) {
        matcher.send_text("測試...").await;
    }
}

pub fn drifting_bottle() -> Matcher<MessageEvent> {
    Matcher::new("DriftingBottle", DriftingBottle)
        .add_pre_matcher(prematchers::option_command_start())
}
