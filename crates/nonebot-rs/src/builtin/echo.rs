use crate::builtin::matcher::prelude::*;

#[doc(hidden)]
#[derive(Clone)]
pub struct Echo {}

#[doc(hidden)]
#[async_trait]
impl Handler<MessageEvent> for Echo {
    on_command!(MessageEvent, "echo", "Echo");
    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let msg = Message::Text {
            text: event.get_raw_message().to_string(),
        };
        matcher.send(vec![msg]).await;
    }
}

/// 单次复读 Matcher
pub fn echo() -> Matcher<MessageEvent> {
    Matcher::new("Echo", Echo {})
        .add_pre_matcher(prematchers::to_me())
        .add_pre_matcher(prematchers::command_start())
}
