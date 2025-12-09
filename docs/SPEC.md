# TDF (TrustDoc Financial) Format Specification

**Version:** 0.1.0-draft  
**Status:** Draft  
**License:** CC0 1.0 (Public Domain)

---

## 1. Overview

TDF is an open document format designed for financial and corporate documents requiring cryptographic integrity verification. The format prioritizes:

- **Tamper-evidence**: Any modification is detectable
- **Simplicity**: Parseable in an afternoon
- **Structured content**: Semantic extraction without reverse-engineering
- **Print fidelity**: Deterministic rendering
- **Compact size**: Modern formats only, strict limits

### 1.1 Design Principles

1. Security through math, not obscurity
2. Open format, open tooling
3. Semantic content first, presentation second
4. Offline verification (no server required)
5. Corporate-ready (audit trails, multi-party signatures)

---

## 2. File Structure

A TDF file is a ZIP archive with the `.tdf` extension.

```
document.tdf (ZIP)
├── manifest.cbor           # Required: metadata and version
├── content.cbor            # Required: semantic document content
├── styles.css              # Required: presentation rules
├── layout.cbor             # Optional: fixed positioning for print
├── data.json               # Optional: machine-readable data extract
├── hashes.bin              # Required: Merkle tree
├── signatures.cbor         # Required: at least one signature
└── assets/                 # Optional: embedded resources
    ├── images/
    │   └── *.webp, *.avif, *.png
    └── fonts/
        └── *.woff2
```

### 2.1 MIME Type

```
application/vnd.trustdoc.financial+zip
```

### 2.2 File Extension

```
.tdf
```

---

## 3. Manifest

The manifest contains document metadata and integrity anchors.

**File:** `manifest.cbor`

```yaml
schema_version: "0.1.0"
document:
  id: "uuid-v4"
  title: "Q2 2025 Financial Report"
  language: "en"
  created: "2025-06-15T10:30:00Z"
  modified: "2025-06-15T14:22:00Z"
  
authors:
  - id: "did:web:cfo.acme.com"
    name: "Jane Smith"
    role: "CFO"

classification: "confidential"  # public | internal | confidential | restricted

integrity:
  root_hash: "sha256:a1b2c3..."
  algorithm: "sha256"  # sha256 | blake3
```

---

## 4. Content Model

Content is semantic, not positional. The renderer decides layout.

**File:** `content.cbor`

### 4.1 Document Structure

```yaml
document:
  sections:
    - id: "sec-1"
      title: "Executive Summary"
      content: [...]
    - id: "sec-2"
      title: "Financial Statements"
      content: [...]
```

### 4.2 Content Primitives

#### 4.2.1 Text Blocks

```yaml
- type: heading
  level: 1  # 1-4
  text: "Revenue Analysis"
  id: "h-revenue"

- type: paragraph
  text: "Total revenue increased by 15% compared to Q1."
  id: "p-001"

- type: list
  ordered: true
  items:
    - "Operating revenue: €1.2M"
    - "Investment income: €340K"
  id: "list-001"
```

#### 4.2.2 Tables

Tables are semantic, not just grids. Cell types enable proper extraction.

```yaml
- type: table
  id: "tbl-revenue"
  caption: "Revenue by Region"
  columns:
    - id: "region"
      header: "Region"
      type: text
    - id: "q1"
      header: "Q1 2025"
      type: currency
      currency: "EUR"
    - id: "q2"
      header: "Q2 2025"
      type: currency
      currency: "EUR"
    - id: "change"
      header: "Change"
      type: percentage
  rows:
    - region: "EMEA"
      q1: { raw: 542000.00, display: "€542,000" }
      q2: { raw: 623000.00, display: "€623,000" }
      change: { raw: 0.149, display: "+14.9%" }
    - region: "APAC"
      q1: { raw: 318000.00, display: "€318,000" }
      q2: { raw: 401000.00, display: "€401,000" }
      change: { raw: 0.261, display: "+26.1%" }
  footer:
    - type: total
      cells: ["Total", "€860,000", "€1,024,000", "+19.1%"]
```

**Cell Types:**

| Type | Raw Value | Display |
|------|-----------|---------|
| `text` | string | string |
| `number` | float64 | formatted string |
| `currency` | float64 + ISO 4217 | locale string |
| `percentage` | float64 (0-1) | formatted string |
| `date` | ISO 8601 | locale string |
| `formula` | expression | computed value |

#### 4.2.3 Diagrams

Diagrams are structured data, not images. Enables extraction and re-rendering.

```yaml
- type: diagram
  id: "diag-org"
  diagram_type: hierarchical  # hierarchical | flowchart | relationship
  title: "Executive Team"
  nodes:
    - id: "ceo"
      label: "CEO\nJane Smith"
      shape: box  # box | circle | diamond | rounded
      style: "primary"
    - id: "cfo"
      label: "CFO\nBob Lee"
      shape: box
    - id: "cto"
      label: "CTO\nAlice Chen"
      shape: box
  edges:
    - from: "ceo"
      to: "cfo"
      type: solid  # solid | dashed | dotted
      label: null
    - from: "ceo"
      to: "cto"
      type: solid
  layout:
    direction: top-down  # top-down | left-right | bottom-up | right-left
    spacing: normal  # compact | normal | wide
```

**Diagram Types:**

| Type | Use Case |
|------|----------|
| `hierarchical` | Org charts, tree structures |
| `flowchart` | Processes, decision trees |
| `relationship` | Entity relationships, dependencies |

#### 4.2.4 Figures

For non-diagrammatic images.

```yaml
- type: figure
  id: "fig-001"
  asset: "assets/images/chart-q2.webp"
  alt: "Q2 revenue chart showing 15% growth"
  caption: "Figure 1: Revenue trend Q1-Q2 2025"
  width: 600  # pixels, optional
```

#### 4.2.5 References and Footnotes

```yaml
- type: paragraph
  text: "According to IFRS standards{{fn:1}}, revenue recognition..."
  
- type: footnote
  id: "fn:1"
  text: "International Financial Reporting Standards, Section 15.2"
```

---

## 5. Styles

Presentation rules using a CSS subset.

**File:** `styles.css`

### 5.1 Supported CSS Properties

```css
/* Typography */
font-family, font-size, font-weight, font-style
line-height, letter-spacing, text-align, text-indent

/* Colors */
color, background-color

/* Spacing */
margin, padding (all variants)

/* Borders */
border, border-radius (all variants)

/* Tables */
border-collapse, border-spacing

/* Print */
page-break-before, page-break-after, page-break-inside
```

### 5.2 Predefined Classes

```css
.heading-1 { font-size: 24pt; font-weight: bold; }
.heading-2 { font-size: 18pt; font-weight: bold; }
.heading-3 { font-size: 14pt; font-weight: bold; }
.heading-4 { font-size: 12pt; font-weight: bold; }

.table { border-collapse: collapse; }
.table-header { background-color: #f5f5f5; font-weight: bold; }
.table-cell { padding: 8px; border: 1px solid #ddd; }

.diagram { margin: 16px 0; }
.diagram-node { /* renderer-specific */ }
.diagram-edge { /* renderer-specific */ }

.currency-positive { color: #2e7d32; }
.currency-negative { color: #c62828; }
```

### 5.3 Custom Styles

Documents may define custom styles that extend defaults.

```css
/* Custom in styles.css */
.company-header { 
  color: #1a237e; 
  border-bottom: 2px solid #1a237e; 
}
```

---

## 6. Fixed Layout (Print)

Optional positioning for print-deterministic output.

**File:** `layout.cbor`

```yaml
version: 1
pages:
  size: "A4"  # A4 | letter | legal | custom
  orientation: portrait  # portrait | landscape
  margins:
    top: 25mm
    bottom: 25mm
    left: 20mm
    right: 20mm

elements:
  - ref: "h-revenue"
    page: 1
    position: { x: 20mm, y: 30mm }
    
  - ref: "tbl-revenue"
    page: 1
    position: { x: 20mm, y: 60mm }
    size: { width: 170mm, height: auto }
    
  - ref: "diag-org"
    page: 2
    position: { x: 20mm, y: 30mm }
    size: { width: 170mm, height: 120mm }
```

---

## 7. Data Extract

Machine-readable structured data.

**File:** `data.json`

```json
{
  "schema": "tdf-data-v1",
  "extracted": "2025-06-15T14:22:00Z",
  "tables": {
    "tbl-revenue": {
      "columns": ["region", "q1", "q2", "change"],
      "rows": [
        ["EMEA", 542000.00, 623000.00, 0.149],
        ["APAC", 318000.00, 401000.00, 0.261]
      ],
      "currency": "EUR"
    }
  },
  "metrics": {
    "total_revenue_q2": 1024000.00,
    "revenue_growth": 0.191
  },
  "entities": {
    "people": [
      {"name": "Jane Smith", "role": "CEO"},
      {"name": "Bob Lee", "role": "CFO"},
      {"name": "Alice Chen", "role": "CTO"}
    ]
  },
  "diagrams": {
    "diag-org": {
      "type": "hierarchical",
      "node_count": 3,
      "edge_count": 2
    }
  }
}
```

---

## 8. Integrity Model

### 8.1 Merkle Tree Structure

**File:** `hashes.bin`

Binary format for compactness:

```
[4 bytes]   Magic: "TDFH"
[1 byte]    Version: 0x01
[1 byte]    Algorithm: 0x01 (SHA-256) | 0x02 (BLAKE3)
[4 bytes]   Node count (uint32, big-endian)
[32 bytes]  Root hash
[...]       Leaf hashes (32 bytes each)
```

### 8.2 Hash Computation

Each component is hashed independently:

```
root_hash
├── manifest_hash     = hash(manifest.cbor)
├── content_hash      = hash(content.cbor)
├── styles_hash       = hash(styles.css)
├── layout_hash       = hash(layout.cbor) or null_hash
├── data_hash         = hash(data.json) or null_hash
└── assets_hash
    ├── hash(assets/images/*)
    └── hash(assets/fonts/*)
```

### 8.3 Verification Algorithm

```python
def verify(tdf_file):
    # 1. Extract archive
    # 2. Recompute all leaf hashes
    # 3. Rebuild Merkle tree
    # 4. Compare root hash to stored root
    # 5. Verify signature over root hash
    # 6. Return: valid | invalid (with details)
```

---

## 9. Signatures

**File:** `signatures.cbor`

```yaml
signatures:
  - version: 1
    signer:
      id: "did:web:cfo.acme.com"
      name: "Jane Smith"
      certificate: "base64..."  # Optional X.509 for PKI compat
    timestamp:
      time: "2025-06-15T14:22:00Z"
      authority: "did:web:timestamp.digicert.com"
      proof: "base64..."
    scope: "full"  # full | content-only | sections:[...]
    algorithm: "Ed25519"  # Ed25519 | secp256k1 | RSA-PSS
    root_hash: "sha256:a1b2c3..."
    signature: "base64..."
```

### 9.1 Supported Signature Algorithms

| Algorithm | Key Size | Use Case |
|-----------|----------|----------|
| Ed25519 | 256-bit | Default, fast, modern |
| secp256k1 | 256-bit | Web3/blockchain compatibility |
| RSA-PSS | 2048+ bit | Legacy PKI compatibility |

### 9.2 Multi-Party Signatures

Multiple signatures are stored as array entries. Order preserved.

```yaml
signatures:
  - signer: { id: "did:web:cfo.acme.com" }
    timestamp: { time: "2025-06-15T14:22:00Z" }
    # ...
  - signer: { id: "did:web:ceo.acme.com" }
    timestamp: { time: "2025-06-15T15:01:00Z" }
    # ...
```

### 9.3 Timestamp Authorities

Trusted timestamping prevents backdating.

Supported protocols:
- RFC 3161 (traditional TSA)
- DID-based authorities
- Blockchain anchoring (optional)

---

## 10. Assets

### 10.1 Images

**Location:** `assets/images/`

| Format | Required Support | Max Size |
|--------|------------------|----------|
| WebP | Yes | 5 MB |
| AVIF | Yes | 5 MB |
| PNG | Yes (lossless needs) | 2 MB |

Total image budget: **10 MB**

### 10.2 Fonts

**Location:** `assets/fonts/`

| Format | Required Support |
|--------|------------------|
| WOFF2 | Yes |

Total font budget: **500 KB**

Fonts must be subsetted to characters used in document.

---

## 11. Size Tiers

Conforming implementations must declare supported tier.

| Tier | Max Size | Use Case |
|------|----------|----------|
| Micro | 256 KB | Invoices, receipts, simple contracts |
| Standard | 5 MB | Reports, proposals, statements |
| Extended | 50 MB | Annual reports, technical manuals |

---

## 12. Conformance Levels

### 12.1 TDF/Core

Minimum viable implementation:
- Parse manifest, content, styles
- Render text, tables
- Verify integrity and signatures
- Reflowable output

### 12.2 TDF/Print

Adds:
- Fixed layout support
- Deterministic rendering
- PDF export

### 12.3 TDF/Full

Adds:
- Diagram rendering
- Data extraction
- Multi-party signature verification
- Timestamp authority validation

---

## 13. Security Considerations

### 13.1 What TDF Guarantees

- Integrity: Any modification is detectable
- Attribution: Signer identity is verifiable
- Timestamp: Document existence at a point in time

### 13.2 What TDF Does Not Guarantee

- Confidentiality: Content is not encrypted
- Access control: Anyone with the file can read it
- Availability: No distribution mechanism specified

### 13.3 Implementation Requirements

- Reject documents with invalid signatures
- Reject documents with hash mismatches
- Validate timestamp proofs when present
- Warn on expired or revoked signer certificates

---

## 14. IANA Considerations

### 14.1 Media Type Registration

```
Type name: application
Subtype name: vnd.trustdoc.financial+zip
Required parameters: none
Optional parameters: none
Encoding considerations: binary
Security considerations: See Section 13
```

### 14.2 File Extension

```
Extension: .tdf
```

---

## Appendix A: CBOR Schema

Formal CDDL schemas for CBOR structures.

```cddl
manifest = {
  schema_version: tstr,
  document: document-meta,
  authors: [* author],
  ? classification: classification-level,
  integrity: integrity-block
}

document-meta = {
  id: tstr,
  title: tstr,
  language: tstr,
  created: tstr,    ; ISO 8601
  modified: tstr    ; ISO 8601
}

author = {
  id: tstr,         ; DID
  name: tstr,
  ? role: tstr
}

classification-level = "public" / "internal" / "confidential" / "restricted"

integrity-block = {
  root_hash: tstr,
  algorithm: "sha256" / "blake3"
}
```

---

## Appendix B: Example Document

See `examples/quarterly-report.tdf` in reference implementation.

---

## Appendix C: Changelog

### v0.1.0 (Draft)
- Initial specification
