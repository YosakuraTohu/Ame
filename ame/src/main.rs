use ame::plugins::{moli::Moli, msg_saver::MsgSaver};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[tokio::main]
async fn main() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);
    let file_appender = rolling::hourly("logs", "ame.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);
    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(file_layer)
        .init();
    let mut nonebot = nonebot_rs::Nonebot::new();
    let mut matchers = nonebot_rs::Matchers::new_empty();
    let moli = Moli::new();
    matchers
        .add_message_matcher(nonebot_rs::builtin::bot_status::bot_status())
        .add_message_matcher(ame::matchers::lolicon::lolicon());
    nonebot
        .add_plugin(nonebot_rs::Logger)
        .add_plugin(matchers)
        .add_plugin(moli)
        .add_plugin(MsgSaver);
    nonebot.run().await
}
