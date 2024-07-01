#![doc = include_str!("../README.md")]
#![deny(clippy::all, rustdoc::all, unused, missing_docs)]

mod error;
mod fedi;
mod rfcs;

use error::Error;
use fedi::post_rfc;
use log::{debug, error, info};
#[allow(unused)]
use rfcs::fetch_random_rfc;
#[allow(unused)]
use rfcs::fetch_specific_rfc;
use std::{ffi::OsString, path::Path, time::Duration};
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
	enter_correct_directory().unwrap();

	dotenv::dotenv().ok();
	env_logger::init();
	info!("request-fedi-comments starting.");
	debug!("Now in directory {}", std::env::current_dir().unwrap_or_default().display());

	info!("Checking server authentication...");
	fedi::verify_client()?;
	info!("Authentication data okay.");

	let bot_cron_job = Job::new_repeated_async(Duration::from_secs(60 * 60), |_, _| Box::pin(try_run_bot())).unwrap();
	let scheduler = JobScheduler::new().await?;
	scheduler.add(bot_cron_job).await.unwrap();
	scheduler.start().await?;

	debug!("Scheduled hourly chron job.");

	try_run_bot().await;

	// Loop forever.
	loop {
		tokio::time::sleep(Duration::from_secs(10_000_000)).await;
	}
}

/// Run the bot once, posting an RFC to the Fediverse.
async fn run_bot() -> Result<(), Error> {
	let client = fedi::create_client()?;
	let rfc = fetch_random_rfc().await?; // fetch_specific_rfc("RFC1234").await?;
	debug!("Retrieved RFC {} for posting", rfc.doc_id.body.strip_prefix("RFC").unwrap());
	let url = post_rfc(&client, rfc)?;
	debug!("Posted RFC: {}", url);
	Ok(())
}

/// Run the bot and log its errors.
async fn try_run_bot() {
	let result = run_bot().await;
	match result {
		Ok(_) => info!("Scheduled posting successful."),
		Err(why) => error!("Error while posting scheduled post: {}", why),
	}
}

fn enter_correct_directory() -> Result<(), Error> {
	let mut args = std::env::args();
	let executable_name = args.next().unwrap();
	let target_directory = args.next().map(OsString::from).unwrap_or_else(|| {
		// Executable *should* live at target/release/executable, so go up 3 parents.
		Path::new(&executable_name)
			.canonicalize()
			.unwrap()
			.parent()
			.unwrap()
			.parent()
			.unwrap()
			.parent()
			.unwrap()
			.to_owned()
			.into()
	});
	std::env::set_current_dir(target_directory)?;
	Ok(())
}
