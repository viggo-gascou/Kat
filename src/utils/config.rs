use color_eyre::{
    eyre,
    eyre::{Context, ContextCompat},
    Report,
};
use config::{File, FileFormat};
use dirs::data_dir;
use secrecy::Secret;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};
use toml::to_string_pretty;

#[derive(Debug, Deserialize, Serialize)]
pub struct InternalConfig {
    pub config_location: String,
}

impl InternalConfig {
    pub fn new() -> Result<Self, Report> {
        
        let config_location = InternalConfig::get_config_location()?;
        let internal_config = InternalConfig {
            config_location: config_location.clone(),
        };
        Ok(internal_config)
    }
    pub fn get_config_location() -> Result<String, Report> {
        let mut internal_config_path =
            data_dir().wrap_err("🙀 Failed to determine data directory")?;

        internal_config_path.push("kat");
        internal_config_path.push("internal_config.toml");

        if internal_config_path.exists() {
            let settings = config::Config::builder()
                .add_source(File::from(internal_config_path).format(FileFormat::Toml))
                .build()
                .wrap_err("🙀 Failed to build the internal config!")?;

            let internal_config: InternalConfig = settings.try_deserialize()?;
            Ok(internal_config.config_location)
        } else {
            // Get the default location for the config directory
            let home_dir = dirs::home_dir().wrap_err("🙀 Could not find home directory")?;
            let default_location = home_dir
                .join(".kat")
                .to_str()
                .wrap_err("🙀 Could not convert path to string")?
                .to_string();

            let mut internal_config = InternalConfig::new()?;
            internal_config.set_location(default_location)?;

            Ok(internal_config.config_location)
        }
    }

    pub fn set_location(&mut self, path: String) -> Result<String, Report> {
        let path = PathBuf::from(shellexpand::full(&path).unwrap().to_string());
        if !path.is_dir() {
            eyre::bail!("🙀 The provided path, {}, is not a directory", path.display());
        }

        self.config_location = path
            .to_str()
            .wrap_err("🙀 Could not convert path to string")?
            .to_string();

        let mut internal_config_path =
            data_dir().wrap_err("🙀 Failed to determine the location of the internal config")?;
        internal_config_path.push("kat");
        fs::create_dir_all(&internal_config_path)?;
        internal_config_path.push("internal_config.toml");

        let toml = to_string_pretty(self)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(internal_config_path)?;
        file.write_all(toml.as_bytes())?;

        Ok(self.config_location.clone())
    }
}

#[derive(Debug, Deserialize)]
pub struct KattisrcUser {
    pub username: String,
    pub token: Secret<String>,
}

#[derive(Debug, Deserialize)]
pub struct KattisrcKattis {
    pub hostname: String,
    pub loginurl: String,
    pub submissionurl: String,
    pub submissionsurl: String,
}

#[derive(Debug, Deserialize)]
pub struct Kattisrc {
    pub user: KattisrcUser,
    pub kattis: KattisrcKattis,
}

impl Kattisrc {
    pub fn new() -> Result<Self, Report> {
        let internal_config = InternalConfig::new()?;
        let config_location = internal_config.config_location;
        let config_file = PathBuf::from(config_location).join("kattisrc");
        let config_file_str = config_file
            .to_str()
            .wrap_err("🙀 Could not convert path to string")?
            .to_string();

        if !config_file.exists() {
            eyre::bail!("🙀 No kattisrc file found in the config directory at {config_file_str}!
                        Please download one and place it in the correct directory!");
        }

        let config = config::Config::builder()
            .add_source(
                File::from(config_file)
                    .format(FileFormat::Ini)
                    .required(true),
            )
            .build()
            .wrap_err("🙀 Failed to build config, make sure kattisrc is in the correct format! See the README.md for more information")?;

        let kattisrc: Kattisrc = config
            .try_deserialize()
            .wrap_err_with(|| format!("🙀 Failed to parse kattisrc at {config_file_str}!"))?;
        Ok(kattisrc)
    }
}

#[derive(Debug, Deserialize)]
struct Language {
    compile_command: Option<String>,
    execute_command: String,
    extensions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Languages(HashMap<String, Language>);

#[derive(Debug, Deserialize)]
struct Default {
    language: String,
}

#[derive(Debug, Deserialize)]
pub struct KatConfig {
    default: Default,
    languages: Languages,
}

impl KatConfig {
    pub fn new() -> Result<Self, Report> {
        let internal_config = InternalConfig::new()?;
        let config_location = internal_config.config_location;
        let config_file = PathBuf::from(config_location).join("config.toml");
        let config_file_str = config_file
            .to_str()
            .wrap_err("🙀 Could not convert path to string")?
            .to_string();

        if !config_file.exists() {
            eyre::bail!("🙀 No config.toml file found in the config directory at {config_file_str}! Please run kat config init to initialise a sample config!");
        }

        let config = config::Config::builder()
            .add_source(File::from(config_file)
            .format(FileFormat::Toml)
            .required(true))
            .build()
            .wrap_err("🙀 Failed to build config, make sure config.toml is in the correct format! See the README.md for more information")?;

        let kat_config: KatConfig = config
            .try_deserialize()
            .wrap_err_with(|| format!("🙀 Failed to parse config.toml at {config_file_str}!"))?;
        Ok(kat_config)
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub kattisrc: Kattisrc,
    pub kat_config: KatConfig,
    pub internal_config: InternalConfig,
}

impl Config {
    pub fn load() -> Result<Self, Report> {
        let internal_config = InternalConfig::new()?;
        let kattisrc = Kattisrc::new()?;
        let kat_config = KatConfig::new()?;
        Ok(Config {
            kattisrc,
            kat_config,
            internal_config,
        })
    }
}
