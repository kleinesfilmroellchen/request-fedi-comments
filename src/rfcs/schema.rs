//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.6
//!

#![allow(dead_code)]
#![allow(unused_imports)]
use log::{debug, warn};
use std::io::{Read, Write};
use yaserde_derive::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(rename = "Fault", namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/", prefix = "soapenv")]
pub struct SoapFault {
	#[yaserde(rename = "faultcode", default)]
	pub fault_code: Option<String>,
	#[yaserde(rename = "faultstring", default)]
	pub fault_string: Option<String>,
}
impl std::error::Error for SoapFault {}

impl std::fmt::Display for SoapFault {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match (&self.fault_code, &self.fault_string) {
			(None, None) => Ok(()),
			(None, Some(fault_string)) => f.write_str(fault_string),
			(Some(fault_code), None) => f.write_str(fault_code),
			(Some(fault_code), Some(fault_string)) => {
				f.write_str(fault_code)?;
				f.write_str(": ")?;
				f.write_str(fault_string)
			},
		}
	}
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
	use super::*;
	use async_trait::async_trait;
	use yaserde::de::from_str;
	use yaserde::ser::to_string;
	use yaserde::{YaDeserialize, YaSerialize};
}

pub mod types {
	use super::*;
	use async_trait::async_trait;
	use yaserde::de::from_str;
	use yaserde::ser::to_string;
	use yaserde::{YaDeserialize, YaSerialize};
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "documentId", namespace = " : http://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct DocumentId {
		#[yaserde(text, default)]
		pub body: String,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "status", namespace = " : http://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct Status {
		#[yaserde(text, default)]
		pub body: String,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "fileFormat", namespace = " : http://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct FileFormat {
		#[yaserde(text, default)]
		pub body: String,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "dayOfMonth", namespace = " : http://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct DayOfMonth {
		#[yaserde(default)]
		pub body: i32,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "monthName", namespace = " : http://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct MonthName {
		#[yaserde(text, default)]
		pub body: String,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "documentRef", namespace = " : http://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct DocumentRef {
		#[yaserde(rename = "doc-id", prefix = " ", default)]
		pub doc_id: Vec<DocumentId>,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "stream", namespace = " : http://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct Stream {
		#[yaserde(text, default)]
		pub body: String,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "doi", namespace = " : http://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct Doi {
		#[yaserde(text, default)]
		pub body: String,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "author",
		namespace = " : http://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct Author {
		#[yaserde(rename = "name", prefix = " ", default)]
		pub name: String,
		#[yaserde(rename = "title", prefix = " ", default)]
		pub title: Option<String>,
		#[yaserde(rename = "organization", prefix = " ", default)]
		pub organization: Option<String>,
		#[yaserde(rename = "org-abbrev", prefix = " ", default)]
		pub org_abbrev: Option<String>,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "date",
		namespace = " : http://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct Date {}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "format",
		namespace = " : http://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct Format {
		#[yaserde(rename = "file-format", prefix = " ", default)]
		pub file_format: Vec<FileFormat>,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "keywords",
		namespace = " : http://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct Keywords {
		#[yaserde(rename = "kw", prefix = " ", default)]
		pub kw: Vec<String>,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "std-entry",
		namespace = " : http://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct StdEntry {
		#[yaserde(rename = "doc-id", prefix = " ", default)]
		pub doc_id: DocumentId,
		#[yaserde(rename = "title", prefix = " ", default)]
		pub title: String,
		#[yaserde(rename = "is-also", prefix = " ", default)]
		pub is_also: Option<DocumentRef>,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "bcp-entry",
		namespace = " : http://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct BcpEntry {
		#[yaserde(rename = "doc-id", prefix = " ", default)]
		pub doc_id: DocumentId,
		#[yaserde(rename = "title", prefix = " ", default)]
		pub title: Option<String>,
		#[yaserde(rename = "is-also", prefix = " ", default)]
		pub is_also: Option<DocumentRef>,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "fyi-entry",
		namespace = " : http://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct FyiEntry {
		#[yaserde(rename = "doc-id", prefix = " ", default)]
		pub doc_id: DocumentId,
		#[yaserde(rename = "title", prefix = " ", default)]
		pub title: Option<String>,
		#[yaserde(rename = "is-also", prefix = " ", default)]
		pub is_also: Option<DocumentRef>,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "abstract", namespace = " : http://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct Abstract {
		#[yaserde(rename = "p", prefix = " ", default)]
		pub p: Vec<String>,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "rfc-entry",
		namespace = " : http://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct RfcEntry {
		#[yaserde(rename = "doc-id", prefix = " ", default)]
		pub doc_id: DocumentId,
		#[yaserde(rename = "title", prefix = " ", default)]
		pub title: String,
		#[yaserde(rename = "page-count", prefix = " ", default)]
		pub page_count: Option<i32>,
		#[yaserde(rename = "abstract", prefix = " ", default)]
		pub r#abstract: Option<Abstract>,
		#[yaserde(rename = "draft", prefix = " ", default)]
		pub draft: Option<String>,
		#[yaserde(rename = "notes", prefix = " ", default)]
		pub notes: Option<String>,
		#[yaserde(rename = "obsoletes", prefix = " ", default)]
		pub obsoletes: Option<DocumentRef>,
		#[yaserde(rename = "obsoleted-by", prefix = " ", default)]
		pub obsoleted_by: Option<DocumentRef>,
		#[yaserde(rename = "updates", prefix = " ", default)]
		pub updates: Option<DocumentRef>,
		#[yaserde(rename = "updated-by", prefix = " ", default)]
		pub updated_by: Option<DocumentRef>,
		#[yaserde(rename = "is-also", prefix = " ", default)]
		pub is_also: Option<DocumentRef>,
		#[yaserde(rename = "see-also", prefix = " ", default)]
		pub see_also: Option<DocumentRef>,
		#[yaserde(rename = "current-status", prefix = " ", default)]
		pub current_status: Status,
		#[yaserde(rename = "publication-status", prefix = " ", default)]
		pub publication_status: Status,
		#[yaserde(rename = "stream", prefix = " ", default)]
		pub stream: Option<Stream>,
		#[yaserde(rename = "area", prefix = " ", default)]
		pub area: Option<String>,
		#[yaserde(rename = "wg_acronym", prefix = " ", default)]
		pub wg_acronym: Option<String>,
		#[yaserde(rename = "errata-url", prefix = " ", default)]
		pub errata_url: Option<String>,
		#[yaserde(rename = "doi", prefix = " ", default)]
		pub doi: Option<Doi>,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "rfc-not-issued-entry",
		namespace = " : http://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct RfcNotIssuedEntry {
		#[yaserde(rename = "doc-id", prefix = " ", default)]
		pub doc_id: DocumentId,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "rfc-index",
		namespace = " : http://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct RfcIndex {
		#[yaserde(rename = "title", attribute)]
		pub title: Option<String>,
		#[yaserde(rename = "rfc-entry")]
		pub rfcs: Vec<RfcEntry>,
		#[yaserde(rename = "rfc-entry-not-issued")]
		pub not_issued_rfcs: Vec<RfcNotIssuedEntry>,
		#[yaserde(rename = "std-entry")]
		pub stds: Vec<RfcEntry>,
		#[yaserde(rename = "bcp-entry")]
		pub bcps: Vec<RfcEntry>,
		#[yaserde(rename = "fyi-entry")]
		pub fyis: Vec<RfcEntry>,
	}
}

pub mod ports {
	use super::*;
	use async_trait::async_trait;
	use yaserde::de::from_str;
	use yaserde::ser::to_string;
	use yaserde::{YaDeserialize, YaSerialize};
}

pub mod bindings {
	use super::*;
	use async_trait::async_trait;
	use yaserde::de::from_str;
	use yaserde::ser::to_string;
	use yaserde::{YaDeserialize, YaSerialize};
}

pub mod services {
	use super::*;
	use async_trait::async_trait;
	use yaserde::de::from_str;
	use yaserde::ser::to_string;
	use yaserde::{YaDeserialize, YaSerialize};
}
