//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.10
//!

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_local_definitions)]

use log::{debug, trace, warn};
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

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
	#[yaserde(rename = "documentId", namespace = " : https://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct DocumentId {
		#[yaserde(text, default)]
		pub body: String,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "status", namespace = " : https://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct Status {
		#[yaserde(text, default)]
		pub body: String,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "fileFormat", namespace = " : https://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct FileFormat {
		#[yaserde(text, default)]
		pub body: String,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "dayOfMonth", namespace = " : https://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct DayOfMonth {
		#[yaserde(default)]
		pub body: i32,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "monthName", namespace = " : https://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct MonthName {
		#[yaserde(text, default)]
		pub body: String,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "documentRef", namespace = " : https://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct DocumentRef {
		#[yaserde(rename = "doc-id", prefix = " ", default)]
		pub doc_id: Vec<DocumentId>,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "stream", namespace = " : https://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct Stream {
		#[yaserde(text, default)]
		pub body: String,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(rename = "doi", namespace = " : https://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct Doi {
		#[yaserde(text, default)]
		pub body: String,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "author",
		namespace = " : https://www.rfc-editor.org/rfc-index",
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
		namespace = " : https://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct Date {}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "format",
		namespace = " : https://www.rfc-editor.org/rfc-index",
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
		namespace = " : https://www.rfc-editor.org/rfc-index",
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
		namespace = " : https://www.rfc-editor.org/rfc-index",
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
		namespace = " : https://www.rfc-editor.org/rfc-index",
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
		namespace = " : https://www.rfc-editor.org/rfc-index",
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
	#[yaserde(rename = "abstract", namespace = " : https://www.rfc-editor.org/rfc-index", prefix = " ")]
	pub struct Abstract {
		#[yaserde(rename = "p", prefix = " ", default)]
		pub p: Vec<String>,
	}
	#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
	#[yaserde(
		rename = "rfc-entry",
		namespace = " : https://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct RfcEntry {
		#[yaserde(rename = "doc-id", prefix = " ", default)]
		pub doc_id: DocumentId,
		#[yaserde(rename = "title", prefix = " ", default)]
		pub title: Option<String>,
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
		pub current_status: Option<Status>,
		#[yaserde(rename = "publication-status", prefix = " ", default)]
		pub publication_status: Option<Status>,
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
		namespace = " : https://www.rfc-editor.org/rfc-index",
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
		namespace = " : https://www.rfc-editor.org/rfc-index",
		namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
		prefix = " "
	)]
	pub struct RfcIndex {
		#[yaserde(rename = "title", attribute)]
		pub title: Option<String>,
		#[yaserde(rename = "rfc-entry")]
		pub rfcs: Vec<RfcEntry>,
		#[yaserde(rename = "rfc-not-issued-entry")]
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

pub mod multiref {
	//! This module contains the `MultiRef` type which is a wrapper around `Rc<RefCell<T>>` that implements `YaDeserialize` and `YaSerialize` for `T` and allows for multiple references to the same object.
	//! Inspired by [this](https://github.com/media-io/yaserde/issues/165#issuecomment-1810243674) comment on the yaserde repository.
	//! Needs `xml-rs` and `yaserde` as dependencies.

	use std::{cell::RefCell, ops::Deref, rc::Rc};
	use yaserde::{YaDeserialize, YaSerialize};

	pub struct MultiRef<T> {
		inner: Rc<RefCell<T>>,
	}

	impl<T: YaDeserialize + YaSerialize> YaDeserialize for MultiRef<T> {
		fn deserialize<R: std::io::prelude::Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
			let inner = T::deserialize(reader)?;
			Ok(Self { inner: Rc::new(RefCell::new(inner)) })
		}
	}

	impl<T: YaDeserialize + YaSerialize> YaSerialize for MultiRef<T> {
		fn serialize<W: std::io::prelude::Write>(
			&self,
			writer: &mut yaserde::ser::Serializer<W>,
		) -> Result<(), String> {
			self.inner.as_ref().borrow().serialize(writer)?;
			Ok(())
		}

		fn serialize_attributes(
			&self,
			attributes: Vec<xml::attribute::OwnedAttribute>,
			namespace: xml::namespace::Namespace,
		) -> Result<(Vec<xml::attribute::OwnedAttribute>, xml::namespace::Namespace), String> {
			self.inner.as_ref().borrow().serialize_attributes(attributes, namespace)
		}
	}

	impl<T: YaDeserialize + YaSerialize + Default> Default for MultiRef<T> {
		fn default() -> Self {
			Self { inner: Rc::default() }
		}
	}

	impl<T: YaDeserialize + YaSerialize> Clone for MultiRef<T> {
		fn clone(&self) -> Self {
			Self { inner: self.inner.clone() }
		}
	}

	impl<T: YaDeserialize + YaSerialize + std::fmt::Debug> std::fmt::Debug for MultiRef<T> {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			self.inner.as_ref().borrow().fmt(f)
		}
	}

	impl<T> Deref for MultiRef<T> {
		type Target = Rc<RefCell<T>>;
		fn deref(&self) -> &Self::Target {
			&self.inner
		}
	}
}
