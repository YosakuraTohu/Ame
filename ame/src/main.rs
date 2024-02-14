use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::hourly("logs", "ame.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_writer(std::io::stdout)
        .init();
    let mut nonebot = nonebot_rs::Nonebot::new();
    let mut matchers = nonebot_rs::Matchers::new_empty();
    matchers
        .add_message_matcher(nonebot_rs::builtin::bot_status::bot_status())
        .add_message_matcher(ame::matchers::lolicon::lolicon());
    nonebot.add_plugin(nonebot_rs::Logger).add_plugin(matchers);
    nonebot.run().await
}
