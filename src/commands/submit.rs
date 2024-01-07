use std::{fs, io::Write, path::PathBuf};

use crate::{
    cli::Submit,
    utils::{
        check_change_hostname, find_problem_dir, get_problem_file,
        get_submissions_url_from_hostname, get_submit_url_from_hostname, problem_exists,
    },
    App,
};

use regex::Regex;
use scraper::{Html, Selector};

use color_eyre::{
    eyre::{self, ContextCompat, Context},
    Report,
};
use reqwest::multipart::{Form, Part};

#[derive(Debug)]
pub struct Submission<'a> {
    pub problem_id: String,
    pub language: &'a str,
    pub problem_file: String,
    pub problem_file_path: PathBuf,
}

#[derive(Debug)]
pub struct SubmissionData {
    pub submission_id: String,
    pub plagiarism: String,
    pub time: String,
    pub problem: String,
    pub status: String,
    pub cpu: String,
    pub lang: String,
    pub testcases: String,
    pub tests: Vec<SubmissionTest>,
}

#[derive(Debug)]
pub struct SubmissionTest {
    pub number: String,
    pub status: String,
}

pub async fn submit(app: &App, args: &Submit) -> Result<(), Report> {
    let (problem_path, problem_id) = find_problem_dir(app, &args.path)?;
    let (problem_file, problem_file_path, language) = get_problem_file(
        app,
        &args.problem,
        &args.language,
        &problem_path,
        &problem_id,
    )?;
    let submission = Submission {
        problem_id: problem_id.clone(),
        language: &language,
        problem_file: problem_file.clone(),
        problem_file_path: problem_file_path.clone(),
    };
    let should_submit;

    if problem_file_path.exists() {
        if args.yes {
            // If the user has specified the -y or --yes flag, we skip the confirmation dialog
            should_submit = true;
        } else {
            should_submit = dialoguer::Confirm::new()
                .with_prompt(format!(
                    "Do you want to submit this file ({})?",
                    submission.problem_file
                ))
                .interact()
                .wrap_err("🙀 Failed to get user input")?;
        }

        if should_submit {
            println!(
                "🚀 Submitting problem: {} with the file {} ...\n",
                submission.problem_id, submission.problem_file
            );
            let submission_url = send_submission(app, submission).await?;

            println!("👀 Watching submission ...\n");
            watch_submission(app, &submission_url).await?;

            if args.open {
                webbrowser::open(&submission_url).wrap_err(format!(
                    "🌐 Website {} could not be opened!",
                    submission_url
                ))?;
            }
            Ok(())
        } else {
            eyre::bail!("🙀 Submission aborted");
        }
    } else {
        eyre::bail!("🙀 Problem file does not exist: {}", problem_file);
    }
}

pub async fn send_submission(app: &App, submission: Submission<'_>) -> Result<String, Report> {
    let login_url = check_change_hostname(app, &submission.problem_id, "login")?;
    let hostname = login_url
        .trim_start_matches("https://")
        .split('/')
        .next()
        .ok_or_else(|| eyre::eyre!("🙀 Failed to extract hostname from URL"))?;
    let url = get_submit_url_from_hostname(hostname);

    if !problem_exists(app, &submission.problem_id, hostname).await? {
        // bail early if the problem does not exist
        eyre::bail!("🙀 Problem does not exist: {}", submission.problem_id);
    }

    app.http_client.login(app, &login_url).await?;

    let file_bytes = fs::read(&submission.problem_file_path)
        .wrap_err_with(|| format!("🙀 Failed to read file: {}", &submission.problem_file))?;

    let file = Part::bytes(file_bytes)
        .file_name(submission.problem_file.clone())
        .mime_str("application/octet-stream")?;

    let mainclass = submission.problem_file;

    let form = Form::new()
        .text("submit", "true")
        .text("submit_ctr", "2")
        .text("language", submission.language.to_string())
        .text("mainclass", mainclass)
        .text("problem", submission.problem_id)
        .text("script", "true")
        .part("sub_file[]", file);

    let response = app
        .http_client
        .client
        .post(url)
        .multipart(form)
        .send()
        .await
        .wrap_err("🙀 Failed to send submission request to kattis")?;

    let status = response.status();

    if !status.is_success() {
        if status.is_server_error() {
            eyre::bail!(
                "🙀 There was a problem with the Kattis server, it might be down. Error code: {}. Please try again later or check https://status.kattis.com/!",
                status,
            )
        } else if status.is_client_error() {
            eyre::bail!(
                "🙀 A client error occurred while trying to submit. Error code: {}. Please check your submission and try again!",
                status,
            )
        } else {
            eyre::bail!(
                "🙀 There was a problem with the submission, error code: {}",
                status,
            )
        }
    }

    let body = response
        .text()
        .await
        .wrap_err("🙀 Failed to get response body")?;

    let re = regex::Regex::new(r"Submission ID: (?<ID>[0-9]+)").unwrap();
    let submission_id = re
        .captures(&body)
        .and_then(|cap| cap.name("ID"))
        .map(|m| m.as_str())
        .ok_or_else(|| eyre::eyre!("🙀 Failed to get submission ID - got {}", body))?;

    Ok(get_submissions_url_from_hostname(hostname, submission_id))
}

async fn watch_submission(app: &App, submission_url: &str) -> Result<(), Report> {
    let mut running = false;
    let mut current_stage = String::new();

    let status_emoji = |status: &str| match status {
        "New" => "🆕",
        "Compiling" => "🔨",
        "Running" => "🏃",
        "Accepted" => "✅",
        "Wrong Answer" => "❌",
        _ => "🙀",
    };

    loop {
        let submission_data = parse_submission_data(app, submission_url).await?;

        let num_tests = submission_data.tests.len();
        let mut emoji_bar = String::new();

        if submission_data.status != current_stage {
            current_stage = submission_data.status.clone();
            print!(
                "\rCurrent Stage: {} {} ...",
                current_stage,
                status_emoji(&current_stage)
            );
            std::io::stdout()
                .flush()
                .wrap_err("🙀 Failed to flush stdout")?;
            if current_stage.to_lowercase() == "running" {
                running = true;
            }
        }
        for (test_idx, test) in submission_data.tests.iter().enumerate() {
            let emoji = match test.status.as_str() {
                "Accepted" => "🟢",
                "Wrong Answer" => "🔴",
                _ => "⚪️",
            };
            emoji_bar.push_str(emoji);
            if running {
                print!("\nTesting {}/{} {}", test_idx + 1, num_tests, emoji_bar);
                running = false;
            } else {
                print!("\rTesting {}/{} {}", test_idx + 1, num_tests, emoji_bar);
            }
            std::io::stdout()
                .flush()
                .wrap_err("🙀 Failed to flush stdout")?;
        }

        // If the submission status is not "new, running or compiling" or the first test case fails, break the loop
        // because they are the only statuses that indicate that the submission is still running - that I know of
        if !["new", "running", "compiling"]
            .contains(&submission_data.status.to_lowercase().as_str())
            || (submission_data
                .tests
                .get(0)
                .map_or(false, |test| test.status == "Wrong Answer"))
        {
            println!("\n");

            if ["accepted", "wrong_answer"]
                .contains(&submission_data.status.to_lowercase().as_str())
            {
                println!(
                    "Final Status: {} {} - {} tests passed in - {}",
                    submission_data.status,
                    status_emoji(&submission_data.status),
                    submission_data.testcases,
                    submission_data.cpu
                );
            } else {
                println!(
                    "The submission failed without being accepted or directly rejected - status: {} {}",
                    submission_data.status, status_emoji(&submission_data.status)
                );
            }
            // print the final status along with how many tests passed out of the total as well as the cpu time

            break;
        }

        tokio::time::sleep(std::time::Duration::from_secs_f32(0.25)).await;
    }

    Ok(())
}

async fn parse_submission_data(app: &App, submission_url: &str) -> Result<SubmissionData, Report> {
    let response = app
        .http_client
        .client
        .get(submission_url)
        .send()
        .await
        .wrap_err("🙀 Failed to get submission from kattis")?;

    if !response.status().is_success() {
        eyre::bail!(
            "🙀 Failed to get submission, error code: {}",
            response.status()
        )
    }

    let body = response
        .text()
        .await
        .wrap_err("🙀 Failed to get response body")?;
    let document = Html::parse_document(&body);
    let judge_table = document
        .select(&Selector::parse("table#judge_table").unwrap())
        .next()
        .wrap_err("🙀 No submission table found!")?;
    let tests_selector = Selector::parse("tr.testcases-row i").unwrap();
    let data_selector = Selector::parse("tr[data-submission-id]").unwrap();

    let data_row = judge_table
        .select(&data_selector)
        .next()
        .wrap_err("🙀 No data row found")?;

    let mut submission_data = SubmissionData {
        submission_id: data_row
            .value()
            .attr("data-submission-id")
            .wrap_err("🙀 No submission id found")?
            .to_string(),
        plagiarism: String::new(),
        time: String::new(),
        problem: String::new(),
        status: String::new(),
        cpu: String::new(),
        lang: String::new(),
        testcases: String::new(),
        tests: Vec::new(),
    };

    let td_selector = Selector::parse("td[data-type]").unwrap();
    let tds = data_row.select(&td_selector);
    for td in tds {
        let data_type = td.value().attr("data-type").unwrap();
        let data_value = td.inner_html();
        match data_type {
            "plagiarism" => submission_data.plagiarism = data_value,
            "time" => submission_data.time = data_value,
            "problem" => {
                let re = Regex::new(r#"<a href=".*">(.*)</a>"#).unwrap();
                submission_data.problem = re
                    .captures(&data_value)
                    .wrap_err("🙀 Could not find problem name from submission table")?
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_string();
            }
            "status" => {
                let re = Regex::new(r"<span>(.*)</span>").unwrap();
                submission_data.status = re
                    .captures(&data_value)
                    .wrap_err("🙀 Could not find submission status from submission table")?
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_string();
            }
            "cpu" => submission_data.cpu = data_value.replace("&nbsp;", " "),
            "lang" => submission_data.lang = data_value,
            "testcases" => {
                let re = Regex::new(r#"<div class="horizontal_item">(.*)</div>"#).unwrap();
                submission_data.testcases = re
                    .captures(&data_value)
                    .wrap_err("🙀 Could not find test cases from submission table")?
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_string();
            }
            _ => (),
        }
    }

    let tests = judge_table.select(&tests_selector);
    for test in tests {
        let title = test.value().attr("title").unwrap().to_string();
        let re = Regex::new(r"Test case (\d+)/\d+: (.*)").unwrap();
        let captures = re
            .captures(&title)
            .wrap_err("🙀 Could not find test title element")?;
        let number = captures
            .get(1)
            .wrap_err("🙀 Could not find test number from test title")?
            .as_str()
            .to_string();
        let status = captures
            .get(2)
            .wrap_err("🙀 Could not find test status from test title")?
            .as_str()
            .to_string();
        let test_data = SubmissionTest { number, status };
        submission_data.tests.push(test_data);
    }

    Ok(submission_data)
}
