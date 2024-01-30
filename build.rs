use clap::CommandFactory;
use clap_complete::{generate_to, Shell::Bash, Shell::Fish, Shell::Zsh};
use std::{env, error::Error};

include!("src/cli.rs");

fn main() -> Result<(), Box<dyn Error>> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = Cli::command();

    let shells = [Bash, Zsh, Fish];

    for &shell in shells.iter() {
        generate_to(shell, &mut cmd, "kat", &outdir)?;
        println!("Generated completion file for {} shell", shell);
    }

    Ok(())
}
