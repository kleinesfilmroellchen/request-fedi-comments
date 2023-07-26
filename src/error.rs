//! Unified error type.

use std::sync::Arc;

/// Unified error type for anything that can go wrong.
#[derive(Clone, Debug)]
pub enum Error {
	/// Reqwest errors from the internet.
	Reqwest(Arc<reqwest::Error>),
	/// Basic I/O errors.
	Io(Arc<std::io::Error>),
	/// XML parsing errors.
	XML(String),
	/// Environment variable retrieval errors.
	Var(std::env::VarError),
	/// Elefren/Fediverse client API errors.
	Fedi(Arc<elefren::Error>),
}

impl From<reqwest::Error> for Error {
	fn from(value: reqwest::Error) -> Self {
		Self::Reqwest(Arc::new(value))
	}
}

impl From<std::env::VarError> for Error {
	fn from(value: std::env::VarError) -> Self {
		Self::Var(value)
	}
}

impl From<elefren::Error> for Error {
	fn from(value: elefren::Error) -> Self {
		Self::Fedi(Arc::new(value))
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self {
		Self::Io(Arc::new(value))
	}
}
