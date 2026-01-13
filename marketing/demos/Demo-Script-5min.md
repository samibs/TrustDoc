# TDF 5-Minute Demo Script
## Quick Overview for Busy Executives

---

## Setup (Before Demo)

```bash
# Navigate to TrustDoc directory
cd /path/to/TrustDoc

# Build if needed
cargo build --release

# Create demo directory
mkdir -p /tmp/tdf-demo
cd /tmp/tdf-demo
```

---

## DEMO SCRIPT

### Opening (30 seconds)

**SAY:**
> "I'm going to show you how TDF solves the document trust problem in 5 minutes. We'll create a financial document, sign it, and then prove that any tampering is instantly detected."

---

### Part 1: Generate Keys (30 seconds)

**SAY:**
> "First, let's generate a signing key. In production, this would be stored in an HSM or secure key vault."

**RUN:**
```bash
tdf keygen --output . --name cfo-key
```

**EXPECTED OUTPUT:**
```
Signing key (private) written to: ./cfo-key.signing
  ⚠️  Keep this file secure and never share it!
Verifying key (public) written to: ./cfo-key.verifying
  ✓  This file can be shared publicly
```

**SAY:**
> "We now have a keypair—private for signing, public for verification. The CFO keeps the private key secure."

---

### Part 2: Create Sample Document (30 seconds)

**SAY:**
> "Let's create a simple financial document. This could be imported from Excel, but I'll create a JSON file to show the structure."

**RUN:**
```bash
cat > financial-report.json << 'EOF'
{
  "title": "Q4 2025 Financial Summary",
  "language": "en",
  "styles": "",
  "sections": [
    {
      "id": "summary",
      "title": "Executive Summary",
      "content": [
        {
          "type": "heading",
          "level": 1,
          "text": "Q4 2025 Financial Results",
          "id": "h1"
        },
        {
          "type": "table",
          "id": "financials",
          "caption": "Key Metrics",
          "columns": [
            {"id": "metric", "header": "Metric", "type": "text"},
            {"id": "value", "header": "Amount (EUR)", "type": "currency", "currency": "EUR"}
          ],
          "rows": [
            {"metric": {"raw": "Revenue", "display": "Revenue"}, "value": {"raw": 1200000, "display": "€1,200,000"}},
            {"metric": {"raw": "Expenses", "display": "Expenses"}, "value": {"raw": 800000, "display": "€800,000"}},
            {"metric": {"raw": "Net Profit", "display": "Net Profit"}, "value": {"raw": 400000, "display": "€400,000"}}
          ]
        }
      ]
    }
  ]
}
EOF
```

**SAY:**
> "Notice this is structured data—actual numbers, not a picture. Revenue: €1.2 million, Expenses: €800K, Net Profit: €400K."

---

### Part 3: Create & Sign TDF (45 seconds)

**SAY:**
> "Now the CFO signs this document. The signature locks in the exact data."

**RUN:**
```bash
tdf create financial-report.json \
  --output q4-report.tdf \
  --key cfo-key.signing \
  --signer-id "did:web:cfo.acme.com" \
  --signer-name "CFO Jane Smith" \
  --timestamp-manual
```

**EXPECTED OUTPUT:**
```
Created TDF document with manual timestamp: q4-report.tdf
```

**SAY:**
> "The TDF file is created. It contains the data, a Merkle tree of hashes, and the CFO's digital signature. Let's verify it works."

---

### Part 4: Verify Document (45 seconds)

**SAY:**
> "Anyone with the public key can now verify this document. Watch."

**RUN:**
```bash
tdf verify q4-report.tdf --key cfo-key.verifying
```

**EXPECTED OUTPUT:**
```
TDF Verification Report
=======================
Document: q4-report.tdf
Security Tier: STANDARD (5 MB max, 1000:1 decompression ratio)

INTEGRITY: ✓ VALID
  Root Hash: [32-character hash]
  Algorithm: SHA-256

SIGNATURES: 1 found

  ✓ CFO Jane Smith (did:web:cfo.acme.com)
    Algorithm: Ed25519
    Timestamp: [timestamp]
    Status: VALID

RESULT: ✓ DOCUMENT VERIFIED
```

**SAY:**
> "Green checkmarks everywhere. Integrity valid, signature valid. This document is exactly what the CFO approved."

---

### Part 5: Tampering Detection (1 minute)

**SAY:**
> "Now here's the magic. Let me show you what happens if someone tries to tamper with the document."

**RUN:**
```bash
# Make a copy and tamper with it
cp q4-report.tdf tampered-report.tdf

# Unzip, modify, rezip (simulating attacker)
unzip -q tampered-report.tdf -d tampered/
# Change a single character in the content
sed -i 's/400000/900000/g' tampered/content.cbor 2>/dev/null || \
  sed -i '' 's/400000/900000/g' tampered/content.cbor
# Repack
cd tampered && zip -q ../tampered-report.tdf * && cd ..
rm -rf tampered/
```

**SAY:**
> "I just changed the net profit from €400K to €900K. An attacker might do this to inflate numbers. Let's verify the tampered document."

**RUN:**
```bash
tdf verify tampered-report.tdf --key cfo-key.verifying
```

**EXPECTED OUTPUT:**
```
TDF Verification Report
=======================
Document: tampered-report.tdf

INTEGRITY: ✗ INVALID
...
RESULT: ✗ VERIFICATION FAILED
```

**SAY:**
> "INTEGRITY INVALID. The system caught the tampering instantly. The hash doesn't match what the CFO signed. This is impossible to defeat without the CFO's private key."

---

### Part 6: Data Extraction (30 seconds)

**SAY:**
> "One more thing—because TDF is structured data, we can extract it directly for import into your systems."

**RUN:**
```bash
tdf extract q4-report.tdf --output extracted.json
cat extracted.json | head -30
```

**SAY:**
> "The actual numbers, ready for your ERP. No OCR, no manual re-entry, no errors."

---

### Closing (30 seconds)

**SAY:**
> "So in 5 minutes, you've seen:
> 1. Document creation with structured data
> 2. Cryptographic signing by the CFO
> 3. Instant verification by anyone
> 4. Tamper detection that catches any change
> 5. Direct data extraction for your systems
>
> This is what document trust looks like. Questions?"

---

## Cleanup

```bash
cd /tmp
rm -rf tdf-demo
```

---

## Common Questions & Answers

**Q: What if the CFO loses their key?**
A: Issue a revocation certificate. All documents signed before revocation remain valid; new signatures with that key are rejected.

**Q: Can this work with PDF?**
A: TDF can export to PDF for human viewing, but the TDF file is the source of truth.

**Q: How does this compare to DocuSign?**
A: DocuSign signs a picture of your document. TDF signs the actual data. And we're 85% cheaper at scale.

**Q: Is it hard to integrate?**
A: We have SDKs for Rust, JavaScript, and a REST API. Most integrations take 2-4 weeks.

---

*End of 5-Minute Demo Script*
