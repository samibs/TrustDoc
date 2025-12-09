use crate::content::DocumentContent;
use crate::error::{TdfError, TdfResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub manifest: Manifest,
    pub content: DocumentContent,
    pub styles: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub schema_version: String,
    pub document: DocumentMeta,
    pub authors: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<Classification>,
    pub integrity: IntegrityBlock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMeta {
    pub id: String,
    pub title: String,
    pub language: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Classification {
    Public,
    Internal,
    Confidential,
    Restricted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityBlock {
    pub root_hash: String,
    pub algorithm: HashAlgorithm,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HashAlgorithm {
    Sha256,
    Blake3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub version: u8,
    pub pages: PageLayout,
    pub elements: Vec<LayoutElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageLayout {
    pub size: PageSize,
    pub orientation: Orientation,
    pub margins: Margins,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PageSize {
    A4,
    Letter,
    Legal,
    #[serde(rename = "CUSTOM")]
    Custom { width: String, height: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Portrait,
    Landscape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margins {
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutElement {
    pub ref_id: String,
    pub page: u32,
    pub position: Position,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: String,
    pub height: String,
}

impl Document {
    pub fn new(
        title: String,
        language: String,
        content: DocumentContent,
        styles: String,
    ) -> Self {
        let now = Utc::now();
        let document_id = uuid::Uuid::new_v4().to_string();

        let manifest = Manifest {
            schema_version: "0.1.0".to_string(),
            document: DocumentMeta {
                id: document_id,
                title: title.clone(),
                language,
                created: now,
                modified: now,
            },
            authors: Vec::new(),
            classification: None,
            integrity: IntegrityBlock {
                root_hash: String::new(),
                algorithm: HashAlgorithm::Sha256,
            },
        };

        Document {
            manifest,
            content,
            styles,
            layout: None,
            data: None,
        }
    }

    pub fn validate(&self) -> TdfResult<()> {
        if self.manifest.schema_version.is_empty() {
            return Err(TdfError::InvalidDocument(
                "Schema version is required".to_string(),
            ));
        }

        if self.manifest.document.id.is_empty() {
            return Err(TdfError::InvalidDocument("Document ID is required".to_string()));
        }

        if self.manifest.document.title.is_empty() {
            return Err(TdfError::InvalidDocument("Document title is required".to_string()));
        }

        if self.content.sections.is_empty() {
            return Err(TdfError::InvalidDocument(
                "Document must have at least one section".to_string(),
            ));
        }

        Ok(())
    }
}

