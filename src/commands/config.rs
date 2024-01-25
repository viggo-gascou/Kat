use crate::{
    cli::{Config, ConfigCommands},
    utils::config::InternalConfig,
    App,
};

use std::{
    fs,
    path::{Path, PathBuf},
};

use color_eyre::{
    eyre::{Context, ContextCompat},
    Report,
};

use colored::Colorize;

pub async fn config(app: &App, args: &Config) -> Result<(), Report> {
    match &args.subcommand {
        ConfigCommands::Locate => {
            get_config_location(app).wrap_err("🙀 Failed to load config location")
        }
        ConfigCommands::Set(set_args) => {
            set_config_location(app, &set_args.path).wrap_err("🙀 Failed to set config location")
        }
    }
}

fn get_config_location(app: &App) -> Result<(), Report> {
    let internal_config = &app.config.internal_config;
    let config_location = &internal_config.config_location;

    let config_dir = PathBuf::from(config_location);
    let config_location = config_dir.join("config.toml");
    let kattisrc_location = config_dir.join("kattisrc");
    let templates_location = config_dir.join("templates");

    println!(
        "\nYour config directory is located at: {}",
        config_dir.display()
    );
    println!("Meaning that your config files are located at:\n");
    println!("\t- Config location: {}", config_location.display());
    println!("\t- Kattisrc location: {}", kattisrc_location.display());
    println!("\t- Templates location: {}", templates_location.display());

    Ok(())
}

fn set_config_location(app: &App, path: &Path) -> Result<(), Report> {
    let mut internal_config = InternalConfig::new()?;

    let config_dir = path;

    let old_config_dir = PathBuf::from(app.config.internal_config.config_location.clone());

    fs::create_dir_all(config_dir).wrap_err("🙀 Failed to create config directory")?;

    internal_config.set_location(
        path.to_str()
            .wrap_err("🙀 Failed to convert path to string!")?
            .to_string(),
    )?;

    println!(
        "😸 Successfully set the config directory from {} to {}",
        old_config_dir.display(),
        config_dir.display()
    );
    println!(
        "Remember to move your config files to the new location by e.g., running: {}",
        format!("mv {} {}", old_config_dir.display(), config_dir.display()).bold()
    );

    Ok(())
}
