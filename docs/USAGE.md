# TDF Usage Guide

## Quick Start

### 1. Generate Signing Keys

```bash
tdf keygen --name my-keys
```

This creates:
- `my-keys.signing` - Private key (keep secure!)
- `my-keys.verifying` - Public key (can be shared)

### 2. Create a Document

Create a JSON file with your document content:

```json
{
  "title": "My Financial Report",
  "language": "en",
  "styles": "body { font-family: Arial; }",
  "sections": [
    {
      "id": "sec-1",
      "title": "Introduction",
      "content": [
        {
          "type": "paragraph",
          "text": "This is a test document."
        }
      ]
    }
  ]
}
```

Then create the TDF:

```bash
tdf create document.json -o report.tdf \
  --key my-keys.signing \
  --signer-id "did:web:mycompany.com" \
  --signer-name "Jane Doe"
```

### 3. Verify Document

```bash
# Basic integrity check
tdf verify report.tdf

# With signature verification
tdf verify report.tdf --key my-keys.verifying
```

### 4. Extract Data

```bash
tdf extract report.tdf -o data.json
```

### 5. View in Browser

Open `tdf-viewer/index.html` in a browser and drag-and-drop your `.tdf` file.

## Document Structure

### Sections

Documents are organized into sections:

```json
{
  "sections": [
    {
      "id": "unique-section-id",
      "title": "Section Title",
      "content": [ /* content blocks */ ]
    }
  ]
}
```

### Content Blocks

#### Paragraph

```json
{
  "type": "paragraph",
  "text": "Paragraph text here.",
  "id": "optional-id"
}
```

#### Heading

```json
{
  "type": "heading",
  "level": 1,
  "text": "Heading Text",
  "id": "h-1"
}
```

#### Table

```json
{
  "type": "table",
  "id": "tbl-revenue",
  "caption": "Revenue Table",
  "columns": [
    {
      "id": "region",
      "header": "Region",
      "type": "text"
    },
    {
      "id": "amount",
      "header": "Amount",
      "type": "currency",
      "currency": "EUR"
    }
  ],
  "rows": [
    {
      "region": { "raw": "EMEA", "display": "EMEA" },
      "amount": { "raw": 100000.00, "display": "€100,000", "currency": "EUR" }
    }
  ]
}
```

#### Diagram

```json
{
  "type": "diagram",
  "id": "diag-org",
  "diagram_type": "hierarchical",
  "nodes": [
    {
      "id": "ceo",
      "label": "CEO\nJane Smith",
      "shape": "box"
    }
  ],
  "edges": [
    {
      "from": "ceo",
      "to": "cfo",
      "type": "solid"
    }
  ]
}
```

## Security Best Practices

1. **Protect Signing Keys**: Never share `.signing` files. Store them securely.

2. **Verify Before Trust**: Always verify documents before trusting their content.

3. **Check Signatures**: Use `--key` option to verify signatures, not just integrity.

4. **Key Rotation**: Generate new keys periodically and revoke old ones.

5. **Backup Keys**: Securely backup signing keys (encrypted).

## File Size Limits

- **Micro**: 256 KB (invoices, receipts)
- **Standard**: 5 MB (reports, proposals)
- **Extended**: 50 MB (annual reports, manuals)

## Troubleshooting

### "Integrity check failed"

The document has been modified. Do not trust its contents.

### "Signature verification failed"

- Check that you're using the correct verifying key
- Verify the signer ID matches
- Document may have been tampered with

### "Missing required file in archive"

The TDF file is corrupted or incomplete. Re-create from source.

### CBOR parsing errors (TypeScript)

Ensure `cbor-web` is installed:
```bash
cd tdf-ts && npm install
```

