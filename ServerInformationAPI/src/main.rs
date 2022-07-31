#![allow(non_snake_case)]
use anyhow::Result;
use ServerInformationAPI::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}
