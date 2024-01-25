use crate::{
    cli::Watch,
    commands::test::test_problem,
    utils::{find_problem_dir, find_test_files, get_problem_file},
    App,
};

use std::path::{Path, PathBuf};

use color_eyre::{
    self,
    eyre::{Context, ContextCompat},
    Report,
};

use notify::{event::ModifyKind, EventKind, RecommendedWatcher, Watcher};

use colored::Colorize;

pub async fn watch(app: &App, args: &Watch) -> Result<(), Report> {
    let (problem_path, problem_id) = find_problem_dir(app, &args.path)?;
    let (problem_file, problem_file_path, language) =
        get_problem_file(app, &args.file, &args.language, &problem_path, &problem_id)?;
    let tests = find_test_files(app, &args.test_cases, &problem_path)?;

    println!(
        "{}",
        format!(
            "👀 Watching the file {} for changes to test the problem {} ...\n",
            &problem_file, problem_id
        )
        .bold()
        .bright_blue()
    );

    watch_problem(
        app,
        &problem_id,
        &problem_path,
        &problem_file_path,
        tests,
        &language,
    )
    .await?;

    Ok(())
}

async fn watch_problem(
    app: &App,
    problem_id: &str,
    problem_path: &Path,
    problem_file_path: &Path,
    tests: Vec<(PathBuf, PathBuf)>,
    language: &str,
) -> Result<(), Report> {
    let problem_file = problem_file_path
        .file_name()
        .expect("🙀 Failed to get file name from path")
        .to_str()
        .wrap_err("🙀 Failed to convert file name to string")?;

    // Test the problem once before starting to watch
    if test_problem(
        app,
        problem_id,
        problem_path,
        problem_file_path,
        tests.clone(),
        language,
    )? {
        print_pass_message(problem_id, problem_file);
    }

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;

    watcher.watch(problem_file_path, notify::RecursiveMode::NonRecursive)?;

    loop {
        let event = rx.recv().wrap_err(format!(
            "🙀 Something went wrong with watching the file at: {}",
            &problem_file_path.display()
        ))?;
        match event {
            Ok(event) => {
                // Only run tests if the data/content of the file changed - not if if saved again without changes
                if let EventKind::Modify(ModifyKind::Data(_)) = event.kind {
                    println!(
                        "{}",
                        "👀 File changed, testing again ...".bold().bright_blue()
                    );
                    if test_problem(
                        app,
                        problem_id,
                        problem_path,
                        problem_file_path,
                        tests.clone(),
                        language,
                    )? {
                        print_pass_message(problem_id, problem_file);
                    }
                    println!("{}", "=".repeat(25).bright_purple()); // Separator line
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn print_pass_message(problem_id: &str, problem_file: &str) {
    println!("If you want you can try to submit your solution using {}
Or you can keep editing the file {problem_file} and the tests will be run again automatically when you save it.
Press Ctrl+C to stop watching the file and exit 😸",format!("kat submit {problem_id}").bold());
}
