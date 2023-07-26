//! Fediverse bot interaction handling.

use elefren::prelude::*;

use std::env::var;

use crate::error::Error;
use crate::rfcs::RfcEntry;

/// Creates the client structure. We don't use any of elefren's app creation, registration, or OAuth authentication functionality,
/// instead this is a one-time (or repeated) manual process the user should execute.
pub fn create_client() -> Result<Mastodon, Error> {
	Ok(Mastodon::from(elefren::Data {
		base: var("instance_url")?.into(),
		client_id: var("client_id")?.into(),
		client_secret: var("client_secret")?.into(),
		redirect: "https://github.com/kleinesfilmroellchen/request-fedi-comments".into(),
		token: var("access_token")?.into(),
	}))
}

/// Create a nicely-formatted post text from an RFC.
pub fn rfc_to_post_text(rfc: RfcEntry, character_limit: usize) -> String {
	let number = rfc.doc_id.body.strip_prefix("RFC").unwrap().trim().to_owned();
	let mut description = rfc.r#abstract.map_or_else(|| "(no description)".into(), |a| a.p.join("\n")).trim().to_owned();
	let title = rfc.title.trim().to_owned();

	// Parts we can't abbreviate. Note that Mastodon adds a constant penalty of 23 characters for (valid) URLs.
	let unabbreviatable_length = 4 + number.chars().count().max(4) + 3 + 2 + 1 + 23;
	let description_length = description.chars().count();
	let length_overshoot = unabbreviatable_length + description_length + title.chars().count() - character_limit;
	// Trim description if necessary.
	if length_overshoot > 0 {
		description = description.chars().take(description_length - length_overshoot - 1).collect::<String>() + "…";
	}

	format!("RFC {number:04} – {title}\n\n{description}\nhttps://www.rfc-editor.org/rfc/rfc{number}.html")
}

/// Post an RFC via the given Fediverse client.
pub fn post_rfc(client: &Mastodon, rfc: RfcEntry) -> Result<String, Error> {
	let status = StatusBuilder::new()
		.status(rfc_to_post_text(
			rfc,
			usize::from_str_radix(&var("character_limit")?, 10)
				.map_err(|_| Error::Var(std::env::VarError::NotPresent))?,
		))
		.language(elefren::Language::Eng)
		.visibility(elefren::status_builder::Visibility::Public)
		.content_type("text/plain")
		.build()?;
	let status = client.new_status(status)?;
	Ok(status.uri)
}
