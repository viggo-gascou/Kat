use crate::{
    App,
	cli::Config,
};

use color_eyre::{eyre, eyre::Context, Report};


pub async fn config(app: &App, args: &Config) -> Result<(), Report> {
	unimplemented!("This command has not been implemented yet");
}
