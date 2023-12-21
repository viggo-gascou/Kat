use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

pub fn parse() -> Cli {
    Cli::parse()
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "kat - a simple CLI tool for interacting with kattis",
    long_about = "kat is a CLI tool for getting, 
submitting, testing and interacting with the programming problem solving website kattis - written in rust"
)]
pub struct Cli {
    #[arg(
        short,
        long,
        help = "If set, enables verbose output, which prints more information to the terminal.",
        default_value_t = false,
        global = true
    )]
    pub verbose: bool,

    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Configure kat")]
    Config(Config),
    #[command(about = "Get a problem from kattis")]
    Get(Get),
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
    #[command(about = "Initialise the configuration files.")]
    Init,
    #[command(about = "Locate the configuration files.")]
    Locate,
    #[command(about = "Set the location of where the configuration files are stored.")]
    Set(SetLocation),
}

#[derive(Args, Debug)]
pub struct SetLocation {
    #[arg(help = "The new location for the configuration files.")]
    pub path: String,
}

#[derive(Args, Debug)]
pub struct Get {
    #[arg(help = "The id of the problem you want to download from kattis. 
		If you don't know the id, you can find it from the url of the problem, i.e. https://open.kattis.com/problems/<PROBLEM_ID>")]
    problem: String,
    #[arg(
        short,
        long,
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
pub struct Open {
    #[arg(help = "The id of the problem you want to open in the browser.")]
    pub problem: Option<String>,
}

#[derive(Args, Debug)]
pub struct Submit {
    #[arg(
        default_value = ".",
        help = "The path of the problem (folder) you want to submit."
    )]
    pub path: PathBuf,
    #[arg(
        short,
        long,
        help = "The path of the solution file to submit. 
				If not specified, the first file with the same name as the problem in the problem folder will be used."
    )]
    pub file: Option<PathBuf>,
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
        help = "The path of the problem (folder) you want to test."
    )]
    pub path: PathBuf,
    #[arg(
        short,
        long,
        help = "The path of the solution file to test.
                If not specified, the first file with the same name as the problem in the problem folder will be used.
                If multiple files with the correct extension are found, you will be prompted to choose which one to use."
    )]
    pub problem: Option<PathBuf>,
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
        default_value_t = false,
        help = "If set, try to submit the problem to kattis if all tests pass."
    )]
    pub submit: bool,
}

#[derive(Args, Debug)]
pub struct Watch {
    #[arg(
        default_value = ".",
        help = "The path of the problem (folder) you want to watch."
    )]
    pub path: PathBuf,
    #[arg(
        short,
        long,
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
    pub test_case: Option<String>,
}
