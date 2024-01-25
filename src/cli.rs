use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueHint};
use clap_verbosity_flag::Verbosity;

pub fn parse_cli() -> Cli {
    Cli::parse()
}

const WELCOME: &str = concat!("Welcome to kat! - v", env!("CARGO_PKG_VERSION"), "\n");

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = format!("{WELCOME}
kat is a CLI tool for getting, submitting, testing and interacting with the programming problem solving website Kattis.
To get started have a look at the commands below, or run `kat help <COMMAND>` for more information about a specific command."),
)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Commands,

    #[command(flatten)]
    pub verbose: Verbosity,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Commands to help you configure kat")]
    Config(Config),
    #[command(about = "Get a problem from kattis")]
    Get(Get),
    #[command(about = "Initialise the configuration files")]
    Init(Init),
    #[command(about = "Open a problem in the browser")]
    Open(Open),
    #[command(about = "Submit a problem to kattis")]
    Submit(Submit),
    #[command(about = "Test a problem against its test case(s)")]
    Test(Test),
    #[command(
        about = "Watch a problem file for changes, and automatically test it when it changes"
    )]
    Watch(Watch),
}

#[derive(Args, Debug)]
pub struct Config {
    #[command(subcommand)]
    pub subcommand: ConfigCommands,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    #[command(about = "Locate the configuration files.")]
    Locate,
    #[command(about = "Set the location of where the configuration files are stored.")]
    Set(SetLocation),
}

#[derive(Args, Debug)]
pub struct SetLocation {
    #[arg(value_hint = ValueHint::DirPath,
        help = "The new location for the configuration files.")]
    pub path: PathBuf,
}

#[derive(Args, Debug)]
pub struct Get {
    #[arg(help = "The id of the problem you want to download from kattis. 
If you don't know the id, you can find it from the url of the problem, i.e. https://open.kattis.com/problems/<PROBLEM_ID>")]
    pub problem: String,
    #[arg(
        short,
        long,
        value_hint = ValueHint::DirPath,
        help = "The path where you want to download the problem. 
If not specified, the problem will be downloaded to the current directory."
    )]
    pub path: Option<PathBuf>,
    #[arg(
        short,
        long,
        help = "The programming language to setup the problem for.
If not specified, the language will be determined default language in the configuration file."
    )]
    pub language: Option<String>,
}

#[derive(Args, Debug)]
pub struct Init {
    #[arg(
        help = "Shortcut to specify which files to download. Options are 'all' or 'config'. 
'config' will only download the config file, while 'all' will download all available config files.
If not specified, you will be prompted to choose which files."
    )]
    pub choice: Option<String>,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "If set, you will not be prompted to confirm whether you want to overwrite existing config files."
    )]
    pub yes: bool,
}

#[derive(Args, Debug)]
pub struct Open {
    #[arg(help = "The id of the problem you want to open in the browser.")]
    pub problem: Option<String>,
}

#[derive(Args, Debug)]
pub struct Submit {
    #[arg(
        default_value = ".",
        value_hint = ValueHint::DirPath,
        help = "The path of the problem (folder) you want to submit. By default, the current directory is used."
    )]
    pub path: PathBuf,
    #[arg(
        short,
        long,
        value_hint = ValueHint::FilePath,
        help = "The path of the solution file to submit. 
If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use."
    )]
    pub file: Option<PathBuf>,
    #[arg(
        short,
        long,
        help = "The programming language to test the problem against. 
This can be used to override the default language set in the configuration file."
    )]
    pub language: Option<String>,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "If set, the the problem will be run against all of the local test cases before submitting.
If the problem fails any of the test cases, the submission will be aborted."
    )]
    pub test_first: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "If set, you will not be prompted to confirm the submission before it is sent to kattis."
    )]
    pub yes: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "If set, the submission will be opened in the browser after all tests have been run on the kattis server."
    )]
    pub open: bool,
}

#[derive(Args, Debug)]
pub struct Test {
    #[arg(
        default_value = ".",
        value_hint = ValueHint::DirPath,
        help = "The path of the problem (folder) you want to test. By default, the current directory is used."
    )]
    pub path: PathBuf,
    #[arg(
        short,
        long,
        value_hint = ValueHint::FilePath,
        help = "The path of the solution file to test. If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use."
    )]
    pub file: Option<PathBuf>,
    #[arg(
        short,
        long,
        help = "The id(s) of the test case(s) to test against. 
If not specified, all test cases will be tested, e.g. '1', '1-3', or '1,3-5'."
    )]
    pub test_cases: Option<String>,
    #[arg(
        short,
        long,
        help = "The programming language to test the problem against. 
This can be used to override the default language set in the configuration file."
    )]
    pub language: Option<String>,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "If set, try to submit the problem to kattis if all tests pass."
    )]
    pub submit: bool,
}

#[derive(Args, Debug)]
pub struct Watch {
    #[arg(
        default_value = ".",
        value_hint = ValueHint::DirPath,
        help = "The path of the problem (folder) you want to watch."
    )]
    pub path: PathBuf,
    #[arg(
        short,
        long,
        value_hint = ValueHint::FilePath,
        help = "The path of the solution file to watch. 
If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use."
    )]
    pub file: Option<PathBuf>,
    #[arg(
        short,
        long,
        help = "The id(s) of the test case(s) to test against.
If not specified, all test cases will be tested, e.g. '1', '1-3', or '1,3-5'."
    )]
    pub test_cases: Option<String>,
    #[arg(
        short,
        long,
        help = "The programming language to test the problem against. 
This can be used to override the default language set in the configuration file."
    )]
    pub language: Option<String>,
}
