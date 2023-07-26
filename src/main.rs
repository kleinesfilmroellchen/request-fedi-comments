#![doc = include_str!("../README.md")]
#![deny(clippy::all, rustdoc::all, unused, missing_docs)]

mod error;
mod fedi;
mod rfcs;

use fedi::post_rfc;

use crate::rfcs::fetch_random_rfc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), error::Error> {
	dotenv::dotenv().ok();

	let client = fedi::create_client()?;
	let rfc = fetch_random_rfc().await?;
	println!("Retrieved RFC {} for posting", rfc.doc_id.body.strip_prefix("RFC").unwrap());
	let url = post_rfc(&client, rfc)?;
	println!("Posted RFC: {}", url);

	Ok(())
}
