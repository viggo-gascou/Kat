use color_eyre::{eyre, eyre::{Context, ContextCompat}, Report};
use config::{File, FileFormat};
use dirs::home_dir;
use secrecy::Secret;
use serde::Deserialize;

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
        let config_file = home_dir()
            .wrap_err("🙀 Failed to determine home directory")?
            .join(".kattisrc");
        if !config_file.exists() {
            eyre::bail!("🙀 No .kattisrc file found in home directory! Please download it from kattis and place it in your home directory - see README.md for more information.");
        }

        let config = config::Config::builder()
            .add_source(
                File::from(config_file)
                    .format(FileFormat::Ini)
                    .required(true),
            )
            .build()
            .wrap_err("🙀 Failed to build config, make sure .kattisrc is in the correct format! See the README.md for more information")?;

        let kattisrc: Kattisrc = config.try_deserialize().wrap_err("🙀 Failed to parse .kattisrc!")?;
        Ok(kattisrc)
    }
}

use std::collections::HashMap;

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
        let config_file = home_dir()
            .wrap_err("🙀 Failed to determine home directory")?
            .join("kat.toml");
        if !config_file.exists() {
            eyre::bail!("🙀 No .kat.toml file found in home directory! Please run kat config init to initialise a sample config!");
        }

        let config = config::Config::builder()
            .add_source(File::from(config_file)
            .format(FileFormat::Toml)
            .required(true))
            .build()
            .wrap_err("🙀 Failed to build config, make sure .kat.toml is in the correct format! See the README.md for more information")?;


        let kat_config: KatConfig = config.try_deserialize().wrap_err("🙀 Failed to parse kat.toml!")?;
        Ok(kat_config)
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub kattisrc: Kattisrc,
    pub kat_config: KatConfig,
}

impl Config {
    pub fn new() -> Result<Self, Report> {
        let kattisrc = Kattisrc::new()?;
        let kat_config = KatConfig::new()?;
        Ok(Config { kattisrc, kat_config })
    }
}