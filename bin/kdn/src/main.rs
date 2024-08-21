use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    kdn::Cli::parse().init_telemetry()?.run().await
}
