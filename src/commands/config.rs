use std::{fs, path::{PathBuf, Path}};

use crate::{
    cli::{Config, ConfigCommands},
    utils::config::InternalConfig,
    App,
};

use color_eyre::{eyre::{Context, ContextCompat}, Report};

pub async fn config(app: &App, args: &Config) -> Result<(), Report> {
    match &args.subcommand {
        ConfigCommands::Init => init_config(app)
            .await
            .wrap_err("🙀 Failed to initialise config"),
        ConfigCommands::Locate => {
            get_config_location(app).wrap_err("🙀 Failed to load config location")
        }
        ConfigCommands::Set(set_args) => set_config_location(app, &set_args.path)
            .wrap_err("🙀 Failed to set config location"),
    }
}

async fn init_config(_app: &App) -> Result<(), Report> {
    // initialise the config directory
    let internal_config = InternalConfig::new()?;
    let config_location = internal_config.config_location;
    let config_dir = PathBuf::from(&config_location);
    fs::create_dir_all(&config_dir).wrap_err("🙀 Failed to create config directory")?;

    // Define the path to the config file
    let config_file_path = config_dir.join("config.toml");

    // Check if the config file exists
    if !config_file_path.exists() {
        // If the config file doesn't exist, download the sample config
        let sample_config_url =
            "https://github.com/viggo-gascou/kat/raw/main/templates/config.toml";
        // blocking request
        let sample_config = reqwest::blocking::get(sample_config_url)
            .wrap_err("🙀 Failed to download sample config file")?
            .text()
            .wrap_err("🙀 Failed to read sample config file")?;

        // Save the sample config to the config file
        fs::write(config_file_path, sample_config)
            .wrap_err("🙀 Failed to write sample config file")?;
    }

    Ok(())
}

fn get_config_location(_app: &App) -> Result<(), Report> {
    let internal_config = InternalConfig::new()?;
    let config_location = internal_config.config_location;

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

    internal_config.set_location(path.to_str().wrap_err("🙀 Failed to convert path to string!")?.to_string())?;

    println!(
        "😸 Successfully set the config directory from {} to {}",
        old_config_dir.display(),
        config_dir.display()
    );
    println!("Remember to move your config files to the new location!");

    Ok(())
}
