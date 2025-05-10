use anyhow::Context;
use backend::http;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    http::serve().await?;

    Ok(())
}
