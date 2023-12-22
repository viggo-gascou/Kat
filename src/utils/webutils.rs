use crate::{utils::Config, App};

use color_eyre::{eyre, eyre::Context, Report};
use regex::Regex;
use reqwest::{Client, StatusCode};

const USER_AGENT: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/", env!("CARGO_PKG_VERSION"));

pub fn is_problem_id(problem: &str) -> bool {
    /* Matches any alphanumeric character or period - so that it can match
    problem IDs like 'itu.seatallocation' or 'twosum' */
    Regex::new(r"^[a-zA-Z0-9\.]+$").unwrap().is_match(problem)
}

pub fn get_problem_url(problem: &str) -> String {
    let host_name = Config::load().unwrap().kattisrc.kattis.hostname;
    format!("https://{}/problems/{}", host_name, problem)
}

pub fn get_problem_url_from_hostname(problem: &str, hostname: &str) -> String {
    format!("https://{}/problems/{}", hostname, problem)
}

pub fn get_submission_url(submission_id: &str) -> String {
    let host_name = Config::load().unwrap().kattisrc.kattis.hostname;
    format!("https://{}/submissions/{}", host_name, submission_id)
}

pub fn get_sample_url(problem: &str) -> String {
    let host_name = Config::load().unwrap().kattisrc.kattis.hostname;
    format!(
        "https://{}/problems/{}/file/statement/samples.zip",
        host_name, problem
    )
}

pub async fn problem_exists(app: &App, problem: &str, hostname: &str) -> Result<bool, Report> {
    let problem_url = get_problem_url_from_hostname(problem, hostname);
    let response = app.http_client.client.get(&problem_url).send().await?;

    match response.status() {
        StatusCode::OK => Ok(true),
        StatusCode::NOT_FOUND => Ok(false),
        _ => {
            let status = response.status();
            eyre::bail!("🙀 Failed to get problem: {} - {}", problem, status)
        }
    }
}

pub fn check_change_hostname(app: &App, problem: &str) -> Result<String, Report> {
    // if the problem id contains a period, it is probably a custom problem
    // and we should change the hostname to the custom hostname
    // eg. The problem itu.seatallocation is hosted on itu.kattis.com instead of open.kattis.com
    // so we should change the hostname to itu.kattis.com before fetching the tests

    let hostname = &app.config.kattisrc.kattis.hostname;
    if problem.contains(".") {
        let problem_hostname = problem.split(".").nth(0).unwrap();

        println!(
            "\n👀 It looks like problem {} is hosted on {}.kattis.com instead of {}.",
            problem, problem_hostname, hostname
        );
        let change_hostname = dialoguer::Confirm::new()
            .with_prompt(format!(
                "Do you want to change the hostname to {}.kattis.com?",
                problem_hostname
            ))
            .interact()
            .unwrap();

        if change_hostname {
            Ok(format!("{}.kattis.com", problem_hostname))
        } else {
            Ok(hostname.to_string())
        }
    } else {
        Ok(hostname.to_string())
    }
}

#[derive(Debug)]
pub struct HttpClient {
    pub client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self, Report> {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .wrap_err("🙀 Failed to create http client")?;

        Ok(HttpClient { client })
    }
}
