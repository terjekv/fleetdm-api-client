use std::fmt;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::error::{FleetError, Result};

/// Pagination information for list responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub has_next_results: bool,
    pub has_previous_results: bool,
}

/// Standard paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    #[serde(flatten)]
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

/// An in-memory file to upload to Fleet.
#[derive(Clone, PartialEq, Eq)]
pub struct FileUpload {
    filename: String,
    bytes: Vec<u8>,
    content_type: Option<String>,
}

impl FileUpload {
    /// Create an upload with a filename and file contents.
    pub fn new(filename: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Result<Self> {
        let filename = filename.into();
        let bytes = bytes.into();
        validate_upload_filename(&filename)?;
        if bytes.is_empty() {
            return Err(FleetError::Validation(
                "upload contents must not be empty".into(),
            ));
        }
        Ok(Self {
            filename,
            bytes,
            content_type: None,
        })
    }

    /// Set the MIME type sent with the multipart file part.
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Result<Self> {
        let content_type = content_type.into();
        if content_type.trim().is_empty() {
            return Err(FleetError::Validation(
                "upload content type must not be empty".into(),
            ));
        }
        self.content_type = Some(content_type);
        Ok(self)
    }

    pub(crate) fn to_part(&self) -> Result<reqwest::multipart::Part> {
        let part =
            reqwest::multipart::Part::bytes(self.bytes.clone()).file_name(self.filename.clone());
        match &self.content_type {
            Some(content_type) => part
                .mime_str(content_type)
                .map_err(|error| FleetError::Validation(format!("invalid content type: {error}"))),
            None => Ok(part),
        }
    }

    /// File name supplied to Fleet.
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// File contents supplied to Fleet.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Optional MIME type supplied to Fleet.
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }
}

pub(crate) fn validate_upload_filename(filename: &str) -> Result<()> {
    if filename.trim().is_empty() {
        return Err(FleetError::Validation(
            "upload filename must not be empty".into(),
        ));
    }
    if filename.trim() != filename
        || filename.len() > 255
        || filename == "."
        || filename == ".."
        || filename.contains(['/', '\\'])
        || filename.chars().any(char::is_control)
    {
        return Err(FleetError::Validation(
            "upload filename must be a safe basename of at most 255 bytes".into(),
        ));
    }
    Ok(())
}

impl fmt::Debug for FileUpload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FileUpload")
            .field("filename", &self.filename)
            .field("byte_len", &self.bytes.len())
            .field("content_type", &self.content_type)
            .finish_non_exhaustive()
    }
}

/// Binary file returned by Fleet, including useful response metadata.
#[derive(Clone)]
pub struct FileDownload {
    bytes: Vec<u8>,
    content_type: Option<String>,
    content_disposition: Option<String>,
}

/// A successful file response consumed one bounded chunk at a time.
///
/// This is used for software packages, which Fleet permits to be much larger
/// than is safe to buffer in memory.
pub struct FileDownloadStream {
    response: reqwest::Response,
    content_type: Option<String>,
    content_disposition: Option<String>,
    content_length: Option<u64>,
}

impl FileDownloadStream {
    pub(crate) fn from_response(response: reqwest::Response) -> Self {
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let content_disposition = response
            .headers()
            .get(reqwest::header::CONTENT_DISPOSITION)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let content_length = response.content_length();
        Self {
            response,
            content_type,
            content_disposition,
            content_length,
        }
    }

    /// Read the next response chunk. `None` indicates end-of-stream.
    pub async fn next_chunk(&mut self) -> Result<Option<Vec<u8>>> {
        Ok(self.response.chunk().await?.map(|chunk| chunk.to_vec()))
    }

    /// Response `Content-Type`, if Fleet supplied one.
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// Response `Content-Disposition`, if Fleet supplied one.
    pub fn content_disposition(&self) -> Option<&str> {
        self.content_disposition.as_deref()
    }

    /// Declared response size, if Fleet supplied a valid `Content-Length`.
    pub fn content_length(&self) -> Option<u64> {
        self.content_length
    }
}

impl fmt::Debug for FileDownload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FileDownload")
            .field("byte_len", &self.bytes.len())
            .field("content_type", &self.content_type)
            .field("content_disposition", &self.content_disposition)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod file_tests {
    use super::*;

    #[test]
    fn upload_rejects_unsafe_filenames() {
        for filename in ["../secret", "dir/file.pkg", "dir\\file.pkg", "file\r\nname"] {
            assert!(matches!(
                FileUpload::new(filename, b"contents"),
                Err(FleetError::Validation(_))
            ));
        }
    }

    #[test]
    fn file_debug_output_does_not_expose_contents() {
        let upload = FileUpload::new("app.pkg", b"super-secret-contents").unwrap();
        let download = FileDownload::from_parts(
            b"super-secret-contents".to_vec(),
            Some("application/octet-stream".into()),
            None,
        );

        assert!(!format!("{upload:?}").contains("super-secret-contents"));
        assert!(!format!("{download:?}").contains("super-secret-contents"));
    }
}

impl FileDownload {
    pub(crate) fn from_parts(
        bytes: Vec<u8>,
        content_type: Option<String>,
        content_disposition: Option<String>,
    ) -> Self {
        Self {
            bytes,
            content_type,
            content_disposition,
        }
    }

    /// Downloaded bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consume the response and return its bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Response `Content-Type`, if Fleet supplied one.
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// Response `Content-Disposition`, if Fleet supplied one.
    pub fn content_disposition(&self) -> Option<&str> {
        self.content_disposition.as_deref()
    }
}

/// Validate common list-query invariants before network I/O.
pub(crate) fn validate_list_options(
    per_page: Option<u32>,
    has_order_key: bool,
    has_order_direction: bool,
    has_after: bool,
) -> Result<()> {
    if per_page == Some(0) {
        return Err(FleetError::Validation(
            "per_page must be greater than zero".into(),
        ));
    }
    if has_order_direction && !has_order_key {
        return Err(FleetError::Validation(
            "order_direction requires order_key".into(),
        ));
    }
    if has_after && !has_order_key {
        return Err(FleetError::Validation("after requires order_key".into()));
    }
    Ok(())
}

/// Sort order for list queries
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, Display, EnumString,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum OrderDirection {
    #[default]
    Asc,
    Desc,
}

/// Common query parameters for list endpoints
#[derive(Debug, Default, Clone)]
pub struct ListQueryParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<String>,
    pub order_direction: Option<OrderDirection>,
}

impl ListQueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    pub fn order_key(mut self, key: impl Into<String>) -> Self {
        self.order_key = Some(key.into());
        self
    }

    pub fn order_direction(mut self, direction: OrderDirection) -> Self {
        self.order_direction = Some(direction);
        self
    }

    /// Convert to query string parameters
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(page) = self.page {
            params.push(("page".to_string(), page.to_string()));
        }
        if let Some(per_page) = self.per_page {
            params.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(ref order_key) = self.order_key {
            params.push(("order_key".to_string(), order_key.clone()));
        }
        if let Some(order_direction) = self.order_direction {
            let direction = match order_direction {
                OrderDirection::Asc => "asc",
                OrderDirection::Desc => "desc",
            };
            params.push(("order_direction".to_string(), direction.to_string()));
        }

        params
    }
}
