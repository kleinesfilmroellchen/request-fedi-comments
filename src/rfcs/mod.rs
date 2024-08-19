//! RFC list retrieval and data extraction.

/// This file was created with zeep using the following basic procedure:
/// * Download the RFC index XSD from <https://www.rfc-editor.org/rfc-index.xsd>
/// * Run zeep as `zeep --input rfc-index.xsd --path . --output src/rfcs/schema.rs --ns ' ' --dns ' '`.
/// * Replace any fields called `abstract` with `r#abstract`
/// * Format the file with `cargo fmt`
/// Additionally, patching in the sequence children of rfc-index is necessary:
/// ```ignore
/// #[yaserde(rename = "rfc-entry")]
/// pub rfcs: Vec<RfcEntry>,
/// #[yaserde(rename = "rfc-not-issued-entry")]
/// pub not_issued_rfcs: Vec<RfcNotIssuedEntry>,
/// #[yaserde(rename = "std-entry")]
/// pub stds: Vec<RfcEntry>,
/// #[yaserde(rename = "bcp-entry")]
/// pub bcps: Vec<RfcEntry>,
/// #[yaserde(rename = "fyi-entry")]
/// pub fyis: Vec<RfcEntry>,
/// ```
pub mod schema;

pub use schema::types::RfcEntry;

use crate::error::Error;
use rand::prelude::*;
use schema::types::RfcIndex;

const RFC_INDEX_URL: &str = "https://www.rfc-editor.org/rfc-index.xml";

/// Fetches the RFC list from the public RFC index.
///
/// # Errors
/// Internet or I/O errors, as well as a misformatted XML (unlikely), are propagated.
pub async fn fetch_rfc_list() -> Result<Vec<RfcEntry>, Error> {
	Ok(yaserde::de::from_str::<RfcIndex>(&reqwest::get(RFC_INDEX_URL).await?.text().await?).map_err(Error::Xml)?.rfcs)
}

/// Fetches a random RFC from the public RFC index.
///
/// # Errors
/// Internet or I/O errors, as well as a misformatted XML (unlikely), are propagated.
#[allow(unused)]
pub async fn fetch_random_rfc() -> Result<RfcEntry, Error> {
	Ok(fetch_rfc_list().await?.into_iter().choose(&mut thread_rng()).unwrap())
}

/// Fetches a specific RFC for testing purposes. The document ID must be of the format "RFC1234" (always with 4 digits currently, use leading 0s if necessary).
///
/// # Errors
/// Internet or I/O errors, as well as a misformatted XML (unlikely), are propagated. An XML error is also synthesized if the document ID is invalid.
#[allow(unused)]
pub async fn fetch_specific_rfc(document_id: impl AsRef<str>) -> Result<RfcEntry, Error> {
	fetch_rfc_list()
		.await?
		.into_iter()
		.find(|rfc| rfc.doc_id.body == document_id.as_ref())
		.ok_or(Error::Xml("Document ID not found".to_owned()))
}
