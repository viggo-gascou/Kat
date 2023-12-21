use crate::utils::config::Config;

use color_eyre::{eyre, eyre::Context, Report};
use regex::Regex;
use reqwest::Client;

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

pub fn get_submission_url(submission_id: &str) -> String {
    let host_name = Config::load().unwrap().kattisrc.kattis.hostname;
    format!("https://{}/submissions/{}", host_name, submission_id)
}

pub fn get_sample_url(problem: &str) -> String {
    let host_name = Config::load().unwrap().kattisrc.kattis.hostname;
    format!("https://{}/problems/{}/file/statement/samples.zip", host_name, problem)
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
