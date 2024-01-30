use crate::{
    cli::Init,
    utils::{config::InternalConfig, HttpClient},
};

use std::{
    fs,
    path::{Path, PathBuf},
};

use color_eyre::{
    eyre::{self, Context},
    owo_colors::OwoColorize,
    Report,
};

use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect, Select};

#[derive(serde::Deserialize, Debug, Clone)]
struct GitHubFile {
    name: String,
    download_url: String,
}

pub async fn init(args: &Init) -> Result<(), Report> {
    // initialise the config directory
    let internal_config = InternalConfig::new()?;
    let config_location = internal_config.config_location;
    let config_dir = PathBuf::from(&config_location);
    fs::create_dir_all(&config_dir).wrap_err("🙀 Failed to create config directory")?;

    // Define the path to the config file
    let config_file_path = config_dir.join("config.toml");

    // Check if the config file exists or if the user has specified the --yes flag download anyways
    if !config_file_path.exists() || args.yes {
        download_sample_files(&config_dir, args).await?;
    } else {
        println!(
            "{}",
            format!(
                "👀 Looks like the config files already exists at {}!",
                config_file_path.display()
            )
            .bright_yellow()
        );
        let overwrite = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to overwrite them? (Careful this will overwrite any existing config files!)")
            .interact()
            .wrap_err("🙀 Failed to get user input")?;

        if overwrite {
            download_sample_files(&config_dir, args).await?;
        } else {
            println!(
                "{}",
                "👍 Ok, not initialising any config files!".bright_green()
            );
            return Ok(());
        }
    }

    println!("{}",
        format!("👍 Successfully initialised the config files at {}, make sure to download your kattisrc file and put it in the same directory!",
        config_dir.display()).underline().bright_green()
    );

    Ok(())
}

async fn download_sample_files(config_dir: &Path, args: &Init) -> Result<(), Report> {
    let templates_url = "https://api.github.com/repos/viggo-gascou/kat-rs/contents/templates";

    let http_client = HttpClient::new().unwrap();

    let api_response = http_client.client.get(templates_url).send().await?;

    if api_response.status() != 200 {
        eyre::bail!("🙀 Failed to get contents of templates folder");
    }

    let files: Vec<GitHubFile> = api_response
        .json()
        .await
        .wrap_err("🙀 Failed to parse contents of templates folder")?;

    let files_to_download = if let Some(choice) = &args.choice {
        match choice.as_str() {
            "all" => {
                // If "all" is specified, download all files
                files.clone()
            }
            "config" => {
                // If "config" is specified, download only the sample config file
                files
                    .iter()
                    .filter(|file| file.name == "config.toml")
                    .cloned()
                    .collect::<Vec<GitHubFile>>()
            }
            _ => {
                // Invalid option, return an error
                eyre::bail!("Invalid option specified for files to download");
            }
        }
    } else {
        let download_option = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What files do you want to download?")
            .items(&["All", "Just the sample config.toml", "Choose", "Cancel"])
            .default(0)
            .interact()
            .unwrap();

        match download_option {
            0 => {
                // If "All" is selected, download all files
                files.clone()
            }
            1 => {
                // If "Just the sample config.toml" is selected, download only the sample config file
                files
                    .iter()
                    .filter(|file| file.name == "config.toml")
                    .cloned()
                    .collect::<Vec<GitHubFile>>()
            }
            2 => {
                // If "Choose" is selected, let the user choose specific files
                let file_names: Vec<&str> = files.iter().map(|file| file.name.as_str()).collect();
                let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select the template files you want to download (use space to select, enter to finish)")
                    .items(&file_names[..])
                    .interact()
                    .unwrap();
                if selections.is_empty() {
                    eyre::bail!("No files selected - if this was a mistake - please run `kat init` again and and use the space bar to select items, then press enter to finish.");
                }
                selections
                    .iter()
                    .map(|&i| files[i].clone())
                    .collect::<Vec<GitHubFile>>()
            }
            3 => {
                // If "Cancel" is selected, exit the program
                println!(
                    "{}",
                    "👍 Ok, cancelling the config initialisation of kat!"
                        .bold()
                        .bright_yellow()
                );
                return Ok(());
            }
            _ => eyre::bail!("Invalid option selected"),
        }
    };

    println!(
        "{}",
        "📥 Fetching the specified sample config file(s) ..."
            .bold()
            .bright_blue()
    );
    for file in files_to_download {
        let file_path = if file.name.contains("template") {
            fs::create_dir_all(config_dir.join("templates"))
                .wrap_err("🙀 Failed to create templates directory")?;
            config_dir.join("templates").join(&file.name)
        } else {
            config_dir.join(&file.name)
        };

        let response = reqwest::get(&file.download_url)
            .await
            .wrap_err("🙀 Failed to download sample config file")?;

        let content = response.text().await.wrap_err(format!(
            "🙀 Failed to read file contents of file {}",
            &file.name
        ))?;

        // Save the sample config to the config file
        fs::write(file_path, content).wrap_err("🙀 Failed to write sample config file")?;
    }

    Ok(())
}
