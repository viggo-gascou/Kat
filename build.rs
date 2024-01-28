use clap::CommandFactory;
use clap_complete::{
    generate_to, Shell::Bash, Shell::Elvish, Shell::Fish, Shell::Zsh,
};
use std::{env, error::Error};

include!("src/cli.rs");

fn main() -> Result<(), Box<dyn Error>> {
    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions");
    if !outdir.exists() {
        std::fs::create_dir_all(&outdir)?;
    }

    let mut cmd = Cli::command();

    let shells = [Bash, Zsh, Fish, Elvish];

    for &shell in shells.iter() {
        generate_to(shell, &mut cmd, "kat", &outdir)?;
        println!("Generated completion file for {} shell", shell);
    }

    Ok(())
}
