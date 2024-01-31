pub mod cli;
mod commands;
mod utils;
use cli::parse_cli;
use utils::AppConfig;

use color_eyre::{Report, Result};

use colored::Colorize;

#[derive(Debug)]
pub struct App {
    args: cli::Cli,
    config: AppConfig,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    // Setup the default panic and error report hooks for color_eyre
    color_eyre::install()?;

    let args = parse_cli();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    run(args).await;

    Ok(())
}

pub async fn run(args: cli::Cli) {
    let verbose = args.verbose.clone();

    let result = attempt_run(args).await;

    exit_on_err(result, verbose);
}

async fn attempt_run(args: cli::Cli) -> crate::Result<()> {
    use cli::Commands::*;

    // If init subcommand, we want to run it without loading the config
    // as the user likely does not have a config file yet
    if let Init(args) = &args.subcommand {
        commands::init(args).await
    } else {
        let config = AppConfig::load()?;
        let app = App { args, config };

        match &app.args.subcommand {
            Config(args) => commands::config(&app, args).await,
            Get(args) => commands::get(&app, args).await,
            Open(args) => commands::open(&app, args).await,
            Submit(args) => commands::submit(&app, args).await,
            Test(args) => commands::test(&app, args).await,
            Watch(args) => commands::watch(&app, args).await,
            // This should never happen, as we catch it earlier ^^
            Init(_) => unreachable!(),
        }
    }
}

fn exit_on_err(res: crate::Result<()>, verbose: clap_verbosity_flag::Verbosity) {
    if let Err(e) = res {
        match verbose.log_level() {
            Some(log::Level::Error) => {
                // If Error (default), we want to print a short error report
                eprintln!("{}", format!("Error: {e}").bright_red());
            }
            Some(_) => {
                // If the user has specified verbose output, we want to print a more detailed error report
                eprintln!("{}", format!("Error: {e:?}").bright_red());
            }
            None => {
                // If None the user has set quiet output, so no error report is printed
            }
        }

        std::process::exit(1);
    }
}
