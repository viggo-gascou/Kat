use std::{fs, io::Write, path::PathBuf};

use crate::{
    cli::Submit,
    commands::test::test_problem,
    utils::{
        check_change_hostname, find_problem_dir, find_test_files, get_problem_file,
        get_submissions_url_from_hostname, get_submit_url_from_hostname, problem_exists,
        HttpClient,
    },
    App,
};

use color_eyre::{
    eyre::{self, Context, ContextCompat},
    Report,
};

use regex::Regex;
use reqwest::multipart::{Form, Part};
use scraper::{Html, Selector};

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

#[derive(Debug, PartialEq, Eq, Clone)]
enum SubmissionStatus {
    Setup(String),
    Running,
    Accepted,
    Failed(String),
    Unknown,
}

impl SubmissionStatus {
    fn from_str(status: &str) -> Self {
        match status {
            "New" | "Compiling" => Self::Setup(status.to_string()),
            "Running" => Self::Running,
            "Accepted" | "Accepted (100) " => Self::Accepted,
            "Run Time Error"
            | "Time Limit Exceeded"
            | "Compile Error"
            | "Memory Limit Exceeded"
            | "Output Limit Exceeded"
            | "Wrong Answer"
            | "Judge Error" => Self::Failed(status.to_string()),
            _ => Self::Unknown,
        }
    }

    fn emoji(&self) -> &str {
        match self {
            Self::Setup(status) => match status.as_str() {
                "New" => "🆕",
                "Compiling" => "🛠️",
                _ => "",
            },
            Self::Running => "🏃",
            Self::Accepted => "✅",
            Self::Failed(status) => match status.as_str() {
                "Run Time Error" => "💥",
                "Time Limit Exceeded" => "⌛️",
                "Compile Error" => "🆘",
                "Memory Limit Exceeded" => "🧠",
                "Output Limit Exceeded" => "🌊",
                "Wrong Answer" => "💔",
                "Judge Error" => "🔮",
                _ => "",
            },
            Self::Unknown => "",
        }
    }

    fn is_finished(&self) -> bool {
        matches!(self, Self::Accepted | Self::Failed(_))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum TestStatus {
    Accepted,
    Failed,
    Running,
}

impl TestStatus {
    fn from_str(status: &str) -> Self {
        match status {
            "Accepted" => Self::Accepted,
            "Wrong Answer" => Self::Failed,
            _ => Self::Running,
        }
    }

    fn emoji(&self) -> &str {
        match self {
            Self::Accepted => "🟢",
            Self::Failed => "🔴",
            Self::Running => "⚪",
        }
    }

    fn is_finished(&self) -> bool {
        matches!(self, Self::Accepted | Self::Failed)
    }
}

pub async fn submit(app: &App, args: &Submit) -> Result<(), Report> {
    let (problem_path, problem_id) = find_problem_dir(app, &args.path)?;
    let (problem_file, problem_file_path, language) =
        get_problem_file(app, &args.file, &args.language, &problem_path, &problem_id)?;
    let submission = Submission {
        problem_id: problem_id.clone(),
        language: &language,
        problem_file: problem_file.clone(),
        problem_file_path: problem_file_path.clone(),
    };
    let should_submit;

    if problem_file_path.exists() {
        if args.test_first {
            let tests = find_test_files(app, &Some("all".to_string()), &problem_path)?;
            println!(
                "👀 Testing the file {} for the problem {} ...\n",
                &problem_file, problem_id
            );
            if !test_problem(
                app,
                &problem_id,
                &problem_path,
                &problem_file_path,
                tests,
                &language,
            )? {
                eyre::bail!("❌ Some tests seem to have failed, aborting submission!");
            }
        }
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
                "🚀 Submitting problem: {} with the file {} ...",
                submission.problem_id, submission.problem_file
            );
            let http_client = HttpClient::new().unwrap();
            let submission_url = send_submission(app, submission, &http_client).await?;

            println!("👀 Watching submission ...\n");
            watch_submission(&http_client, &submission_url).await?;

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

pub async fn send_submission(
    app: &App,
    submission: Submission<'_>,
    http_client: &HttpClient,
) -> Result<String, Report> {
    let login_url = check_change_hostname(app, &submission.problem_id, "login")?;
    let hostname = login_url
        .trim_start_matches("https://")
        .split('/')
        .next()
        .ok_or_else(|| eyre::eyre!("🙀 Failed to extract hostname from URL"))?;
    let url = get_submit_url_from_hostname(hostname);

    if !problem_exists(http_client, &submission.problem_id, hostname).await? {
        // bail early if the problem does not exist
        eyre::bail!("🙀 Problem does not exist: {}", submission.problem_id);
    }

    http_client.login(app, &login_url).await?;

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

    let response = http_client
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
        .ok_or_else(|| eyre::eyre!("🙀 Failed to get submission ID - got: {}", body))?;

    Ok(get_submissions_url_from_hostname(hostname, submission_id))
}

async fn watch_submission(http_client: &HttpClient, submission_url: &str) -> Result<(), Report> {
    loop {
        let submission_data = parse_submission_data(http_client, submission_url).await?;
        let sub_status = SubmissionStatus::from_str(&submission_data.status);
        let emoji = sub_status.emoji();

        let num_tests = submission_data.tests.len();
        let mut emoji_bar = String::new();

        if let SubmissionStatus::Setup(_) = sub_status {
            print!("\rCurrent Stage: {} {} ...", submission_data.status, emoji);
            std::io::stdout()
                .flush()
                .wrap_err("🙀 Failed to flush stdout")?;
        }

        for (test_idx, test) in submission_data.tests.iter().enumerate() {
            let test_status = TestStatus::from_str(&test.status);

            emoji_bar.push_str(test_status.emoji());
            if let TestStatus::Running = test_status {
                print!("\nTesting {}/{} {}", test_idx + 1, num_tests, emoji_bar);
            } else {
                print!("\rTesting {}/{} {}", test_idx + 1, num_tests, emoji_bar);
            }
            std::io::stdout()
                .flush()
                .wrap_err("🙀 Failed to flush stdout")?;

            // Break if any of the tests fail as Kattis will not run the rest of the tests
            if test_status.is_finished() {
                break;
            }
        }

        // If the submission status is not "new, running or compiling" or any test case fails, break the loop
        if !sub_status.is_finished() {
            println!("\n");

            if ["Accepted", "Wrong answer"].contains(&submission_data.status.as_str()) {
                println!(
                    "Final Status: {} {} - {} tests passed in - {}",
                    submission_data.status,
                    sub_status.emoji(),
                    submission_data.testcases,
                    submission_data.cpu
                );
            } else if "Judge Error" == submission_data.status.as_str() {
                println!("{} The unexpected happened Kattis returned a Judge Error - you should probably contact them!", 
                sub_status.emoji());
            } else {
                println!(
                    "The submission failed without being accepted or directly rejected - status: {} {}",
                    submission_data.status, sub_status.emoji()
                );
            }
            break;
        }
        // wait 0.25 seconds before checking the submission status again to avoid spamming the server
        tokio::time::sleep(std::time::Duration::from_secs_f32(0.25)).await;
    }

    Ok(())
}

async fn parse_submission_data(
    http_client: &HttpClient,
    submission_url: &str,
) -> Result<SubmissionData, Report> {
    let response = http_client
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
