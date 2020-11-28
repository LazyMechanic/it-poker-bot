mod settings;

use settings::Settings;
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();
    run().await
}

async fn run() -> anyhow::Result<()> {
    let settings = Settings::new()?;
    println!("{:?}", settings);

    let bot = Bot::builder()
        .token(settings.telegram.token)
        .build();

    teloxide::repl(bot, |message| async move {
        message.answer_dice().send().await?;
        ResponseResult::<()>::Ok(())
    }).await;

    Ok(())
}

fn init_logger() {
    let log_filters = std::env::var("RUST_LOG").unwrap_or_default();

    pretty_env_logger::formatted_builder()
        .parse_filters(&log_filters)
        .format(|formatter, record| {
            use std::io::Write;
            writeln!(
                formatter,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init()
}