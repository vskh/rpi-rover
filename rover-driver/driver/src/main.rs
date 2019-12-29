use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load settings
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Config"))?;

    // bind socket
    let mut listener = TcpListener::bind(settings.get_str("listen")?).await?;

    Ok(())
}
