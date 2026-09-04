use crate::error::{FleetError, Result};
use crate::models::common::validate_upload_filename;
use serde::Serialize;
use serde_json::Value;
use std::sync::OnceLock;

/// Request body supported by the generated raw API surface.
#[derive(Clone)]
pub enum ApiRequestBody {
    Json(Value),
    Bytes {
        bytes: Vec<u8>,
        content_type: Option<String>,
    },
    Multipart(ApiMultipartBody),
}

impl ApiRequestBody {
    pub fn new(value: impl Into<Value>) -> Self {
        Self::json(value)
    }

    pub fn json(value: impl Into<Value>) -> Self {
        Self::Json(value.into())
    }

    pub fn serialize_json(value: &impl Serialize) -> Result<Self> {
        Ok(Self::Json(serde_json::to_value(value)?))
    }

    pub fn bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::Bytes {
            bytes: bytes.into(),
            content_type: None,
        }
    }

    pub fn bytes_with_content_type(
        bytes: impl Into<Vec<u8>>,
        content_type: impl Into<String>,
    ) -> Self {
        Self::Bytes {
            bytes: bytes.into(),
            content_type: Some(content_type.into()),
        }
    }

    pub fn multipart(body: ApiMultipartBody) -> Self {
        Self::Multipart(body)
    }

    pub(crate) fn apply(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::RequestBuilder> {
        match self {
            Self::Json(value) => Ok(request.json(value)),
            Self::Bytes {
                bytes,
                content_type,
            } => {
                let request = request.body(bytes.clone());
                match content_type {
                    Some(content_type) => {
                        Ok(request.header(reqwest::header::CONTENT_TYPE, content_type))
                    }
                    None => Ok(request),
                }
            }
            Self::Multipart(body) => Ok(request.multipart(body.to_form()?)),
        }
    }
}

impl From<Value> for ApiRequestBody {
    fn from(value: Value) -> Self {
        Self::Json(value)
    }
}

#[derive(Clone, Default)]
pub struct ApiMultipartBody {
    fields: Vec<ApiMultipartField>,
}

impl ApiMultipartBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.push(ApiMultipartField::Text {
            name: name.into(),
            value: value.into(),
        });
        self
    }

    pub fn file(
        mut self,
        name: impl Into<String>,
        filename: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Self {
        self.fields.push(ApiMultipartField::File {
            name: name.into(),
            filename: filename.into(),
            content_type: None,
            bytes: bytes.into(),
        });
        self
    }

    pub fn file_with_content_type(
        mut self,
        name: impl Into<String>,
        filename: impl Into<String>,
        content_type: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Self {
        self.fields.push(ApiMultipartField::File {
            name: name.into(),
            filename: filename.into(),
            content_type: Some(content_type.into()),
            bytes: bytes.into(),
        });
        self
    }

    fn to_form(&self) -> Result<reqwest::multipart::Form> {
        let mut form = reqwest::multipart::Form::new();
        for field in &self.fields {
            match field {
                ApiMultipartField::Text { name, value } => {
                    validate_multipart_name(name)?;
                    form = form.text(name.clone(), value.clone());
                }
                ApiMultipartField::File {
                    name,
                    filename,
                    content_type,
                    bytes,
                } => {
                    validate_multipart_name(name)?;
                    validate_upload_filename(filename)?;
                    let mut part =
                        reqwest::multipart::Part::bytes(bytes.clone()).file_name(filename.clone());
                    if let Some(content_type) = content_type {
                        part = part.mime_str(content_type).map_err(|error| {
                            FleetError::Config(format!("invalid multipart content type: {error}"))
                        })?;
                    }
                    form = form.part(name.clone(), part);
                }
            }
        }
        Ok(form)
    }
}

fn validate_multipart_name(name: &str) -> Result<()> {
    if name.trim().is_empty()
        || name.trim() != name
        || name.chars().any(char::is_control)
        || name.contains(['"', '\\'])
    {
        return Err(FleetError::Validation(
            "multipart field name contains unsafe characters".into(),
        ));
    }
    Ok(())
}

#[derive(Clone)]
enum ApiMultipartField {
    Text {
        name: String,
        value: String,
    },
    File {
        name: String,
        filename: String,
        content_type: Option<String>,
        bytes: Vec<u8>,
    },
}

/// Lossless response returned by generated route wrappers.
#[derive(Clone)]
pub struct ApiResponse {
    status: reqwest::StatusCode,
    headers: reqwest::header::HeaderMap,
    bytes: Vec<u8>,
    body: OnceLock<Value>,
}

impl ApiResponse {
    pub fn new(body: Value) -> Self {
        let bytes = if body.is_null() {
            Vec::new()
        } else {
            serde_json::to_vec(&body).unwrap_or_default()
        };
        Self {
            status: reqwest::StatusCode::OK,
            headers: reqwest::header::HeaderMap::new(),
            bytes,
            body: OnceLock::from(body),
        }
    }

    pub(crate) fn from_parts(
        status: reqwest::StatusCode,
        headers: reqwest::header::HeaderMap,
        bytes: Vec<u8>,
    ) -> Self {
        Self {
            status,
            headers,
            bytes,
            body: OnceLock::new(),
        }
    }

    pub fn status(&self) -> reqwest::StatusCode {
        self.status
    }

    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn body(&self) -> &Value {
        self.body.get_or_init(|| decode_body(&self.bytes))
    }

    pub fn into_body(self) -> Value {
        self.body
            .into_inner()
            .unwrap_or_else(|| decode_body(&self.bytes))
    }

    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        Ok(serde_json::from_slice(&self.bytes)?)
    }
}

fn decode_body(bytes: &[u8]) -> Value {
    if bytes.is_empty() {
        Value::Null
    } else if let Ok(json) = serde_json::from_slice(bytes) {
        json
    } else if let Ok(text) = std::str::from_utf8(bytes) {
        Value::String(text.to_owned())
    } else {
        Value::Null
    }
}

/// Query parameter collection used by generated route wrappers.
pub type ApiQuery<'a> = &'a [(&'a str, &'a str)];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_multipart_rejects_unsafe_content_disposition_values() {
        let unsafe_filename = ApiMultipartBody::new().file("file", "../secret", b"value");
        assert!(matches!(
            unsafe_filename.to_form(),
            Err(FleetError::Validation(_))
        ));

        let unsafe_name = ApiMultipartBody::new().text("field\r\nname", "value");
        assert!(matches!(
            unsafe_name.to_form(),
            Err(FleetError::Validation(_))
        ));
    }
}
