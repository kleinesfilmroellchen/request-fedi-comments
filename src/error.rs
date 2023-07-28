//! Unified error type.

use std::sync::Arc;

use tokio_cron_scheduler::JobSchedulerError;

/// Unified error type for anything that can go wrong.
#[derive(Clone, Debug)]
pub enum Error {
	/// Reqwest errors from the internet.
	Reqwest(Arc<reqwest::Error>),
	/// Basic I/O errors.
	Io(Arc<std::io::Error>),
	/// XML parsing errors.
	Xml(String),
	/// Environment variable retrieval errors.
	Var(std::env::VarError),
	/// Elefren/Fediverse client API errors.
	Fedi(Arc<elefren::Error>),
	/// Job scheduling errors.
	Scheduler(JobSchedulerError),
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

impl From<JobSchedulerError> for Error {
	fn from(value: JobSchedulerError) -> Self {
		Self::Scheduler(value)
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

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Reqwest(underlying) => write!(f, "Networking: {}", underlying),
			Self::Io(underlying) => write!(f, "I/O: {}", underlying),
			Self::Xml(underlying) => write!(f, "XML: {}", underlying),
			Self::Var(underlying) => write!(f, "Environment: {}", underlying),
			Self::Fedi(underlying) => write!(f, "Fediverse: {}", underlying),
			Self::Scheduler(underlying) => write!(f, "Scheduling: {}", underlying),
		}
	}
}
