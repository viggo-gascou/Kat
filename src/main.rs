pub mod cli;
mod commands;
mod utils;
use cli::parse_cli;
use color_eyre::{Report, Result};

use crate::utils::{config, webutils};

#[derive(Debug)]
pub struct App {
    args: cli::Cli,
    config: config::Config,
    http_client: webutils::HttpClient,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    // Setup the default panic and error report hooks for color_eyre
    color_eyre::install()?;

    let args = parse_cli();
    run(args).await;

    Ok(())
}

pub async fn run(args: cli::Cli) {
    let verbose = args.verbose;

    let result = attempt_run(args).await;

    exit_on_err(result, verbose);
}

async fn attempt_run(args: cli::Cli) -> crate::Result<()> {
    use cli::Commands::*;

    let config = config::Config::load()?;
    let http_client = webutils::HttpClient::new().unwrap();

    let app = App {
        args,
        config,
        http_client,
    };

    match &app.args.subcommand {
        Config(args) => commands::config(&app, args).await,
        Get(args) => commands::get(&app, args).await,
        Open(args) => commands::open(&app, args).await,
        Submit(args) => commands::submit(&app, args).await,
        Test(args) => commands::test(&app, args).await,
        Watch(args) => commands::watch(&app, args).await,
    }
}

fn exit_on_err(res: crate::Result<()>, verbose: bool) {
    if let Err(e) = res {
        if verbose {
            eprintln!("Error: {e:?}");
        } else {
            eprintln!("Error: {e}");
            eprintln!();
            eprintln!("Run with --verbose for more information");
        }

        std::process::exit(1);
    }
}
