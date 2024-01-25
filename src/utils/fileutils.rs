use crate::App;

use std::{
    cmp::Ordering,
    collections::HashMap,
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::{
    eyre,
    eyre::{Context, ContextCompat},
    Report,
};

use colored::Colorize;
use glob::glob;
use regex::Regex;

pub fn get_problem_dir(_app: &App, problem: &str) -> Result<PathBuf, Report> {
    let current_dir = std::env::current_dir().wrap_err("🙀 Failed to get current directory")?;
    let problem_dir = current_dir.join(problem);

    Ok(problem_dir)
}

pub fn get_test_dir(app: &App, problem: &str) -> Result<PathBuf, Report> {
    let problem_dir =
        get_problem_dir(app, problem).wrap_err("🙀 Failed to get problem directory")?;
    let test_dir = problem_dir.join("tests");
    Ok(test_dir)
}

pub fn copy_template(
    app: &App,
    language: &Option<String>,
    mut problem: &str,
) -> Result<(), Report> {
    let problem_dir = get_problem_dir(app, problem)?;
    let config_dir = PathBuf::from(&app.config.internal_config.config_location);
    let template_dir = config_dir.join("templates");
    let config = &app.config.kat_config;

    let language = match &language {
        Some(lang) => {
            if config.languages.contains_key(lang) {
                lang
            } else {
                eyre::bail!("🙀 Invalid language: {}", lang);
            }
        }
        None => &config.default.language,
    };

    // safe since language gets set to a value no matter what
    let template_path = match &config.languages[language].template {
        Some(template_file) => {
            // join the template file with the template directory to get the full path to the template file
            Some(template_dir.join(template_file))
        }
        None => {
            log::warn!(
                "{}",
                format!(
                    "🙀 Warning: No template file found for language: {}",
                    language
                )
                .bold()
                .bright_yellow()
            );
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
            .wrap_err("🙀 Failed to get file name from path")?
            .to_str()
            .wrap_err("🙀 Failed to convert file name to string")?
            .to_string();
        let template_file_no_ext = template_path
            .file_stem()
            .wrap_err("🙀 Failed to get file name from path")?
            .to_str()
            .wrap_err("🙀 Failed to convert file name to string")?
            .to_string();            

        // strip the subdomain from the problem id
        if problem.contains('.') {
            problem = problem.split('.').nth(1).unwrap();
        }
        let problem_file_name = template_file_name.replace(&template_file_no_ext, problem);
        let problem_file_path = problem_dir.join(&problem_file_name);

        let template_file = fs::read_to_string(&template_path)
            .wrap_err("🙀 Failed to open template file for reading")?
            .replace("{source_file_no_ext}", problem)
            .replace("{source_file}", &problem_file_name.clone());

        fs::write(problem_file_path, template_file)
            .wrap_err("🙀 Failed to create template file in problem directory")?;
    }

    Ok(())
}

fn parse_filter(filter: &str) -> Vec<u32> {
    let mut test_numbers = Vec::new();

    for part in filter.split(',') {
        if part.contains('-') {
            let range_parts: Vec<&str> = part.split('-').collect();
            let start: u32 = range_parts[0]
                .trim()
                .parse()
                .expect("Invalid number in filter");
            let end: u32 = range_parts[1]
                .trim()
                .parse()
                .expect("Invalid number in filter");
            test_numbers.extend(start..=end);
        } else {
            let number: u32 = part.trim().parse().expect("Invalid number in filter");
            test_numbers.push(number);
        }
    }
    test_numbers
}

pub fn find_test_files(
    _app: &App,
    test_cases: &Option<String>,
    problem_path: &Path,
) -> Result<Vec<(PathBuf, PathBuf)>, Report> {
    let extensions = ["in", "ans"];
    let test_path = problem_path.join("tests");
    if !test_path.exists() {
        eyre::bail!("🙀 This problem does not have any tests - not testing!")
    }
    let filter = match &test_cases {
        Some(filter) => filter,
        None => "all",
    };
    let test_path = test_path
        .to_str()
        .expect("🙀 Failed to convert problem path to string")
        .to_string();

    let mut tests_by_ext = HashMap::new();

    for extension in extensions {
        let pattern = format!("{test_path}/*.{}", &extension);
        let files = glob(&pattern)
            .expect("🙀 Failed to read glob pattern")
            .filter_map(Result::ok)
            .collect::<Vec<PathBuf>>();

        // glob seems to return files in alphabetical order -> rust-lang/glob/issues/98
        // so no need for sorting

        let matching_files = if filter == "all" {
            files
        } else {
            let test_filter = parse_filter(filter);
            files
                .into_iter()
                .filter(|file| {
                    let file_name = file
                        .file_stem()
                        .expect("🙀 Failed to get file name from path")
                        .to_str()
                        .expect("🙀 Failed to convert file name to string");

                    let re = Regex::new(r"\d+").unwrap();
                    let number_part = re
                        .find(file_name)
                        .expect("🙀 No number found in file name")
                        .as_str();
                    let file_number: u32 = number_part
                        .parse()
                        .expect("🙀 File name does not contain a valid number");

                    test_filter.contains(&file_number)
                })
                .collect::<Vec<PathBuf>>()
        };

        tests_by_ext.insert(extension, matching_files);
    }

    if let Some(in_files) = tests_by_ext.get("in") {
        if let Some(ans_files) = tests_by_ext.get("ans") {
            match in_files.len().cmp(&ans_files.len()) {
                Ordering::Greater => {
                    eyre::bail!("🙀 There are more input test files than answer files. Are they all in the {test_path} directory?");
                }
                Ordering::Less => {
                    eyre::bail!("🙀 There are more answer files than input test files. Are they all in the {test_path} directory?");
                }
                Ordering::Equal => {
                    if in_files.is_empty() || ans_files.is_empty() {
                        let file_type = if in_files.is_empty() {
                            "input"
                        } else {
                            "answer"
                        };
                        eyre::bail!("🙀 No {file_type} files found in directory: {test_path}");
                    }
                    let mut test_files = Vec::new();
                    for (in_file, ans_file) in in_files.iter().zip(ans_files.iter()) {
                        test_files.push((in_file.clone(), ans_file.clone()));
                    }
                    Ok(test_files)
                }
            }
        } else {
            eyre::bail!("🙀 No answer files found in directory: {}", test_path);
        }
    } else {
        eyre::bail!("🙀 No input files found in directory: {}", test_path);
    }
}

pub fn find_problem_files(
    problem_path: &Path,
    problem_id: &String,
    extensions: &[String],
) -> Vec<PathBuf> {
    let problem_path = problem_path
        .to_str()
        .expect("🙀 Failed to convert problem path to string")
        .to_string();
    let mut matching_files = Vec::new();
    for extension in extensions {
        let pattern = format!("{problem_path}/*{problem_id}*.{extension}");
        let files = glob(&pattern)
            .expect("Failed to read glob pattern")
            .filter_map(Result::ok);
        matching_files.extend(files);
    }
    matching_files
}

pub fn get_problem_file(
    app: &App,
    file: &Option<PathBuf>,
    language: &Option<String>,
    problem_path: &Path,
    problem_id: &String,
) -> Result<(String, PathBuf, String), Report> {
    // if args.file is not set, try to find a file with the same name as the problem id
    // that has a matching extension from default language set in the config
    // if multiple files are found, prompt the user to choose which one to use
    log::info!(
        "{}",
        format!(
            "🔍 Looking for problem file for the problem {}...\n",
            problem_id
        )
        .bold()
        .bright_blue()
    );
    let config = &app.config.kat_config;
    let language = match &language {
        Some(lang) => {
            if config.languages.contains_key(lang) {
                lang
            } else {
                eyre::bail!("🙀 Invalid language: {}", lang);
            }
        }
        None => &config.default.language,
    };
    let (problem_file, problem_file_path) = match &file {
        Some(problem_file_path) => (
            problem_file_path
                .file_name()
                .expect("🙀 Failed to get file name from path")
                .to_str()
                .wrap_err("🙀 Failed to convert file name to string")?
                .to_string(),
            problem_file_path.clone(),
        ),
        None => {
            let extensions = config
                .languages
                .get(language)
                .wrap_err(format!(
                    "🙀 Could not find any language with the name: {}",
                    language
                ))?
                .extensions
                .clone();
            let matching_files: Vec<PathBuf> =
                find_problem_files(problem_path, problem_id, &extensions);

            if matching_files.is_empty() {
                eyre::bail!(
                    "🙀 No matching problem files found in directory: {}",
                    problem_path.display()
                );
            } else if matching_files.len() == 1 {
                let problem_file = matching_files[0]
                    .file_name()
                    .expect("🙀 Failed to get file name from path")
                    .to_str()
                    .wrap_err("🙀 Failed to convert file name to string")?
                    .to_string();

                (problem_file, matching_files[0].clone())
            } else {
                let mut file_choices: Vec<String> = Vec::new();
                for file in &matching_files {
                    let file_name = file
                        .file_name()
                        .expect("🙀 Failed to get file name from path")
                        .to_str()
                        .wrap_err("🙀 Failed to convert file name to string")?
                        .to_string();
                    file_choices.push(file_name);
                }
                let file_choice = dialoguer::Select::new()
                    .with_prompt(
                        "👉 Multiple matching files found, please choose which one to use:",
                    )
                    .items(&file_choices)
                    .default(0)
                    .interact()
                    .wrap_err("🙀 Failed to get user input")?;
                let problem_file = matching_files[file_choice]
                    .file_name()
                    .expect("🙀 Failed to get file name from path")
                    .to_str()
                    .wrap_err("🙀 Failed to convert file name to string")?
                    .to_string();
                (problem_file, matching_files[file_choice].clone())
            }
        }
    };
    Ok((problem_file, problem_file_path, language.to_string()))
}

pub fn find_problem_dir(_app: &App, path: &Path) -> Result<(PathBuf, String), Report> {
    let current_dir = current_dir().wrap_err("🙀 Failed to get current directory")?;
    if path == PathBuf::from(".") {
        let problem_id = current_dir
            .file_name()
            .expect("🙀 Failed to get file name from path")
            .to_str()
            .wrap_err("🙀 Failed to convert file name to string")?
            .to_string();

        log::info!(
            "{}",
            format!(
                "📂 Using current directory as problem path for the problem {}\n",
                problem_id
            )
            .bold()
            .bright_blue()
        );
        Ok((current_dir, problem_id))
    } else {
        let problem_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            current_dir.join(path)
        };
        let problem_id = problem_path
            .file_name()
            .expect("🙀 Failed to get file name from path")
            .to_str()
            .wrap_err("🙀 Failed to convert file name to string")?
            .to_string();

        if problem_path.exists() {
            log::info!(
                "📂 Using {} as the problem path, for the problem {}\n",
                problem_path.display(),
                problem_id
            );

            Ok((problem_path, problem_id))
        } else {
            eyre::bail!(
                "🙀 Problem path {} does not exist, try fetching it with: kat get {}",
                problem_path.display(),
                problem_id
            );
        }
    }
}
