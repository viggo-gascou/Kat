use crate::{
    cli::Test,
    commands::submit::{send_submission, Submission},
    utils::{find_problem_dir, find_test_files, get_problem_file, HttpClient},
    App,
};

use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use color_eyre::{
    eyre::{self, Context, ContextCompat},
    Report,
};

use colored::Colorize;

pub async fn test(app: &App, args: &Test) -> Result<(), Report> {
    let (problem_path, problem_id) = find_problem_dir(app, &args.path)?;
    let (problem_file, problem_file_path, language) =
        get_problem_file(app, &args.file, &args.language, &problem_path, &problem_id)?;
    let tests = find_test_files(app, &args.test_cases, &problem_path)?;

    println!(
        "{}",
        format!(
            "🧪 Testing problem: {} with the file {} ...\n",
            problem_id, &problem_file
        )
        .bold()
        .bright_blue()
    );

    let passed = test_problem(
        app,
        &problem_id,
        &problem_path,
        &problem_file_path,
        tests,
        &language,
    )?;

    if passed {
        if args.submit {
            let submit = dialoguer::Select::new()
                .with_prompt("Do you want to submit this file?")
                .default(0)
                .items(&["Yes", "No"])
                .interact()
                .wrap_err("🙀 Failed to get user input")?;
            if submit == 0 {
                let submission = Submission {
                    problem_id: problem_id.to_string(),
                    language: &language,
                    problem_file: problem_file_path
                        .file_name()
                        .expect("🙀 Failed to get file name from input file")
                        .to_str()
                        .wrap_err("🙀 Failed to convert file name to string")?
                        .to_string(),
                    problem_file_path: problem_file_path.to_path_buf(),
                };
                let http_client = HttpClient::new().unwrap();
                send_submission(app, submission, &http_client).await?;
            } else {
                println!("🙀 Submission aborted");
            }
        }
    } else if app.args.verbose.log_level().is_some() {
        println!("{}", "❌ Some tests seem to have failed!".bright_red());
    } else if app.args.verbose.is_silent() {
        // print nothing as user has specified --quiet
    } else {
        println!(
            "{}",
            "❌ Some tests seem to have failed, try re-running the tests, with the verbose flag!"
                .bright_red()
        );
    }

    Ok(())
}

pub fn test_problem(
    app: &App,
    problem_id: &str,
    problem_path: &Path,
    problem_file_path: &Path,
    tests: Vec<(PathBuf, PathBuf)>,
    language: &str,
) -> Result<bool, Report> {
    let config = &app.config.kat_config;

    let compile_command = match &config.languages.get(language).unwrap().compile_command {
        Some(compile_command) => compile_command,
        None => "",
    };

    let execute_command = &config.languages.get(language).unwrap().execute_command;

    if !compile_command.is_empty() {
        println!("🔨 Compiling problem: {} ...", problem_id);
        compile_problem(compile_command, problem_path, problem_file_path)?;
    }

    let mut all_tests_passed = true;
    let start_time = std::time::Instant::now();
    for single_test in tests {
        if let Err(e) = execute_problem(
            app,
            execute_command,
            problem_path,
            problem_file_path,
            single_test,
        ) {
            all_tests_passed = false;
            println!("{e}");
        }
    }
    let elapsed_time = format!("{:.2}", start_time.elapsed().as_secs_f64());
    if all_tests_passed {
        println!(
            "{}",
            format!(
                "🏁 All tests for {} passed in {}s!",
                problem_id.underline(),
                elapsed_time
            )
            .bright_green()
        );
    }

    Ok(all_tests_passed)
}

fn compile_problem(
    compile_command: &str,
    problem_path: &Path,
    problem_file_path: &Path,
) -> Result<(), Report> {
    let executable_path = problem_file_path.with_extension("");
    let compile_command =
        &compile_command.replace("{executable_path}", executable_path.to_str().unwrap());

    let (compile_cmd, compile_args) = compile_command
        .split_once(' ')
        .ok_or_else(|| eyre::eyre!("🙀 Could not find arguments for compile command"))?;

    let compile_args = prepare_arguments(compile_args, problem_file_path, problem_path)?;

    let output = Command::new(compile_cmd)
        .args(compile_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                eyre::eyre!("🙀 Could not find compile command: {}", compile_cmd)
            }
            _ => eyre::eyre!("🙀 Failed to execute compile command with error: {}", e),
        })?;

    if !output.status.success() {
        // print all output
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));
        eyre::bail!(
            "🙀 Failed to compile problem: {}",
            problem_file_path.display()
        );
    }

    Ok(())
}

fn execute_problem(
    app: &App,
    execute_command: &str,
    problem_path: &Path,
    problem_file_path: &Path,
    test: (PathBuf, PathBuf),
) -> Result<(), Report> {
    let executable_path = problem_file_path.with_extension("");
    let execute_command =
        &execute_command.replace("{executable_path}", executable_path.to_str().unwrap());

    let (execute_cmd, execute_args) = match execute_command.split_once(' ') {
        Some((cmd, args)) => (cmd, Some(args)),
        None => (execute_command.as_str(), None),
    };

    let execute_args = match execute_args {
        Some(args) => prepare_arguments(args, problem_file_path, problem_path)?,
        None => Vec::new(),
    };

    let (input_file_path, expected_output_file_path) = test;
    let input_file_name = input_file_path
        .file_name()
        .expect("🙀 Failed to get file name from input file")
        .to_str()
        .wrap_err("🙀 Failed to convert file name to string")?
        .to_string();
    let input_file = File::open(input_file_path.clone())?;

    let start_time = std::time::Instant::now();
    let output = Command::new(execute_cmd)
        .args(&execute_args)
        .stdin(Stdio::from(input_file))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                eyre::eyre!("🙀 Could not find execute command: {}", execute_cmd)
            }
            _ => eyre::eyre!("🙀 Failed to execute command with error: {}", e),
        })?;
    let elapsed_time = format!("{:.2}", start_time.elapsed().as_secs_f64());

    if !output.status.success() {
        // print all output
        println!(
            "{}",
            format!("Output: {}", String::from_utf8_lossy(&output.stdout)).bold()
        );
        println!(
            "{}",
            format!("Error: {}", String::from_utf8_lossy(&output.stderr)).bold()
        );
        eyre::bail!(format!(
            "🙀 Failed to execute problem: {}",
            problem_file_path.display()
        ));
    }

    // Compare the output of the program to the expected output
    let expected_output = fs::read_to_string(expected_output_file_path)?;
    let actual_output = String::from_utf8_lossy(&output.stdout).into_owned();
    if actual_output != expected_output {
        match app.args.verbose.log_level() {
            Some(log::Level::Error) | None => {
                eyre::bail!(
                    "{}{}",
                    format!("❌ Test {input_file_name} failed!\n").bright_red(),
                    format!("Output:\n{}", actual_output)
                )
            }
            Some(_) => {
                let stderr_output = String::from_utf8_lossy(&output.stderr);
                let error_output = if !String::from_utf8_lossy(&output.stderr).is_empty() {
                    format!("\nError output:\n{}\n", stderr_output)
                } else {
                    String::new()
                };
                eyre::bail!(
                    "{}{}",
                    format!("❌ Test {input_file_name} failed!\n").bright_red(),
                    format!(
                        "Expected output:\n{}\nActual output:\n{}{}",
                        expected_output, actual_output, error_output,
                    )
                    .bold()
                )
            }
        }
    } else {
        println!(
            "{}",
            format!("✅ Test {} passed in {}s!", input_file_name, elapsed_time).bright_green()
        );
        Ok(())
    }
}

fn prepare_arguments(
    compile_args: &str,
    problem_file_path: &Path,
    problem_path: &Path,
) -> Result<Vec<String>, Report> {
    let mut compile_args =
        shlex::split(compile_args).ok_or_else(|| eyre::eyre!("🙀 Failed to split arguments"))?;

    let executable = problem_file_path
        .file_stem()
        .ok_or_else(|| eyre::eyre!("🙀 Failed to get file stem"))?;

    for arg in compile_args.iter_mut() {
        if arg.contains("{output_directory}") {
            *arg = arg.replace("{output_directory}", problem_path.to_str().unwrap());
        }
        if arg.contains("{executable_path}") {
            *arg = arg.replace("{executable_path}", executable.to_str().unwrap());
        }
        if arg.contains("{source_file}") {
            *arg = arg.replace("{source_file}", problem_file_path.to_str().unwrap());
        }
        if arg.contains("{source_file_no_ext}") {
            *arg = arg.replace("{source_file_no_ext}", executable.to_str().unwrap());
        }
    }

    Ok(compile_args)
}
