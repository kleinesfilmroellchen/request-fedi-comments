#![doc = include_str!("../README.md")]
#![deny(clippy::all, rustdoc::all, unused, missing_docs)]

mod error;
mod fedi;
mod rfcs;

use error::Error;
use fedi::post_rfc;
use log::{debug, error, info};
use rfcs::fetch_random_rfc;
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

	let bot_cron_job = Job::new_repeated_async(Duration::from_secs(60 * 60), |_, _| {
		Box::pin(async {
			let result = run_bot().await;
			match result {
				Ok(_) => info!("Scheduled posting successful."),
				Err(why) => error!("Error while posting scheduled post: {}", why),
			}
		})
	})
	.unwrap();

	let scheduler = JobScheduler::new().await?;
	scheduler.add(bot_cron_job).await.unwrap();
	scheduler.start().await?;

	debug!("Scheduled hourly chron job.");

	// Loop forever.
	loop {
		tokio::time::sleep(Duration::from_secs(10_000_000)).await;
	}
}

/// Run the bot once, posting an RFC to the Fediverse.
async fn run_bot() -> Result<(), Error> {
	let client = fedi::create_client()?;
	let rfc = fetch_random_rfc().await?;
	debug!("Retrieved RFC {} for posting", rfc.doc_id.body.strip_prefix("RFC").unwrap());
	let url = post_rfc(&client, rfc)?;
	debug!("Posted RFC: {}", url);
	Ok(())
}

fn enter_correct_directory() -> Result<(), Error> {
	let mut args = std::env::args();
	let executable_name = args.next().unwrap();
	let target_directory = args.next().map(|d| OsString::from(d)).unwrap_or_else(|| {
		// Executable *should* live at target/release/executable, so go up 3 parents.
		Path::new(&executable_name).canonicalize().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().to_owned().into()
	});
	std::env::set_current_dir(target_directory)?;
	Ok(())
}
