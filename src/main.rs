mod settings;

use settings::Settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();
    run().await
}

async fn run() -> anyhow::Result<()> {
    let settings = Settings::new()?;
    println!("{:?}", settings);

    Ok(())
}

fn init_logger() {
    let log_filters = std::env::var("RUST_LOG").unwrap_or_default();

    env_logger::Builder::new()
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