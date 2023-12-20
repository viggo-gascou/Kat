mod utils;
use kat::cli::parse;
use color_eyre::{eyre, eyre::Context, Report};


#[tokio::main]
async fn main() -> Result<(), Report> {
    // Setup the default panic and error report hooks for color_eyre
    color_eyre::install()?;

    let args = parse();
    kat::run(args).await;

    Ok(())
}