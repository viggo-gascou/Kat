use crate::{
    cli::Get,
    utils::fileutils::{copy_template, get_problem_dir, get_test_dir},
    utils::webutils::{
        check_change_hostname, get_problem_url_from_hostname, is_problem_id, problem_exists,
    },
    App,
};
use color_eyre::{eyre, eyre::Context, Report};
use std::{
    fs::{self},
    io::Write,
};

pub async fn get(app: &App, args: &Get) -> Result<(), Report> {
    let problem = &args.problem;
    if !is_problem_id(problem) {
        eyre::bail!("🙀 Invalid problem id: {}!", problem);
    }

    let problem_dir = get_problem_dir(app, problem)?;
    if problem_dir.exists() {
        println!(
            "👀 Looks like the problem {} has already been fetched!",
            problem
        );
        return Ok(());
    } else {
        std::fs::create_dir(&problem_dir)
            .wrap_err("🙀 Failed to create problem directory at this location")?;
    }

    let hostname = check_change_hostname(app, problem)?;

    if !problem_exists(app, problem, &hostname).await? {
        eyre::bail!("🙀 Problem {} does not exist!", problem);
    }

    fetch_tests(app, problem, &hostname).await?;

    copy_template(app, args, problem)?;

    println!("👍 Successfully initialised the problem {}!", problem);

    Ok(())
}

async fn fetch_tests(app: &App, problem: &str, hostname: &str) -> Result<(), Report> {
    let problem_url = get_problem_url_from_hostname(problem, &hostname);
    let sample_url = format!("{}/file/statement/samples.zip", problem_url);
    let mut tmpfile = tempfile::tempfile().wrap_err("🙀 Failed to create temporary file")?;

    let mut response = app.http_client.client.get(&sample_url).send().await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let test_dir = get_test_dir(app, problem)?;

            fs::create_dir_all(&test_dir).wrap_err("🙀 Failed to create tests directory")?;

            while let Some(chunk) = response.chunk().await? {
                tmpfile
                    .write_all(&chunk)
                    .wrap_err("🙀 Failed to write to samples.zip")?;
            }

            let mut zip =
                zip::ZipArchive::new(tmpfile).wrap_err("🙀 Failed to create zip archive")?;
            zip.extract(&test_dir)
                .wrap_err("🙀 Failed to extract samples.zip")?;

            return Ok(());
        }
        reqwest::StatusCode::NOT_FOUND => {
            println!("🤷 It seems that this problem does not have any test files!");
            return Ok(());
        }
        _ => {
            let status = response.status();
            eyre::bail!("🙀 Failed to get problem: {} - {}", problem, status)
        }
    }
}
