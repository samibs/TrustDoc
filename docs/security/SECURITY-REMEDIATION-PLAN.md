# TDF Security Remediation Plan
## Implementation Roadmap for Critical Fixes

**Document Version**: 1.0
**Created**: 2026-01-10
**Target Completion**: Phase 1 in 2 weeks, Phase 2 in 4 weeks, Phase 3 in 8 weeks

---

## Overview

This plan addresses the 27 vulnerabilities identified in the security audit, organized into 3 phases based on severity and dependency order.

### Phase Summary

| Phase | Focus | Duration | CVEs Addressed |
|-------|-------|----------|----------------|
| **Phase 1** | Critical Cryptographic Fixes | 2 weeks | 9 CRITICAL |
| **Phase 2** | Protocol Hardening | 2 weeks | 10 HIGH |
| **Phase 3** | Defense in Depth | 4 weeks | 8 MEDIUM/LOW |

---

## Phase 1: Critical Cryptographic Fixes (Week 1-2)

### 1.1 Mandatory Signature Verification
**CVE**: CVE-TDF-001, CVE-TDF-018
**Priority**: P0 - BLOCKING
**Effort**: 2 days

#### Current State
```rust
// verify.rs - signatures are optional
let verifying_key = if let Some(key_path) = &key {
    Some(utils::load_verifying_key(key_path)?)
} else {
    None  // Verification skipped entirely!
};
```

#### Target State
```rust
// verify.rs - signatures mandatory unless explicitly unsigned
pub fn verify_document(
    document: PathBuf,
    key: Option<PathBuf>,
    allow_unsigned: bool,  // NEW: explicit opt-in
    // ...
) -> TdfResult<()> {
    // Load document first to check if it has signatures
    let reader = ArchiveReader::read(&document)?;
    let has_signatures = reader.signature_block()
        .map(|b| !b.signatures.is_empty())
        .unwrap_or(false);

    if has_signatures && key.is_none() {
        return Err(TdfError::VerificationFailed(
            "Document has signatures but no verification key provided. \
             Use --key to verify or --allow-unsigned to skip.".to_string()
        ));
    }

    if !has_signatures && !allow_unsigned {
        return Err(TdfError::VerificationFailed(
            "Document has no signatures. Use --allow-unsigned to verify \
             integrity only.".to_string()
        ));
    }
    // ...
}
```

#### Files to Modify
- `tdf-cli/src/main.rs` - Add `--allow-unsigned` flag
- `tdf-cli/src/commands/verify.rs` - Implement mandatory verification
- `tdf-core/src/error.rs` - Add `VerificationFailed` variant

#### Tests
```rust
#[test]
fn test_signed_doc_requires_key() {
    let result = verify_document(signed_doc, None, false);
    assert!(matches!(result, Err(TdfError::VerificationFailed(_))));
}

#[test]
fn test_unsigned_doc_requires_flag() {
    let result = verify_document(unsigned_doc, None, false);
    assert!(matches!(result, Err(TdfError::VerificationFailed(_))));
}
```

---

### 1.2 Bind Timestamp to Signature
**CVE**: CVE-TDF-003, CVE-TDF-006, CVE-TDF-013
**Priority**: P0 - BLOCKING
**Effort**: 3 days

#### Current State
```rust
// Timestamp stored separately, not part of signed data
pub struct DocumentSignature {
    pub timestamp: TimestampInfo,  // Can be modified after signing!
    pub signature: String,
    pub root_hash: String,
}
```

#### Target State
```rust
// New: Signature covers timestamp
pub struct DocumentSignature {
    pub signer: SignerInfo,
    pub algorithm: SignatureAlgorithm,
    pub timestamp: TimestampInfo,
    pub root_hash: String,
    pub signature: String,  // Now signs: hash(root_hash || timestamp || signer_id)
    pub signed_payload: String,  // NEW: The exact bytes that were signed (for verification)
}

impl DocumentSignature {
    /// Create the canonical payload that gets signed
    pub fn create_signing_payload(
        root_hash: &[u8],
        timestamp: &TimestampInfo,
        signer_id: &str,
    ) -> Vec<u8> {
        let mut payload = Vec::new();
        // Domain separator for signature payload
        payload.extend_from_slice(b"TDF-SIG-V1:");
        payload.extend_from_slice(root_hash);
        payload.extend_from_slice(b":");
        // ISO 8601 timestamp - canonical format
        payload.extend_from_slice(timestamp.time.to_rfc3339().as_bytes());
        payload.extend_from_slice(b":");
        payload.extend_from_slice(signer_id.as_bytes());
        payload
    }
}
```

#### Files to Modify
- `tdf-core/src/signature.rs` - Update signing payload construction
- `tdf-core/src/archive.rs` - Update build() to use new signing format
- `tdf-core/src/multiparty.rs` - Update multiparty signing

#### Migration Strategy
```rust
// Version detection for backwards compatibility
pub fn verify_signature_payload(sig: &DocumentSignature) -> TdfResult<Vec<u8>> {
    if sig.signed_payload.is_empty() {
        // Legacy: signature only covers root_hash
        warn!("Legacy signature format detected - timestamp not bound");
        Ok(hex::decode(&sig.root_hash)?)
    } else {
        // V2: signature covers full payload
        Ok(base64::decode(&sig.signed_payload)?)
    }
}
```

---

### 1.3 Validate Signature Against Computed Root Hash
**CVE**: CVE-TDF-007
**Priority**: P0 - BLOCKING
**Effort**: 1 day

#### Current State
```rust
// archive.rs - signatures verified but not compared to computed hash
let integrity_valid = merkle_tree.verify(&components)?;
let root_hash = merkle_tree.root_hash().to_vec();

// MISSING: Check that each signature's root_hash matches computed root!
```

#### Target State
```rust
// archive.rs - explicit root hash binding
let computed_root = merkle_tree.root_hash();
let computed_root_hex = hex::encode(&computed_root);

for sig in &signature_block.signatures {
    if sig.root_hash != computed_root_hex {
        return Err(TdfError::IntegrityError(format!(
            "Signature root hash mismatch: expected {}, got {}",
            computed_root_hex, sig.root_hash
        )));
    }
}
```

#### Files to Modify
- `tdf-core/src/archive.rs` - Add root hash comparison in verify()

---

### 1.4 Add Merkle Tree Domain Separators
**CVE**: CVE-TDF-002
**Priority**: P0 - BLOCKING
**Effort**: 2 days

#### Current State
```rust
// merkle.rs - no domain separation
let combined = chunk[0].clone();
combined.extend_from_slice(&chunk[1]);
let hash = sha256(&combined);
```

#### Target State
```rust
// merkle.rs - domain-separated hashing
const LEAF_PREFIX: u8 = 0x00;
const INTERNAL_PREFIX: u8 = 0x01;

fn hash_leaf(&self, data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(&[LEAF_PREFIX]);  // Domain separator
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn hash_internal(&self, left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(&[INTERNAL_PREFIX]);  // Domain separator
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().to_vec()
}
```

#### Files to Modify
- `tdf-core/src/merkle.rs` - Add prefixed hashing functions
- Update `compute_root()` and `build_tree()` to use new functions

#### Breaking Change
This changes the Merkle root computation. Requires version bump and migration:
```rust
pub enum MerkleVersion {
    V1,  // Legacy: no domain separators
    V2,  // Current: domain separators
}
```

---

### 1.5 Bounded File Reading
**CVE**: CVE-TDF-005, CVE-TDF-009
**Priority**: P0 - BLOCKING
**Effort**: 2 days

#### Current State
```rust
// Unbounded read - can allocate gigabytes
let mut bytes = Vec::new();
file.read_to_end(&mut bytes)?;
```

#### Target State
```rust
// New: bounded reader utility
pub struct BoundedReader<R: Read> {
    inner: R,
    limit: u64,
    read: u64,
}

impl<R: Read> BoundedReader<R> {
    pub fn new(reader: R, limit: u64) -> Self {
        Self { inner: reader, limit, read: 0 }
    }
}

impl<R: Read> Read for BoundedReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.read >= self.limit {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Read limit exceeded: {} bytes", self.limit)
            ));
        }
        let remaining = (self.limit - self.read) as usize;
        let to_read = buf.len().min(remaining);
        let n = self.inner.read(&mut buf[..to_read])?;
        self.read += n as u64;
        Ok(n)
    }
}

// Usage in archive.rs
fn read_file_bounded(file: &mut impl Read, limit: u64) -> TdfResult<Vec<u8>> {
    let mut reader = BoundedReader::new(file, limit);
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    Ok(bytes)
}
```

#### Files to Modify
- `tdf-core/src/io.rs` (NEW) - BoundedReader implementation
- `tdf-core/src/archive.rs` - Replace all `read_to_end()` calls
- `tdf-core/src/lib.rs` - Export io module

---

### 1.6 CBOR Depth Limits
**CVE**: CVE-TDF-009
**Priority**: P0 - BLOCKING
**Effort**: 1 day

#### Current State
```rust
// No depth limit - deeply nested CBOR causes stack overflow
let manifest: Manifest = serde_cbor::from_slice(&bytes)?;
```

#### Target State
```rust
// Option 1: Switch to ciborium with depth limits
use ciborium::de::from_reader_with_recursion_limit;

const MAX_CBOR_DEPTH: usize = 64;

fn deserialize_cbor<T: DeserializeOwned>(bytes: &[u8]) -> TdfResult<T> {
    let cursor = std::io::Cursor::new(bytes);
    from_reader_with_recursion_limit(cursor, MAX_CBOR_DEPTH)
        .map_err(|e| TdfError::ParseError(format!("CBOR error: {}", e)))
}

// Option 2: Keep serde_cbor, add manual size check
fn deserialize_cbor_safe<T: DeserializeOwned>(bytes: &[u8], max_size: usize) -> TdfResult<T> {
    if bytes.len() > max_size {
        return Err(TdfError::SizeExceeded(format!(
            "CBOR data too large: {} > {}", bytes.len(), max_size
        )));
    }
    serde_cbor::from_slice(bytes)
        .map_err(|e| TdfError::ParseError(format!("CBOR error: {}", e)))
}
```

#### Files to Modify
- `tdf-core/Cargo.toml` - Add ciborium or keep serde_cbor
- `tdf-core/src/archive.rs` - Use safe deserialization
- All CBOR parsing locations

---

### 1.7 Fix Integer Overflow in Merkle Deserialization
**CVE**: CVE-TDF-008
**Priority**: P0 - BLOCKING
**Effort**: 0.5 days

#### Current State
```rust
let count = u32::from_be_bytes(...) as usize;
if data.len() < 10 + 32 + (count * 32) {  // Overflow!
```

#### Target State
```rust
let count = u32::from_be_bytes(...) as usize;
let required_size = 10usize
    .checked_add(32)
    .and_then(|s| s.checked_add(count.checked_mul(32)?))
    .ok_or_else(|| TdfError::InvalidDocument("Merkle tree size overflow".to_string()))?;

if data.len() < required_size {
    return Err(TdfError::InvalidDocument("Merkle tree truncated".to_string()));
}
```

#### Files to Modify
- `tdf-core/src/merkle.rs` - Fix `from_binary()` function

---

### 1.8 Fix Decompression Ratio Zero Divisor
**CVE**: CVE-TDF-015
**Priority**: P1 - HIGH
**Effort**: 0.5 days

#### Current State
```rust
if compressed == 0 {
    return Ok(());  // Bypass!
}
```

#### Target State
```rust
pub fn check_decompression_ratio(&self, compressed: u64, uncompressed: u64) -> TdfResult<()> {
    // Handle edge cases
    if uncompressed == 0 {
        return Ok(());  // Empty file is always OK
    }

    if compressed == 0 {
        // Stored (uncompressed) file - check absolute size only
        return self.check_file_size(uncompressed);
    }

    let ratio = uncompressed.checked_div(compressed)
        .unwrap_or(u64::MAX);

    if ratio > self.max_decompression_ratio {
        return Err(TdfError::SizeExceeded(...));
    }
    Ok(())
}
```

#### Files to Modify
- `tdf-core/src/config.rs` - Fix ratio check logic

---

## Phase 2: Protocol Hardening (Week 3-4)

### 2.1 Implement RFC 3161 Proof Validation
**CVE**: CVE-TDF-004, CVE-TDF-017
**Priority**: P1 - HIGH
**Effort**: 5 days

#### Implementation Options

**Option A: Full ASN.1 Implementation**
```toml
# Cargo.toml
[dependencies]
rasn = "0.12"  # ASN.1 library
x509-cert = "0.2"  # Certificate handling
```

```rust
// timestamp.rs - Full RFC 3161 validation
pub fn validate_rfc3161_proof(
    token: &TimestampToken,
    data_hash: &[u8],
    trusted_tsas: &[TrustedTSA],
) -> TdfResult<TimestampValidationResult> {
    let proof_bytes = base64::decode(&token.proof)
        .map_err(|_| TdfError::InvalidProof("Invalid base64"))?;

    // Parse ASN.1 TimeStampResp
    let tsr: TimeStampResp = rasn::der::decode(&proof_bytes)
        .map_err(|_| TdfError::InvalidProof("Invalid ASN.1"))?;

    // Verify TSA signature
    let tst_info = &tsr.time_stamp_token.content;
    let tsa_cert = extract_tsa_certificate(&tsr)?;

    if !verify_tsa_signature(&tsr, &tsa_cert)? {
        return Err(TdfError::InvalidProof("TSA signature invalid"));
    }

    // Verify hash matches
    if tst_info.message_imprint.hashed_message != data_hash {
        return Err(TdfError::InvalidProof("Hash mismatch"));
    }

    // Verify TSA is trusted
    if !is_tsa_trusted(&tsa_cert, trusted_tsas)? {
        return Err(TdfError::InvalidProof("TSA not trusted"));
    }

    Ok(TimestampValidationResult {
        valid: true,
        authority: tsa_cert.subject.to_string(),
        time: tst_info.gen_time,
    })
}
```

**Option B: Defer to External Service**
```rust
// Simpler: mark RFC 3161 as requiring external validation
pub fn validate_rfc3161_proof(token: &TimestampToken) -> TdfResult<TimestampValidationResult> {
    if token.proof.is_empty() {
        return Err(TdfError::InvalidProof("RFC 3161 proof required"));
    }

    // Return "needs external validation" status
    Ok(TimestampValidationResult {
        valid: false,
        requires_external_validation: true,
        proof_data: token.proof.clone(),
        message: "RFC 3161 proof requires external TSA verification".to_string(),
    })
}
```

#### Recommendation
Implement Option B first (blocks invalid claims), then Option A as Phase 3 enhancement.

---

### 2.2 Enforce Algorithm Whitelist
**CVE**: CVE-TDF-010
**Priority**: P1 - HIGH
**Effort**: 1 day

```rust
// config.rs - Algorithm policy
#[derive(Debug, Clone)]
pub struct AlgorithmPolicy {
    pub allowed_signature_algorithms: HashSet<SignatureAlgorithm>,
    pub allowed_hash_algorithms: HashSet<HashAlgorithm>,
    pub minimum_key_size: usize,
}

impl Default for AlgorithmPolicy {
    fn default() -> Self {
        Self {
            allowed_signature_algorithms: hashset![
                SignatureAlgorithm::Ed25519,
                SignatureAlgorithm::Secp256k1,
            ],
            allowed_hash_algorithms: hashset![
                HashAlgorithm::Sha256,
                HashAlgorithm::Blake3,
            ],
            minimum_key_size: 256,
        }
    }
}

// verify.rs - Enforce policy
fn check_algorithm_policy(sig: &DocumentSignature, policy: &AlgorithmPolicy) -> TdfResult<()> {
    if !policy.allowed_signature_algorithms.contains(&sig.algorithm) {
        return Err(TdfError::PolicyViolation(format!(
            "Algorithm {:?} not allowed by policy", sig.algorithm
        )));
    }
    Ok(())
}
```

---

### 2.3 Mandatory Revocation Checking
**CVE**: CVE-TDF-011
**Priority**: P1 - HIGH
**Effort**: 2 days

```rust
// verify.rs - Revocation modes
pub enum RevocationMode {
    /// Skip revocation checking (explicit opt-out)
    Skip,
    /// Use embedded revocation list only
    Embedded,
    /// Use provided revocation list
    External(PathBuf),
    /// Fetch from authority (future)
    Online(String),
}

pub fn verify_document(
    document: PathBuf,
    key: Option<PathBuf>,
    revocation_mode: RevocationMode,
    // ...
) -> TdfResult<()> {
    match revocation_mode {
        RevocationMode::Skip => {
            eprintln!("WARNING: Revocation checking disabled");
        }
        RevocationMode::Embedded => {
            let revocation = reader.revocation_list()
                .ok_or(TdfError::RevocationRequired(
                    "No embedded revocation list and --revocation-skip not set"
                ))?;
            check_revocation(&signatures, &revocation)?;
        }
        RevocationMode::External(path) => {
            let revocation = load_revocation_list(&path)?;
            check_revocation(&signatures, &revocation)?;
        }
        RevocationMode::Online(url) => {
            // Future: fetch from CRL/OCSP endpoint
            unimplemented!("Online revocation checking not yet implemented");
        }
    }
}
```

---

### 2.4 Enforce Whitelist (Not Advisory)
**CVE**: CVE-TDF-012
**Priority**: P1 - HIGH
**Effort**: 1 day

```rust
// verify.rs - Whitelist enforcement modes
pub enum WhitelistMode {
    /// No whitelist checking
    None,
    /// Warn on untrusted signers (current behavior)
    Advisory(PathBuf),
    /// Fail on untrusted signers
    Enforce(PathBuf),
}

fn check_whitelist(
    signatures: &[DocumentSignature],
    mode: &WhitelistMode,
) -> TdfResult<Vec<WhitelistResult>> {
    match mode {
        WhitelistMode::None => Ok(vec![]),
        WhitelistMode::Advisory(path) => {
            let whitelist = load_whitelist(path)?;
            let results = check_signers(&signatures, &whitelist);
            for r in &results {
                if !r.trusted {
                    eprintln!("WARNING: Signer {} not in whitelist", r.signer_id);
                }
            }
            Ok(results)
        }
        WhitelistMode::Enforce(path) => {
            let whitelist = load_whitelist(path)?;
            let results = check_signers(&signatures, &whitelist);
            for r in &results {
                if !r.trusted {
                    return Err(TdfError::UntrustedSigner(format!(
                        "Signer {} not in whitelist", r.signer_id
                    )));
                }
            }
            Ok(results)
        }
    }
}
```

---

### 2.5 Validate Whitelist Public Keys
**CVE**: CVE-TDF-024
**Priority**: P1 - HIGH
**Effort**: 2 days

```rust
// whitelist.rs - Key binding validation
impl SignerWhitelist {
    pub fn validate_signer(
        &self,
        signer_id: &str,
        public_key: &VerifyingKey,
    ) -> WhitelistValidationResult {
        let signer = match self.get_signer(signer_id) {
            Some(s) => s,
            None => return WhitelistValidationResult::NotFound,
        };

        // If whitelist has public key, verify it matches
        if let Some(expected_key) = &signer.public_key {
            let actual_key = hex::encode(public_key.as_bytes());
            if actual_key != *expected_key {
                return WhitelistValidationResult::KeyMismatch {
                    expected: expected_key.clone(),
                    actual: actual_key,
                };
            }
        }

        WhitelistValidationResult::Trusted {
            signer: signer.clone(),
        }
    }
}
```

---

### 2.6 Verify Multiparty Signatures
**CVE**: CVE-TDF-016
**Priority**: P1 - HIGH
**Effort**: 2 days

```rust
// multiparty.rs - Full signature verification on add
pub fn add_signature(
    &mut self,
    signature: DocumentSignature,
    verifying_key: &VerifyingKey,  // NEW: require key for verification
) -> TdfResult<()> {
    // Verify the signature is cryptographically valid
    let payload = DocumentSignature::create_signing_payload(
        &hex::decode(&signature.root_hash)?,
        &signature.timestamp,
        &signature.signer.id,
    );

    verify_ed25519(&payload, &signature.signature, verifying_key)?;

    // Verify root hash matches our document
    if signature.root_hash != self.root_hash {
        return Err(TdfError::InvalidSignature(
            "Signature root hash doesn't match document".to_string()
        ));
    }

    // Verify signer is in required list
    if !self.workflow.required_signers.contains(&signature.signer.id) {
        return Err(TdfError::UnauthorizedSigner(signature.signer.id.clone()));
    }

    self.signatures.push(signature);
    Ok(())
}
```

---

### 2.7 Strict Mode as Default
**CVE**: CVE-TDF-018
**Priority**: P1 - HIGH
**Effort**: 1 day

```rust
// main.rs - Flip default
Verify {
    document: PathBuf,
    #[arg(short, long)]
    key: Option<PathBuf>,

    /// Lenient mode: allow warnings without failing
    #[arg(long)]
    lenient: bool,  // CHANGED: was --strict, now --lenient

    // ...
}

// verify.rs - Default to strict
let strict = !lenient;  // Strict by default
```

---

### 2.8 Validate Signature Scope
**CVE**: CVE-TDF-019
**Priority**: P2 - MEDIUM
**Effort**: 1 day

```rust
// signature.rs - Scope validation
impl SignatureScope {
    pub fn validate(&self, document: &DocumentContent) -> TdfResult<()> {
        match self {
            SignatureScope::Full | SignatureScope::ContentOnly => Ok(()),
            SignatureScope::Sections(section_ids) => {
                let doc_sections: HashSet<_> = document.sections
                    .iter()
                    .map(|s| &s.id)
                    .collect();

                for id in section_ids {
                    if !doc_sections.contains(id) {
                        return Err(TdfError::InvalidScope(format!(
                            "Signature scope references non-existent section: {}", id
                        )));
                    }
                }
                Ok(())
            }
        }
    }
}
```

---

### 2.9 Enforce File Count Limit
**CVE**: CVE-TDF-020
**Priority**: P2 - MEDIUM
**Effort**: 0.5 days

```rust
// archive.rs - Add file count check
fn verify_archive_limits(zip: &ZipArchive<impl Read>, config: &SecurityConfig) -> TdfResult<()> {
    if zip.len() > config.max_file_count {
        return Err(TdfError::SizeExceeded(format!(
            "Archive contains {} files, limit is {}",
            zip.len(), config.max_file_count
        )));
    }
    // ... other checks
}
```

---

### 2.10 Fix add_asset() to Return Result
**CVE**: CVE-TDF-021
**Priority**: P2 - MEDIUM
**Effort**: 0.5 days

```rust
// archive.rs - Return error instead of logging
pub fn add_asset(&mut self, path: String, data: Vec<u8>) -> TdfResult<()> {
    // Validate path
    if path.contains("..") || path.starts_with('/') {
        return Err(TdfError::InvalidPath(format!(
            "Asset path contains invalid characters: {}", path
        )));
    }

    // Check size
    self.security_config.check_file_size(data.len() as u64)?;

    self.assets.insert(path, data);
    Ok(())
}
```

---

## Phase 3: Defense in Depth (Week 5-8)

### 3.1 Full RFC 3161 ASN.1 Implementation
**Effort**: 5 days

Implement complete ASN.1 parsing and TSA verification as described in Option A of 2.1.

---

### 3.2 Cryptographic Revocation Authority
**CVE**: CVE-TDF-025
**Effort**: 3 days

```rust
// revocation.rs - Signed revocation entries
pub struct SignedRevocationList {
    pub entries: Vec<RevocationEntry>,
    pub authority: AuthorityInfo,
    pub signature: String,  // Authority's signature over entries
    pub issued_at: DateTime<Utc>,
    pub next_update: DateTime<Utc>,
}

impl SignedRevocationList {
    pub fn verify(&self, authority_key: &VerifyingKey) -> TdfResult<bool> {
        let payload = self.canonical_payload();
        verify_ed25519(&payload, &self.signature, authority_key)
    }
}
```

---

### 3.3 HSM Integration
**Effort**: 5 days

```rust
// crypto/hsm.rs - HSM abstraction
pub trait HsmProvider: Send + Sync {
    fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, HsmError>;
    fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> Result<bool, HsmError>;
    fn get_public_key(&self, key_id: &str) -> Result<Vec<u8>, HsmError>;
}

// Implementations for:
// - PKCS#11 (YubiKey, AWS CloudHSM)
// - AWS KMS
// - Azure Key Vault
// - Google Cloud KMS
```

---

### 3.4 Audit Logging
**Effort**: 2 days

```rust
// audit.rs - Structured audit log
pub struct AuditLogger {
    output: Box<dyn Write + Send>,
}

impl AuditLogger {
    pub fn log_verification(&self, event: VerificationEvent) {
        let entry = AuditEntry {
            timestamp: Utc::now(),
            event_type: "VERIFICATION",
            document_hash: event.document_hash,
            result: event.result,
            signers: event.signers,
            warnings: event.warnings,
        };
        writeln!(self.output, "{}", serde_json::to_string(&entry).unwrap());
    }
}
```

---

### 3.5 Replace serde_cbor with ciborium
**Effort**: 1 day

```toml
# Cargo.toml
[dependencies]
# serde_cbor = "0.11"  # REMOVE - unmaintained
ciborium = "0.2"  # ADD - actively maintained
```

---

### 3.6 Constant-Time Comparisons
**Effort**: 1 day

```rust
// crypto/constant_time.rs
use subtle::ConstantTimeEq;

pub fn secure_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}

// Replace all hash/signature comparisons
```

---

### 3.7 Timestamp Ordering Enforcement
**CVE**: CVE-TDF-023
**Effort**: 1 day

```rust
// multiparty.rs - Enforce chronological order
pub fn validate_signature_order(&self) -> TdfResult<()> {
    let mut prev_time: Option<DateTime<Utc>> = None;

    for sig in &self.signatures {
        if let Some(prev) = prev_time {
            if sig.timestamp.time < prev {
                return Err(TdfError::InvalidTimestamp(format!(
                    "Signature timestamp {} is before previous {}",
                    sig.timestamp.time, prev
                )));
            }
        }
        prev_time = Some(sig.timestamp.time);
    }
    Ok(())
}
```

---

### 3.8 Rate Limiting & Anomaly Detection
**Effort**: 3 days

```rust
// ratelimit.rs - Verification rate limiting
pub struct RateLimiter {
    window: Duration,
    max_requests: usize,
    requests: Mutex<VecDeque<Instant>>,
}

impl RateLimiter {
    pub fn check(&self) -> Result<(), RateLimitError> {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();

        // Remove old requests
        while let Some(front) = requests.front() {
            if now.duration_since(*front) > self.window {
                requests.pop_front();
            } else {
                break;
            }
        }

        if requests.len() >= self.max_requests {
            return Err(RateLimitError::TooManyRequests);
        }

        requests.push_back(now);
        Ok(())
    }
}
```

---

## Testing Strategy

### Unit Tests (Each Phase)
- Test each vulnerability fix in isolation
- Negative tests (attack scenarios should fail)
- Regression tests for existing functionality

### Integration Tests
```rust
#[test]
fn test_attack_chain_1_document_forgery() {
    // Attempt signature stripping + modification
    let signed = create_signed_document();
    let attacked = strip_signatures_and_modify(&signed);

    // Should fail with mandatory verification
    let result = verify_document(&attacked, None, false);
    assert!(result.is_err());
}

#[test]
fn test_attack_chain_2_revocation_bypass() {
    // Sign with key, revoke, attempt backdate
    let (signing_key, verifying_key) = generate_keypair();
    let signed = sign_document(&doc, &signing_key);

    // Revoke key
    let revocation = revoke_key(&signing_key, Utc::now());

    // Attempt to backdate signature
    let backdated = backdate_signature(&signed, days_ago(7));

    // Should still fail - timestamp bound to signature
    let result = verify_with_revocation(&backdated, &verifying_key, &revocation);
    assert!(result.is_err());
}
```

### Fuzzing
```bash
# Add cargo-fuzz targets
cargo fuzz add merkle_from_binary
cargo fuzz add cbor_deserialize
cargo fuzz add zip_extract
```

### Security Review Checklist
- [ ] All CRITICAL CVEs have tests
- [ ] All HIGH CVEs have tests
- [ ] Fuzz targets for parsing code
- [ ] Integration tests for attack chains
- [ ] Performance benchmarks (no regression)

---

## Rollout Strategy

### Version Scheme
- **v0.2.0**: Phase 1 fixes (breaking: Merkle tree format)
- **v0.3.0**: Phase 2 fixes (breaking: signature format)
- **v1.0.0**: Phase 3 + security certification

### Migration Path
```rust
// Version detection in verify
pub fn verify_document(doc: &TdfDocument) -> TdfResult<VerificationReport> {
    let version = doc.manifest.format_version;

    match version {
        "0.1" => {
            warn!("Legacy format detected - limited security guarantees");
            verify_v01(doc)
        }
        "0.2" => verify_v02(doc),
        "0.3" | "1.0" => verify_current(doc),
        _ => Err(TdfError::UnsupportedVersion(version.to_string())),
    }
}
```

### Communication
1. Security advisory for known vulnerabilities
2. Migration guide for format changes
3. Deprecation timeline for v0.1 format

---

## Success Criteria

### Phase 1 Complete When:
- [ ] All 9 CRITICAL CVEs fixed
- [ ] All unit tests pass
- [ ] Fuzzing finds no new crashes
- [ ] Security review sign-off

### Phase 2 Complete When:
- [ ] All 10 HIGH CVEs fixed
- [ ] Integration tests pass
- [ ] Attack chain tests fail (as expected)
- [ ] Performance within 10% of baseline

### Phase 3 Complete When:
- [ ] All CVEs addressed
- [ ] External security audit passed
- [ ] Documentation updated
- [ ] v1.0.0 release ready

---

## Resource Requirements

| Phase | Developer Days | Reviewer Days | Total |
|-------|---------------|---------------|-------|
| Phase 1 | 12 | 3 | 15 |
| Phase 2 | 15 | 4 | 19 |
| Phase 3 | 20 | 5 | 25 |
| **Total** | **47** | **12** | **59** |

---

*End of Security Remediation Plan*
