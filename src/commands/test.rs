use crate::{
    cli::Test,
    commands::submit::{send_submission, Submission},
    utils::{find_problem_dir, find_test_files, get_problem_file},
    App,
};

use std::{
    collections::HashMap,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::{
    eyre::{self, Context, ContextCompat},
    Report,
};

pub async fn test(app: &App, args: &Test) -> Result<(), Report> {
    let (problem_path, problem_id) = find_problem_dir(app, &args.path)?;
    let (problem_file, problem_file_path, _) = get_problem_file(
        app,
        &args.problem,
        &args.language,
        &problem_path,
        &problem_id,
    )?;
    let tests = find_test_files(app, args, &problem_path, "in")?;

    println!(
        "🧪 Testing problem: {} with the file {} ...\n",
        problem_id, &problem_file
    );

    test_problem(
        app,
        args,
        &problem_id,
        &problem_path,
        &problem_file_path,
        tests,
    )
    .await?;

    Ok(())
}

async fn test_problem(
    app: &App,
    args: &Test,
    problem_id: &str,
    problem_path: &Path,
    problem_file_path: &Path,
    tests: HashMap<PathBuf, PathBuf>,
) -> Result<(), Report> {
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

    let compile_command = match &config.languages.get(language).unwrap().compile_command {
        Some(compile_command) => compile_command,
        None => "",
    };

    let execute_command = &config.languages.get(language).unwrap().execute_command;

    if !compile_command.is_empty() {
        println!("🔨 Compiling problem: {} ...", problem_id);
        compile_problem(app, compile_command, problem_path, problem_file_path)?;
    }

    let mut all_tests_passed = true;
    for single_test in tests {
        if let Err(e) = execute_problem(
            app,
            execute_command,
            problem_path,
            problem_file_path,
            single_test,
        ) {
            all_tests_passed = false;
            eprintln!("Error: {}", e);
        }
    }

    if all_tests_passed {
        println!("🏁 All tests passed!");
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
                    language,
                    problem_file: problem_file_path
                        .file_name()
                        .expect("🙀 Failed to get file name from input file")
                        .to_str()
                        .wrap_err("🙀 Failed to convert file name to string")?
                        .to_string(),
                    problem_file_path: problem_file_path.to_path_buf(),
                };
                send_submission(app, submission).await?;
            } else {
                println!("🙀 Submission aborted");
            }
            return Ok(());
        } else {
            return Ok(());
        }
    } else if app.args.verbose {
        println!("❌ Some tests seem to have failed!");
    } else {
        println!("❌ Some tests seem to have failed, try re-running the tests, with --verbose!");
    }

    Ok(())
}

fn compile_problem(
    _app: &App,
    compile_command: &str,
    problem_path: &Path,
    problem_file_path: &Path,
) -> Result<(), Report> {
    let (compile_cmd, compile_args) = compile_command
        .split_once(' ')
        .ok_or_else(|| eyre::eyre!("🙀 Could not find arguments for compile command"))?;

    let compile_args = prepare_arguments(compile_args, problem_file_path, problem_path)?;

    let output = Command::new(compile_cmd)
        .args(compile_args)
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
    let (execute_cmd, execute_args) = execute_command
        .split_once(' ')
        .ok_or_else(|| eyre::eyre!("🙀 Could not find arguments for compile command"))?;

    let execute_args = prepare_arguments(execute_args, problem_file_path, problem_path)?;

    let (input_file_path, expected_output_file_path) = test;
    let input_file_name = input_file_path
        .file_name()
        .expect("🙀 Failed to get file name from input file")
        .to_str()
        .wrap_err("🙀 Failed to convert file name to string")?
        .to_string();
    let input_file = File::open(input_file_path.clone())?;

    let output = Command::new(execute_cmd)
        .args(&execute_args)
        .stdin(input_file)
        .output()
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                eyre::eyre!("🙀 Could not find execute command: {}", execute_cmd)
            }
            _ => eyre::eyre!("🙀 Failed to execute command with error: {}", e),
        })?;

    if !output.status.success() {
        // print all output
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
        eyre::bail!(
            "🙀 Failed to execute problem: {}",
            problem_file_path.display()
        );
    }

    // Compare the output of the program to the expected output
    let expected_output = fs::read_to_string(expected_output_file_path)?;
    let actual_output = String::from_utf8_lossy(&output.stdout).into_owned();
    if actual_output != expected_output {
        if app.args.verbose {
            Err(eyre::eyre!(
                "❌ Test {input_file_name} failed!\nExpected output: {}\nActual output: {}",
                expected_output,
                actual_output
            ))
        } else {
            Err(eyre::eyre!("❌ Test {input_file_name} failed!"))
        }
    } else {
        println!("✅ Test {} passed!", input_file_name);
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
