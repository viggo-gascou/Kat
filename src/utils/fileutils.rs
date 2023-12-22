use std::path::{Path, PathBuf};

use color_eyre::{
    eyre,
    eyre::{Context, ContextCompat},
    Report,
};

use crate::{cli::Get, App};

pub fn get_problem_dir(_app: &App, problem: &str) -> Result<PathBuf, Report> {
    let current_dir = std::env::current_dir().wrap_err("🙀 Failed to get current directory")?;
    let problem_dir = current_dir.join(problem);

    Ok(problem_dir)
}

pub fn get_test_dir(app: &App, problem: &str) -> Result<PathBuf, Report> {
    let problem_dir = get_problem_dir(app, problem).wrap_err("🙀 Failed to get problem directory")?;
    let test_dir = problem_dir.join("tests");
    Ok(test_dir)
}

pub fn copy_template(app: &App, args: &Get, problem: &str) -> Result<(), Report> {
    let problem_dir = get_problem_dir(app, problem)?;
    let template_dir = PathBuf::from(format!(
        "{}/templates",
        app.config.internal_config.config_location
    ));
    let config = &app.config.kat_config;

    let language = match &args.language {
        Some(lang) => {
            if config.languages.contains_key(lang) {
                lang
            } else {
                eyre::bail!("🙀 Invalid language: {}", lang);
            }
        }
        None => &config.default.language,
    };

    let template_path = match config.languages.get(language) {
        Some(lang) => {
            if let Some(template_file) = &lang.template {
                // join the template file with the template directory to get the full path to the template file
                Some(template_dir.join(template_file))
            } else {
                println!("🙀 No template file found for language: {}", language);
                None
            }
        }
        None => {
            println!("🙀 No template file found for language: {}", language);
            None
        }
    };

    // if there is a template file, copy it to the problem directory and rename it to the problem id
    if let Some(template_path) = template_path {
        if !Path::new(&template_path).exists() {
            eyre::bail!(
                "🙀 Template file does not exist: {}",
                template_path.display()
            );
        }
        let template_file_name = template_path
            .file_name()
            .wrap_err("🙀 Failed to get file name from path")?;
        let template_file_no_ext = template_path
            .file_stem()
            .wrap_err("🙀 Failed to get file name from path")?
            .to_str()
            .wrap_err("🙀 Failed to convert file name to string")?
            .to_string();
        let template_file_name = template_file_name
            .to_str()
            .wrap_err("🙀 Failed to convert file name to string")?
            .to_string();
        let problem_file_name = template_file_name.replace(&template_file_no_ext, problem);
        let problem_file_path = problem_dir.join(problem_file_name);

        std::fs::copy(template_path, problem_file_path)
            .wrap_err("🙀 Failed to copy template file")?;
    }

    Ok(())
}
