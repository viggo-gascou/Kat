pub mod cli;
mod utils;
mod commands;

use color_eyre::Result;

#[derive(Debug)]
pub struct App {
    args: cli::Cli,
    config: utils::config::Config,
    http_client: utils::webutils::HttpClient,
}

pub async fn run(args: cli::Cli) {
    let verbose = args.verbose;

    let result = attempt_run(args).await;

    exit_on_err(result, verbose);
}


async fn attempt_run(args: cli::Cli) -> crate::Result<()>{ 
    use cli::Commands::*;

    let config = utils::config::Config::new()?;
    let http_client = utils::webutils::HttpClient::new().unwrap();

    let app = App { args, config, http_client };

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
            eprintln!("{}: {e:?}", "Error");
        } else {
            eprintln!("{}: {e}", "Error");
            eprintln!();
            eprintln!("Run with --verbose for more information");
        }

        std::process::exit(1);
    }
}