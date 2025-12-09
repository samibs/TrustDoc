use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "heading")]
    Heading {
        level: u8,
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
    },
    #[serde(rename = "paragraph")]
    Paragraph {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
    },
    #[serde(rename = "list")]
    List {
        ordered: bool,
        items: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
    },
    #[serde(rename = "table")]
    Table {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        columns: Vec<TableColumn>,
        rows: Vec<TableRow>,
        #[serde(skip_serializing_if = "Option::is_none")]
        footer: Option<Vec<String>>,
    },
    #[serde(rename = "diagram")]
    Diagram {
        id: String,
        diagram_type: DiagramType,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        nodes: Vec<DiagramNode>,
        edges: Vec<DiagramEdge>,
        #[serde(skip_serializing_if = "Option::is_none")]
        layout: Option<DiagramLayout>,
    },
    #[serde(rename = "figure")]
    Figure {
        id: String,
        asset: String,
        alt: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
    },
    #[serde(rename = "footnote")]
    Footnote {
        id: String,
        text: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableColumn {
    pub id: String,
    pub header: String,
    #[serde(rename = "type")]
    pub cell_type: CellType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableRow {
    #[serde(flatten)]
    pub cells: HashMap<String, CellValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CellValue {
    Text(String),
    Number {
        raw: f64,
        display: String,
    },
    Currency {
        raw: f64,
        display: String,
        currency: String,
    },
    Percentage {
        raw: f64,
        display: String,
    },
    Date {
        raw: String,
        display: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CellType {
    Text,
    Number,
    Currency,
    Percentage,
    Date,
    Formula,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DiagramType {
    Hierarchical,
    Flowchart,
    Relationship,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagramNode {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<DiagramShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DiagramShape {
    Box,
    Circle,
    Diamond,
    Rounded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagramEdge {
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub edge_type: EdgeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EdgeType {
    Solid,
    Dashed,
    Dotted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagramLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<LayoutDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<LayoutSpacing>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum LayoutDirection {
    TopDown,
    LeftRight,
    BottomUp,
    RightLeft,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LayoutSpacing {
    Compact,
    Normal,
    Wide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentContent {
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub content: Vec<ContentBlock>,
}

