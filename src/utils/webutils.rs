use crate::App;

use color_eyre::{eyre, eyre::Context, Report};

use regex::Regex;
use reqwest::{multipart::Form, Client, StatusCode};
use secrecy::ExposeSecret;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/", env!("CARGO_PKG_VERSION"));

pub fn is_problem_id(problem: &str) -> bool {
    /* Matches any alphanumeric character or period - so that it can match
    problem IDs like 'itu.seatallocation' or 'twosum' */
    Regex::new(r"^[a-zA-Z0-9\.]+$").unwrap().is_match(problem)
}

pub fn get_login_url_from_hostname(hostname: &str) -> String {
    format!("https://{}/login", hostname)
}

pub fn get_problem_url_from_hostname(problem: &str, hostname: &str) -> String {
    format!("https://{}/problems/{}", hostname, problem)
}

pub fn get_submissions_url_from_hostname(host_name: &str, submission_id: &str) -> String {
    format!("https://{}/submissions/{}", host_name, submission_id)
}

pub fn get_submit_url_from_hostname(host_name: &str) -> String {
    format!("https://{}/submit", host_name)
}

pub fn get_sample_url_from_problem_url(problem_url: &str) -> String {
    format!("{}/file/statement/samples.zip", problem_url)
}

pub async fn problem_exists(
    http_client: &HttpClient,
    problem: &str,
    hostname: &str,
) -> Result<bool, Report> {
    let problem_url = get_problem_url_from_hostname(problem, hostname);
    let response = http_client.client.get(&problem_url).send().await?;

    match response.status() {
        StatusCode::OK => Ok(true),
        StatusCode::NOT_FOUND => Ok(false),
        _ => {
            let status = response.status();
            eyre::bail!("🙀 Failed to get problem: {} - {}", problem, status)
        }
    }
}

pub fn check_change_hostname(app: &App, problem: &str, url_type: &str) -> Result<String, Report> {
    // if the problem id contains a period, it is probably a custom problem
    // and we should change the hostname to the custom hostname
    // eg. The problem itu.seatallocation is hosted on itu.kattis.com instead of open.kattis.com
    // so we should change the hostname to itu.kattis.com before fetching the tests
    let mut hostname = app.config.kattisrc.kattis.hostname.clone();
    if problem.contains('.') {
        let problem_hostname = problem.split('.').next().unwrap();

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
            hostname = format!("{}.kattis.com", problem_hostname);
        }
    }
    if url_type == "get" {
        Ok(get_problem_url_from_hostname(problem, &hostname))
    } else if url_type == "submit" {
        Ok(get_submit_url_from_hostname(&hostname))
    } else if url_type == "submissions" {
        Ok(get_submissions_url_from_hostname(&hostname, problem))
    } else if url_type == "sample" {
        Ok(get_sample_url_from_problem_url(
            &get_problem_url_from_hostname(problem, &hostname),
        ))
    } else if url_type == "login" {
        Ok(get_login_url_from_hostname(&hostname))
    } else {
        eyre::bail!("🙀 Invalid url type: {}", url_type)
    }
}

#[derive(Debug)]
pub struct HttpClient {
    pub client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self, Report> {
        let client = Client::builder()
            .cookie_store(true)
            .user_agent(USER_AGENT)
            .build()
            .wrap_err("🙀 Failed to create http client")?;

        Ok(HttpClient { client })
    }

    pub async fn login(&self, app: &App, login_url: &str) -> Result<(), Report> {
        let username = app.config.kattisrc.user.username.clone();
        let token = app.config.kattisrc.user.token.expose_secret().clone();

        let form = Form::new()
            .text("script", "true")
            .text("user", username)
            .text("token", token);

        let response = self
            .client
            .post(login_url)
            .multipart(form)
            .send()
            .await
            .wrap_err("🙀 Failed to send login request to kattis")?;

        let status = response.status();
        if status == StatusCode::OK {
            Ok(())
        } else if status == StatusCode::FORBIDDEN {
            eyre::bail!("🙀 Invalid username or token - please check your credentials in the kattisrc file!")
        } else {
            eyre::bail!("🙀 Failed to login to kattis, error: {}", status,)
        }
    }
}
