//! Draw an RFC dependency graph.

mod error;
mod fedi;
mod rfcs;

use error::Error;
use rfcs::fetch_rfc_list;
use rfcs::schema::types::DocumentRef;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
	let rfcs = fetch_rfc_list().await?;

	println!("digraph rfc_depgraph {{");

	for rfc in rfcs {
		println!("{0} [label=\"{0}\"];", rfc.doc_id.body);
		if let Some(DocumentRef { doc_id, .. }) = rfc.updated_by {
			for id in doc_id {
				let updated_by_id = id.body;
				println!("{} -> {};", rfc.doc_id.body, updated_by_id);
			}
		}
	}

	println!("}}");

	Ok(())
}
