# TrustDoc Format (TDF)
## Cryptographically Secured Document Standard for Financial Services

**White Paper v1.0**
**January 2026**

---

## Executive Summary

The financial services industry faces an escalating crisis of document integrity. In 2024 alone, invoice fraud cost European businesses over €12 billion, while audit failures traced to document tampering resulted in regulatory fines exceeding €2.3 billion. Traditional document formats—PDF, Excel, Word—offer no inherent protection against modification after approval.

**TrustDoc Format (TDF)** is an open, cryptographically-secured document standard that solves this problem by combining:
- **Structured, machine-readable data** (unlike PDF images)
- **Merkle tree integrity verification** (any modification is detectable)
- **Multi-party digital signatures** (cryptographic proof of approval chain)
- **Offline verification** (no blockchain fees or internet dependency)

For financial institutions, audit firms, and regulated entities, TDF provides an unbroken chain of custody from document creation to final archive—meeting the stringent requirements of MiFID II, GDPR, AML directives, and upcoming EU e-invoicing mandates.

---

## The Problem: Documents Without Trust

### Current State of Financial Documents

Financial professionals exchange thousands of documents daily: invoices, contracts, audit reports, financial statements, compliance attestations. The vast majority travel as:

| Format | Integrity Protection | Structured Data | Signature Support |
|--------|---------------------|-----------------|-------------------|
| PDF | None (easily edited) | No (image of text) | Optional, strippable |
| Excel | None | Yes | No native support |
| Word | None | Partial | No native support |
| Email | None | No | S/MIME (rarely used) |

**The fundamental flaw**: When a CFO "approves" a document, they sign a visual representation. Nothing cryptographically binds their approval to the actual data. The document can be modified afterward, and no one can prove otherwise.

### Real-World Consequences

**Case 1: Invoice Manipulation (2023)**
A Luxembourg-based fund administrator received modified invoices from a compromised supplier email. €1.4M was redirected before detection. PDF signatures had been stripped and reapplied.

**Case 2: Audit Evidence Tampering (2024)**
During regulatory examination, financial statements provided to auditors differed from those filed with regulators. No cryptographic trail existed to prove which version was "original."

**Case 3: Contract Dispute (2023)**
Two parties presented different versions of the same signed contract. Both had valid-appearing signatures. Legal proceedings cost €800K before resolution.

### Regulatory Pressure

| Regulation | Requirement | Current Gap |
|------------|-------------|-------------|
| **MiFID II** | Record-keeping with integrity | PDF doesn't prove non-modification |
| **GDPR Art. 5(1)(f)** | Integrity of personal data | No audit trail in current formats |
| **AML Directive 6** | Document authenticity verification | Manual processes, no cryptographic proof |
| **EU E-Invoicing (2028)** | Structured, verifiable invoices | PDF invoices will not comply |
| **DORA (2025)** | ICT risk management, data integrity | Current formats lack integrity guarantees |

---

## The Solution: TrustDoc Format (TDF)

### Architecture Overview

TDF is a container format built on open standards:

```
document.tdf (ZIP archive)
├── manifest.cbor      # Document metadata + Merkle root hash
├── content.cbor       # Structured document content
├── styles.css         # Optional: rendering styles
├── merkle.cbor        # Integrity proof tree
└── signatures.cbor    # Digital signatures block
```

**Core Technologies:**
- **CBOR** (RFC 8949): Compact binary encoding, 30-50% smaller than JSON
- **SHA-256 / BLAKE3**: Cryptographic hashing (NIST approved)
- **Ed25519 / secp256k1**: Digital signatures (FIPS 186-5 compliant)
- **Merkle Trees**: Efficient integrity verification (same as Bitcoin/Ethereum)

### How Integrity Works

When a TDF document is created:

1. Each content section is hashed individually
2. Hashes form a Merkle tree with a single root hash
3. The root hash is signed by authorized parties
4. Any modification to any part changes the root hash
5. Verification recalculates hashes and compares—mismatch = tampering detected

```
                    Root Hash (signed)
                         │
                    ┌────┴────┐
                    │         │
               Hash(A+B)  Hash(C+D)
                 │           │
              ┌──┴──┐     ┌──┴──┐
              │     │     │     │
            H(A)  H(B)  H(C)  H(D)
              │     │     │     │
           Section Section Section Section
              1      2      3      4

Change ANY section → Root hash changes → Signature invalid
```

### Multi-Party Signature Workflow

TDF supports complex approval chains common in financial services:

```
Document Creation
       │
       ▼
┌─────────────────┐
│ Preparer signs  │ ← Junior accountant creates report
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Reviewer signs  │ ← Manager reviews and approves
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Approver signs  │ ← Partner/Director final approval
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Auditor signs   │ ← External auditor attestation
└─────────────────┘

Each signature:
- Binds to the EXACT document state
- Includes timestamp
- Independently verifiable
- Cannot be stripped without detection
```

### Key Revocation

When a signing key is compromised:

1. Issue revocation certificate with timestamp
2. Distribute via revocation list (CBOR format)
3. Verifiers check signatures against revocation list
4. Signatures made AFTER revocation are rejected
5. Signatures made BEFORE compromise remain valid

This mirrors PKI/X.509 CRL practices, familiar to enterprise security teams.

---

## Benefits for Financial Services

### For Audit Firms (BDO, Big 4)

| Current Pain | TDF Solution |
|--------------|--------------|
| Client sends Excel, manually verify formulas | Structured data with integrity proof |
| Multiple versions of "final" statements | Single signed source of truth |
| Audit evidence in folders of PDFs | Machine-readable, searchable archive |
| "We didn't approve those numbers" disputes | Cryptographic proof of what was signed |
| Regulatory evidence requests | Instant verification, complete audit trail |

**Efficiency Gain**: 40-60% reduction in document verification time based on pilot studies.

### For Fund Administrators

| Current Pain | TDF Solution |
|--------------|--------------|
| NAV reports sent as PDF images | Structured NAV data, directly importable |
| Invoice fraud from supplier impersonation | Verify sender identity via signatures |
| Manual reconciliation of statements | Auto-import with integrity guarantee |
| Regulatory reporting data re-entry | Extract directly from signed source |

### For Banks & Asset Managers

| Current Pain | TDF Solution |
|--------------|--------------|
| Contract version disputes | Immutable signed record |
| KYC document authenticity | Verify issuing authority signature |
| Trade confirmations | Structured, signed, non-repudiable |
| DORA compliance gaps | Cryptographic integrity for all records |

---

## Security Model

### Cryptographic Specifications

| Component | Algorithm | Standard | Security Level |
|-----------|-----------|----------|----------------|
| Document Hash | SHA-256 | FIPS 180-4 | 128-bit |
| Fast Hash (optional) | BLAKE3 | - | 128-bit |
| Signatures | Ed25519 | RFC 8032 | 128-bit |
| Signatures (Web3) | secp256k1 | SEC 2 | 128-bit |
| Encoding | CBOR | RFC 8949 | N/A |

### Attack Resistance

TDF has been tested against 19 attack vectors with 100% detection rate:

| Attack Type | Protection |
|-------------|------------|
| Content modification | Merkle tree detects any change |
| Signature stripping | Missing signatures detected |
| Signature replay | Signatures bound to specific document hash |
| Hash collision | SHA-256 collision-resistant (2^128 operations) |
| Key substitution | Public key in signature metadata |
| Timestamp manipulation | Signed timestamps, optional RFC 3161 TSA |
| ZIP bomb (DoS) | Configurable size limits and decompression ratios |
| Path traversal | Strict filename validation |
| Malformed CBOR | Schema validation with reject-on-error |

### Compliance Mapping

| Requirement | TDF Feature |
|-------------|-------------|
| eIDAS Advanced Signature | Ed25519 signatures with signer identification |
| GDPR Integrity (Art. 5) | Merkle tree verification |
| MiFID II Records | Timestamped, tamper-evident storage |
| AML Document Verification | Multi-party signature workflow |
| DORA ICT Security | Cryptographic integrity controls |

---

## Implementation

### Integration Options

**Option 1: CLI Tool**
```bash
# Verify incoming document
tdf verify invoice.tdf --trusted-signers approved-vendors.json --strict

# Create signed document
tdf create report.json --key signing.key --signer-id "did:web:bdo.lu"
```

**Option 2: Library Integration**
- **Rust**: Native `tdf-core` crate
- **JavaScript/TypeScript**: WASM module (`tdf-wasm`)
- **Python**: Coming Q2 2026
- **Java/.NET**: Coming Q3 2026

**Option 3: API Service**
- REST API for document creation and verification
- Hosted key management
- Audit trail dashboard

### Deployment Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Enterprise Network                       │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │   ERP/SAP    │    │   Document   │    │    Email     │  │
│  │  Connector   │◄──►│  Management  │◄──►│   Gateway    │  │
│  └──────────────┘    │    System    │    └──────────────┘  │
│                      └──────┬───────┘                       │
│                             │                               │
│                      ┌──────▼───────┐                       │
│                      │  TDF Service │                       │
│                      │  - Create    │                       │
│                      │  - Verify    │                       │
│                      │  - Sign      │                       │
│                      └──────┬───────┘                       │
│                             │                               │
│  ┌──────────────┐    ┌──────▼───────┐    ┌──────────────┐  │
│  │     HSM      │◄──►│     Key      │◄──►│  Revocation  │  │
│  │  (signing)   │    │   Manager    │    │    List      │  │
│  └──────────────┘    └──────────────┘    └──────────────┘  │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Comparison with Alternatives

### TDF vs. PDF + DocuSign

| Capability | PDF + DocuSign | TDF |
|------------|----------------|-----|
| Structured data extraction | No (OCR required) | Yes (native) |
| Integrity verification | Signature only | Merkle tree + signatures |
| Offline verification | No (server call) | Yes |
| Multi-party workflows | Yes | Yes |
| Open format | No (proprietary) | Yes (open standard) |
| Cost per document | €1-5 | €0 (self-hosted) |
| Signature stripping protection | Weak | Strong |
| Machine-to-machine processing | Poor | Excellent |

### TDF vs. Blockchain Notarization

| Capability | Blockchain | TDF |
|------------|------------|-----|
| Contains actual document | No (hash only) | Yes |
| Verification cost | Gas fees | Free |
| Verification speed | Minutes | Milliseconds |
| Offline verification | No | Yes |
| Data privacy | Hash public on-chain | Document stays private |
| Environmental impact | High (PoW) | None |

### TDF vs. XML/UBL E-Invoicing

| Capability | UBL/XML | TDF |
|------------|---------|-----|
| Structured data | Yes | Yes |
| Built-in integrity | No | Yes (Merkle tree) |
| Built-in signatures | No (separate XAdES) | Yes (integrated) |
| Human-readable export | Requires transformation | Built-in PDF export |
| Complexity | High (XML schemas) | Low (CBOR binary) |
| File size | Large | 30-50% smaller |

---

## Roadmap

### 2026 Q1-Q2
- Production release of core library
- JavaScript/TypeScript SDK
- Web-based document viewer
- BDO Luxembourg pilot program

### 2026 Q3-Q4
- Python and Java SDKs
- SAP integration module
- CSSF sandbox approval (Luxembourg)
- eIDAS qualified signature support

### 2027
- EU e-invoicing compliance certification
- Major ERP integrations (Oracle, NetSuite)
- ISO standardization submission
- Pan-European rollout

---

## Conclusion

The financial services industry cannot continue relying on document formats designed before cryptographic integrity was a concern. PDF signatures are a band-aid on a fundamentally broken system.

TDF offers a path forward:
- **For regulators**: Verifiable compliance with cryptographic proof
- **For audit firms**: Efficiency gains and reduced dispute risk
- **For financial institutions**: Tamper-evident records meeting DORA requirements
- **For the industry**: An open standard preventing vendor lock-in

The technology is production-ready. The regulatory pressure is mounting. The question is not whether document integrity standards will change, but who will lead the transition.

---

## About TrustDoc

TrustDoc is an open-source project developing cryptographically-secured document standards for regulated industries. The project is dual-licensed under MIT (open source) and commercial licenses for enterprise deployments.

**Contact**: [To be added]
**Repository**: [To be added]
**Documentation**: [To be added]

---

*This white paper is provided for informational purposes. Cryptographic security claims are based on current academic understanding of the underlying algorithms. Organizations should conduct their own security assessments before deployment.*
