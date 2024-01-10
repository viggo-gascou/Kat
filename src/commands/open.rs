use crate::{
    cli::Open,
    utils::webutils::{check_change_hostname, is_problem_id},
    App,
};
use color_eyre::{eyre, eyre::Context, Report};

use std::env;

pub async fn open(app: &App, args: &Open) -> Result<(), Report> {
    // first get the problem id from the current directory if it is not specified
    let problem_id = args.problem.as_ref().map_or_else(
        || -> Result<String, Report> {
            let current_dir = env::current_dir().wrap_err("🙀 Failed to get current directory!")?;
            let file_name = current_dir
                .file_name()
                .ok_or_else(|| eyre::eyre!("🙀 Failed to get file name from path"))?;

            Ok(file_name.to_string_lossy().to_string())
        },
        |problem| Ok(problem.clone()),
    )?;

    // then check if the problem id is valid
    if let Some(problem) = &args.problem {
        if !is_problem_id(problem) {
            eyre::bail!("Invalid problem id: {}", problem);
        }
    }

    let problem_url = check_change_hostname(app, &problem_id, "get")?;

    webbrowser::open(&problem_url)
        .wrap_err(format!("🌐 Website {} could not be opened!", problem_url))?;

    Ok(())
}
