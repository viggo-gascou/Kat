use crate::{
    App,
	cli::Test,
};

use color_eyre::{eyre, eyre::Context, Report};


pub async fn test(app: &App, args: &Test) -> Result<(), Report> {
	unimplemented!("This command has not been implemented yet");
}