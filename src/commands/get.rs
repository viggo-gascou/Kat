use crate::{
    cli::Get,
    utils::fileutils::{copy_template, get_problem_dir, get_test_dir},
    utils::webutils::{
        check_change_hostname, get_sample_url_from_problem_url, is_problem_id, problem_exists,
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
    let url = check_change_hostname(app, problem, "get")?;
    let hostname = url
        .trim_start_matches("https://")
        .split('/')
        .next()
        .ok_or_else(|| eyre::eyre!("🙀 Failed to extract hostname from URL"))?;

    if hostname != "open.kattis.com" {
        let login_url = format!("https://{}/login", hostname);
        app.http_client.login(app, &login_url).await?;
    }

    if !problem_exists(app, problem, hostname).await? {
        eyre::bail!("🙀 Problem {} does not exist!", problem);
    }

    let problem_dir = get_problem_dir(app, problem)?;
    if problem_dir.exists() {
        println!(
            "👀 Looks like the problem {} has already been fetched!",
            problem
        );
        let overwrite = dialoguer::Confirm::new()
            .with_prompt("Do you want to get it again? (Careful this will overwrite the existing problem directory!)")
            .interact()
            .wrap_err("🙀 Failed to get user input")?;
        if overwrite {
            std::fs::remove_dir_all(&problem_dir)
                .wrap_err("🙀 Failed to remove existing problem directory")?;
            std::fs::create_dir(&problem_dir)
                .wrap_err("🙀 Failed to create problem directory at this location")?;
        } else {
            println!("{}", &problem_dir.display());
            println!("👍 Ok, not fetching the problem {problem}!");
            return Ok(());
        }
    } else {
        std::fs::create_dir(&problem_dir)
            .wrap_err("🙀 Failed to create problem directory at this location")?;
    }

    println!("📥 Fetching problem {} from {}...", problem, url);

    fetch_tests(app, problem, &url).await?;

    println!("📝 Creating template file for problem {}...", problem);

    copy_template(app, &args.language, problem)?;

    println!("👍 Successfully initialised the problem {}!", problem);

    Ok(())
}

async fn fetch_tests(app: &App, problem: &str, problem_url: &str) -> Result<(), Report> {
    let sample_url = get_sample_url_from_problem_url(problem_url);
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

            Ok(())
        }
        reqwest::StatusCode::NOT_FOUND => {
            println!("🤷 It seems that this problem does not have any test files!");
            Ok(())
        }
        _ => {
            let status = response.status();
            eyre::bail!("🙀 Failed to get problem: {} - {}", problem, status)
        }
    }
}
