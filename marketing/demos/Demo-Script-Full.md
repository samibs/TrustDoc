# TDF Full Demo Script
## Comprehensive 20-Minute Technical Demo

---

## Pre-Demo Checklist

```bash
# 1. Navigate to TrustDoc directory
cd /path/to/TrustDoc

# 2. Build release version
cargo build --release

# 3. Verify CLI works
./target/release/tdf --version

# 4. Create clean demo directory
rm -rf /tmp/tdf-demo && mkdir -p /tmp/tdf-demo
cd /tmp/tdf-demo

# 5. Copy example files
cp /path/to/TrustDoc/examples/invoice.json .
cp /path/to/TrustDoc/examples/trusted-signers.json .
```

---

## PART 1: KEY MANAGEMENT (3 minutes)

### 1.1 Generate Ed25519 Keys

**SAY:**
> "Let's start with key generation. TDF supports two algorithms: Ed25519 for general use, and secp256k1 for Web3 compatibility. I'll generate keys for two signers."

**RUN:**
```bash
# Generate CFO keys
tdf keygen --output . --name cfo-key
```

**OUTPUT:**
```
Signing key (private) written to: ./cfo-key.signing
  ⚠️  Keep this file secure and never share it!
Verifying key (public) written to: ./cfo-key.verifying
  ✓  This file can be shared publicly
```

**RUN:**
```bash
# Generate CEO keys
tdf keygen --output . --name ceo-key
```

**SAY:**
> "Each signer has a keypair. In production, private keys would be in an HSM or hardware wallet. Public keys can be freely distributed."

### 1.2 Generate secp256k1 Keys (Optional)

**SAY:**
> "For blockchain integration, we also support secp256k1—the same algorithm used by Ethereum."

**RUN:**
```bash
tdf keygen --output . --name web3-key --secp256k1
```

---

## PART 2: DOCUMENT CREATION (4 minutes)

### 2.1 View Input JSON

**SAY:**
> "TDF takes structured data as input. Let me show you a sample invoice."

**RUN:**
```bash
cat invoice.json | head -50
```

**SAY:**
> "Notice this is actual data—not a PDF image. Tables with typed columns, currency values with raw numbers and display formatting."

### 2.2 Create Unsigned TDF

**SAY:**
> "Let's create a TDF without signing first."

**RUN:**
```bash
tdf create invoice.json --output unsigned-invoice.tdf
```

### 2.3 Create Signed TDF

**SAY:**
> "Now let's create a properly signed document. The CFO approves this invoice."

**RUN:**
```bash
tdf create invoice.json \
  --output signed-invoice.tdf \
  --key cfo-key.signing \
  --signer-id "did:web:cfo.acme.com" \
  --signer-name "CFO Jane Smith" \
  --timestamp-manual
```

**SAY:**
> "The document is created with:
> - Structured content locked in the file
> - Merkle tree computed over all sections
> - CFO's digital signature on the root hash
> - Timestamp of when it was signed"

### 2.4 View Document Info

**RUN:**
```bash
tdf info signed-invoice.tdf
```

**SAY:**
> "Here's the document metadata. Notice the root hash—that's the fingerprint of the entire document. Any change would produce a different hash."

---

## PART 3: VERIFICATION (5 minutes)

### 3.1 Basic Verification

**SAY:**
> "Anyone with the public key can verify this document."

**RUN:**
```bash
tdf verify signed-invoice.tdf --key cfo-key.verifying
```

**SAY:**
> "Green checkmarks: integrity valid, signature valid. This is exactly what the CFO signed."

### 3.2 Verification with Trusted Signers Whitelist

**SAY:**
> "In an enterprise, you want to ensure documents are signed by authorized people. Let's use a whitelist."

**RUN:**
```bash
cat trusted-signers.json
```

**SAY:**
> "This whitelist defines who's authorized: CFO, CEO, Controller, and an external auditor. Each has specific roles."

**RUN:**
```bash
tdf verify signed-invoice.tdf \
  --key cfo-key.verifying \
  --trusted-signers trusted-signers.json
```

**SAY:**
> "Now it shows: Trusted ✓ (in whitelist, roles: financial-approver, auditor). The system confirms this signer is authorized."

### 3.3 Untrusted Signer Detection

**SAY:**
> "What happens when someone outside the whitelist signs?"

**RUN:**
```bash
# Create document signed by unauthorized person
tdf create invoice.json \
  --output rogue-invoice.tdf \
  --key ceo-key.signing \
  --signer-id "did:web:attacker.malicious.com" \
  --signer-name "Unknown Person"

# Verify against whitelist
tdf verify rogue-invoice.tdf \
  --key ceo-key.verifying \
  --trusted-signers trusted-signers.json
```

**SAY:**
> "Warning: Signer NOT IN WHITELIST. The signature is cryptographically valid, but it's not from an authorized person."

### 3.4 Strict Mode

**SAY:**
> "Strict mode fails verification if there are ANY warnings."

**RUN:**
```bash
tdf verify rogue-invoice.tdf \
  --key ceo-key.verifying \
  --trusted-signers trusted-signers.json \
  --strict

echo "Exit code: $?"
```

**SAY:**
> "Exit code 1—verification failed. In your CI/CD pipeline, this would block the process."

---

## PART 4: TAMPER DETECTION (4 minutes)

### 4.1 Setup Tampered Document

**SAY:**
> "Let me demonstrate tamper detection. I'll modify a signed document."

**RUN:**
```bash
# Copy the signed document
cp signed-invoice.tdf tampered-invoice.tdf

# Unzip and modify
mkdir tampered
cd tampered
unzip -q ../tampered-invoice.tdf

# Show the files
ls -la
```

**SAY:**
> "A TDF is just a ZIP file with CBOR-encoded data. An attacker could try to modify it."

### 4.2 Modify Content

**RUN:**
```bash
# Show original content hash
shasum -a 256 content.cbor

# Modify the content (change a price)
# Using a hex editor simulation - append a byte
echo "x" >> content.cbor

# Show modified hash
shasum -a 256 content.cbor

# Repack
zip -q ../tampered-invoice.tdf *
cd ..
rm -rf tampered
```

**SAY:**
> "I've modified the content file. Even adding one byte changes the hash completely."

### 4.3 Verify Tampered Document

**RUN:**
```bash
tdf verify tampered-invoice.tdf --key cfo-key.verifying
```

**SAY:**
> "INTEGRITY: INVALID. The Merkle tree hash doesn't match. The system detected tampering instantly. This is mathematically impossible to defeat."

---

## PART 5: KEY REVOCATION (3 minutes)

### 5.1 Revoke a Compromised Key

**SAY:**
> "What if the CFO's laptop is stolen? We need to revoke their key."

**RUN:**
```bash
tdf revoke \
  --key-id "did:web:cfo.acme.com" \
  --reason "key-compromise" \
  --authority "Security Team" \
  --output revoked-keys.cbor
```

**SAY:**
> "We've created a revocation list. This would be distributed to all verification points."

### 5.2 Verify Against Revocation List

**RUN:**
```bash
tdf verify signed-invoice.tdf \
  --key cfo-key.verifying \
  --revocation-list revoked-keys.cbor
```

**SAY:**
> "Now it shows: Revoked ✓ (reason: KeyCompromise). The document is still technically valid—it was signed before the compromise—but the system alerts you to the revocation."

---

## PART 6: SECURITY TIERS (2 minutes)

### 6.1 Demonstrate Size Limits

**SAY:**
> "TDF has configurable security tiers to prevent ZIP bomb attacks."

**RUN:**
```bash
# Show available tiers
echo "Security Tiers:"
echo "  micro:      256 KB max, 100:1 decompression ratio"
echo "  standard:   5 MB max, 1000:1 ratio (default)"
echo "  extended:   50 MB max, 10000:1 ratio"
echo "  permissive: 100 MB max (testing only)"
```

**RUN:**
```bash
# Verify with different tiers
tdf verify signed-invoice.tdf --security-tier micro
tdf verify signed-invoice.tdf --security-tier standard
tdf verify signed-invoice.tdf --security-tier extended
```

**SAY:**
> "Choose the tier appropriate for your documents. Micro for invoices, Standard for reports, Extended for large datasets."

---

## PART 7: DATA EXTRACTION (2 minutes)

### 7.1 Extract to JSON

**SAY:**
> "The killer feature: extracting structured data for your systems."

**RUN:**
```bash
tdf extract signed-invoice.tdf --output extracted-data.json
cat extracted-data.json | head -40
```

**SAY:**
> "Actual data: metadata, tables with rows and columns. This goes directly into your ERP—no OCR, no manual entry, no errors."

### 7.2 Export to PDF

**SAY:**
> "For human viewing, we can export to PDF."

**RUN:**
```bash
tdf export signed-invoice.tdf --output invoice.pdf
ls -la invoice.pdf
```

**SAY:**
> "The PDF is for viewing. The TDF is the source of truth."

---

## PART 8: RECAP & Q&A (2 minutes)

**SAY:**
> "Let me summarize what we've seen:
>
> 1. **Key Management**: Ed25519 and secp256k1 keypairs
> 2. **Document Creation**: Structured data with cryptographic signatures
> 3. **Verification**: Integrity + authenticity + whitelist validation
> 4. **Tamper Detection**: Any modification instantly caught
> 5. **Key Revocation**: Handle compromised keys
> 6. **Security Tiers**: Protection against DoS attacks
> 7. **Data Extraction**: Direct import to your systems
>
> This is enterprise-grade document security. Questions?"

---

## CLEANUP

```bash
cd /tmp
rm -rf tdf-demo
```

---

## APPENDIX: Common Demo Scenarios

### Scenario A: Audit Firm Demo

Focus on:
- Whitelist validation (authorized signers)
- Tamper detection
- Data extraction for audit software

### Scenario B: Fund Admin Demo

Focus on:
- Multi-party signatures (preparer → reviewer → approver)
- Direct NAV data extraction
- Security tiers for large reports

### Scenario C: Bank/Compliance Demo

Focus on:
- Key revocation workflow
- Strict mode for automated processing
- Regulatory alignment (DORA, MiFID II)

### Scenario D: IT/Security Demo

Focus on:
- Cryptographic details (SHA-256, Ed25519)
- Merkle tree structure
- Attack resistance (show multiple attack types)

---

## TROUBLESHOOTING

### "Command not found"
```bash
# Use full path
./target/release/tdf --help
# Or add to PATH
export PATH=$PATH:/path/to/TrustDoc/target/release
```

### "Invalid key format"
```bash
# Regenerate keys
tdf keygen --output . --name new-key
```

### "Permission denied"
```bash
chmod +x ./target/release/tdf
```

---

*End of Full Demo Script*
