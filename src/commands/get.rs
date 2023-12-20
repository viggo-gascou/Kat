use crate::{
    App,
	cli::Get,
};

use color_eyre::{eyre, eyre::Context, Report};


pub async fn get(app: &App, args: &Get) -> Result<(), Report> {
	unimplemented!("This command has not been implemented yet");
}