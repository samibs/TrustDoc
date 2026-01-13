# TDF Security Audit Report
## Red Team Assessment - Adversarial Analysis

**Classification**: INTERNAL - SECURITY SENSITIVE
**Date**: 2026-01-10
**Auditor**: Adversarial Security Analysis
**Scope**: Complete TDF Protocol and Implementation

---

## Executive Summary

This red team assessment identified **27 security vulnerabilities** across the TrustDoc Format (TDF) implementation, ranging from **CRITICAL** to **LOW** severity. The most severe findings allow complete bypass of signature verification, timestamp manipulation, and revocation circumvention.

### Severity Distribution

| Severity | Count | Immediate Action Required |
|----------|-------|---------------------------|
| CRITICAL | 9 | YES - Block deployment |
| HIGH | 10 | YES - Fix before production |
| MEDIUM | 6 | Recommended fix |
| LOW | 2 | Best practice improvements |

### Top 3 Critical Findings

1. **Signature Verification Bypass** - Signatures are completely optional; documents verify without any cryptographic proof
2. **Timestamp Manipulation** - Timestamps can be modified post-signature, enabling backdating attacks
3. **Memory Exhaustion DoS** - ZIP bomb protection can be bypassed via unbounded `read_to_end()` calls

---

## CRITICAL VULNERABILITIES

### CVE-TDF-001: Signature Stripping Attack
**Severity**: CRITICAL | **CVSS**: 9.8 | **Exploitability**: Trivial

**Location**: `tdf-cli/src/commands/verify.rs:194-198`

**Description**: Signature verification is entirely optional. When no `--key` flag is provided, verification succeeds without checking any signatures.

**Vulnerable Code**:
```rust
if report.signature_count > 0 {
    // signature verification logic
} else if key.is_some() {
    println!("  No signatures to verify.");
} else {
    println!("  Use --key to verify signatures.");
}
// Document still reports as "VERIFIED"
```

**Attack Scenario**:
1. Attacker intercepts signed TDF document
2. Removes all signatures from archive
3. Modifies content (changes financial figures)
4. Recipient verifies without `--key` flag
5. Document reports "VERIFIED" - attack succeeds

**Impact**: Complete loss of document authenticity guarantees

**Proof of Concept**:
```bash
# Create legitimate signed document
tdf create invoice.json --output legit.tdf --key cfo.signing

# Attack: strip signatures
unzip legit.tdf -d attack/
rm attack/signatures.cbor
cd attack && zip -q ../attack.tdf * && cd ..

# Verify "succeeds" without key
tdf verify attack.tdf
# Output: "RESULT: DOCUMENT VERIFIED"
```

---

### CVE-TDF-002: Merkle Tree Domain Separation Missing
**Severity**: CRITICAL | **CVSS**: 8.1 | **Exploitability**: Medium

**Location**: `tdf-core/src/merkle.rs:86-93`

**Description**: The Merkle tree construction lacks domain separation between leaf and internal nodes. An attacker can craft colliding trees with identical root hashes.

**Vulnerable Code**:
```rust
let combined = if chunk.len() == 2 {
    let mut combined = chunk[0].clone();
    combined.extend_from_slice(&chunk[1]);  // No domain separator!
    combined
} else {
    chunk[0].clone();
};
```

**Attack Scenario**: Second-preimage attack where `hash(leaf_A || leaf_B)` could equal `hash(internal_node)` if carefully constructed.

**Impact**: Two different documents could produce identical root hashes, breaking integrity guarantees.

**Remediation**: Add domain separators (`0x00` for leaves, `0x01` for internal nodes) before hashing.

---

### CVE-TDF-003: Timestamp Not Bound to Signature
**Severity**: CRITICAL | **CVSS**: 9.1 | **Exploitability**: Trivial

**Location**: `tdf-core/src/signature.rs:15-24`

**Description**: Timestamps are stored separately from signatures and are never validated against the signed content. Timestamps can be freely modified after signing.

**Vulnerable Code**:
```rust
pub struct DocumentSignature {
    pub timestamp: TimestampInfo,  // Separate field, not part of signed data!
    pub signature: String,         // Signs hash only, not timestamp
    pub root_hash: String,
}
```

**Attack Scenario**:
1. Sign document today (2026-01-10)
2. After signing, modify `timestamp.time` to past date (2025-01-10)
3. Document now appears to be signed a year ago
4. Verification passes - timestamp is never validated

**Impact**:
- Document backdating for legal/financial fraud
- Circumvention of revocation (sign after key revoked, backdate to before revocation)
- Timeline manipulation in audit trails

---

### CVE-TDF-004: RFC 3161 Proof Never Validated
**Severity**: CRITICAL | **CVSS**: 8.6 | **Exploitability**: Trivial

**Location**: `tdf-core/src/timestamp.rs:187-204`

**Description**: RFC 3161 timestamp proofs are never actually validated. Any base64 string is accepted as a "valid" proof.

**Vulnerable Code**:
```rust
"rfc3161" => {
    if token.proof.is_empty() {
        result.add_warning("RFC 3161 timestamp token missing proof");
    } else {
        // In full implementation, would validate ASN.1...
        result.add_warning("RFC 3161 proof present but not fully validated");
    }
}
// No actual validation occurs - any proof string accepted!
```

**Attack Scenario**: Attacker claims any timestamp authority signed their document by providing fake proof bytes.

**Impact**: RFC 3161 timestamps are purely decorative - provide no actual cryptographic proof of time.

---

### CVE-TDF-005: Memory Exhaustion via Unbounded Read
**Severity**: CRITICAL | **CVSS**: 7.5 | **Exploitability**: Medium

**Location**: `tdf-core/src/archive.rs:286-350`

**Description**: `read_to_end()` is called without size limits after ZIP decompression ratio check, allowing memory exhaustion.

**Vulnerable Code**:
```rust
// Size check happens on COMPRESSED size
security_config.check_file_size(file.size())?;

// But read_to_end allocates for UNCOMPRESSED size
let mut bytes = Vec::new();
manifest_file.read_to_end(&mut bytes)?;  // Can allocate gigabytes!
```

**Attack Scenario**:
1. Create ZIP with highly compressible content (e.g., 5MB compressed → 5GB uncompressed)
2. Decompression ratio check passes (1000:1 limit)
3. `read_to_end()` tries to allocate 5GB
4. OOM crash or system freeze

**Impact**: Denial of Service against verification infrastructure

---

### CVE-TDF-006: Revocation Bypass via Timestamp Manipulation
**Severity**: CRITICAL | **CVSS**: 8.8 | **Exploitability**: Trivial

**Location**: `tdf-core/src/revocation.rs:85-91` combined with CVE-TDF-003

**Description**: Revocation check uses signature timestamp, which can be freely modified. Attacker backdates signature to before revocation.

**Vulnerable Code**:
```rust
pub fn is_revoked_at(&self, signer_id: &str, check_time: DateTime<Utc>) -> Option<&RevocationEntry> {
    self.revoked_keys.iter().find(|entry| {
        entry.signer_id == signer_id && entry.revoked_at <= check_time
    })
}
```

**Attack Scenario**:
1. Key revoked on 2026-01-10
2. Attacker signs document on 2026-01-15 with revoked key
3. Modifies timestamp to 2026-01-05 (before revocation)
4. Revocation check: `revoked_at (Jan 10) <= signature_time (Jan 5)` = FALSE
5. Signature accepted as valid

**Impact**: Revoked keys can continue signing documents indefinitely by backdating.

---

### CVE-TDF-007: Signature-to-RootHash Binding Missing
**Severity**: CRITICAL | **CVSS**: 8.4 | **Exploitability**: Medium

**Location**: `tdf-core/src/archive.rs:605-607`

**Description**: During verification, signatures are validated cryptographically but never checked against the computed Merkle root hash.

**Vulnerable Code**:
```rust
let integrity_valid = merkle_tree.verify(&components)?;
let root_hash = merkle_tree.root_hash().to_vec();

// Signatures verified but sig.root_hash never compared with computed root_hash!
for sig in &signature_block.signatures {
    // verify_ed25519() checks signature is valid, but not that it signed THIS hash
}
```

**Attack Scenario**:
1. Obtain legitimate signatures from Document A
2. Create new Document B with different content
3. Attach Document A's signatures to Document B
4. Verification passes: signatures are valid (for wrong document)

**Impact**: Signature reuse across different documents

---

### CVE-TDF-008: Integer Overflow in Size Calculation
**Severity**: CRITICAL | **CVSS**: 7.2 | **Exploitability**: Medium

**Location**: `tdf-core/src/merkle.rs:179-181`

**Description**: Integer overflow in Merkle tree binary deserialization allows buffer overread.

**Vulnerable Code**:
```rust
let count = u32::from_be_bytes([data[6], data[7], data[8], data[9]]) as usize;

if data.len() < 10 + 32 + (count * 32) {  // count * 32 can overflow!
    return Err(...);
}
```

**Attack Scenario**: Send `count = 0xFFFFFFFF`, causing `count * 32` to overflow to small value, bypassing bounds check.

**Impact**: Out-of-bounds read, potential information disclosure or crash

---

### CVE-TDF-009: Unbounded CBOR Deserialization
**Severity**: CRITICAL | **CVSS**: 7.5 | **Exploitability**: Medium

**Location**: `tdf-core/src/archive.rs:289, 299, 315, 326, 350`

**Description**: CBOR deserialization has no recursion depth limit or size bounds, allowing stack exhaustion.

**Vulnerable Code**:
```rust
let manifest: Manifest = serde_cbor::from_slice(&manifest_bytes)?;
// No depth limit, deeply nested structures cause stack overflow
```

**Impact**: Denial of Service via malformed CBOR causing stack overflow

---

## HIGH SEVERITY VULNERABILITIES

### CVE-TDF-010: Algorithm Downgrade Attack
**Severity**: HIGH | **CVSS**: 6.8

**Location**: `tdf-core/src/signature.rs:270-336`

**Description**: No algorithm policy enforcement. Weak or unsupported algorithms are accepted with warnings, not errors.

---

### CVE-TDF-011: Revocation Check Optional
**Severity**: HIGH | **CVSS**: 7.1

**Location**: `tdf-core/src/signature.rs:257-268`

**Description**: If no revocation list provided, revocation checking is completely skipped.

---

### CVE-TDF-012: Whitelist Bypass - Advisory Only
**Severity**: HIGH | **CVSS**: 6.5

**Location**: `tdf-cli/src/commands/verify.rs:169-185`

**Description**: Whitelist violations produce warnings only, not failures. Untrusted signers accepted unless `--strict` mode.

---

### CVE-TDF-013: Replay Attack via Timestamp Reuse
**Severity**: HIGH | **CVSS**: 7.3

**Location**: `tdf-core/src/timestamp.rs:159-239`

**Description**: `verify_timestamp_token_with_config()` receives `_data` parameter (unused) - timestamps never validated against content.

---

### CVE-TDF-014: Path Traversal in Assets
**Severity**: HIGH | **CVSS**: 6.1

**Location**: `tdf-core/src/archive.rs:56-62`

**Description**: `add_asset()` accepts paths containing `..` without validation.

---

### CVE-TDF-015: Decompression Ratio Bypass (Zero Divisor)
**Severity**: HIGH | **CVSS**: 6.8

**Location**: `tdf-core/src/config.rs:108-119`

**Description**: When `compressed == 0` (stored files), ratio check returns `Ok()` without validation.

---

### CVE-TDF-016: Multiparty Signature Verification Skipped
**Severity**: HIGH | **CVSS**: 6.5

**Location**: `tdf-core/src/multiparty.rs:78-80`

**Description**: `add_signature()` doesn't verify signatures cryptographically, only checks structure.

---

### CVE-TDF-017: Async RFC 3161 Unimplemented
**Severity**: HIGH | **CVSS**: 5.9

**Location**: `tdf-core/src/timestamp.rs:88-99`

**Description**: `Rfc3161TimestampProvider` returns empty proof, never contacts actual TSA.

---

### CVE-TDF-018: Integrity Failure Returns Success
**Severity**: HIGH | **CVSS**: 6.3

**Location**: `tdf-cli/src/commands/verify.rs:86-93`

**Description**: Without `--strict`, integrity failures return exit code 0.

---

### CVE-TDF-019: Signature Scope Not Enforced
**Severity**: HIGH | **CVSS**: 5.8

**Location**: `tdf-core/src/signature.rs:84-92`

**Description**: `SignatureScope::Sections(vec)` never validated against actual document sections.

---

## MEDIUM SEVERITY VULNERABILITIES

### CVE-TDF-020: No Maximum Asset Count
**Location**: `tdf-core/src/config.rs:75`
`max_file_count` defined but never enforced.

### CVE-TDF-021: add_asset() Fails Silently
**Location**: `tdf-core/src/archive.rs:56-62`
Size check failure logged but asset still inserted.

### CVE-TDF-022: Hardcoded Hash Sizes
**Location**: `tdf-core/src/merkle.rs:187-199`
Assumes all hashes are 32 bytes; breaks if algorithm changes.

### CVE-TDF-023: Timestamp Ordering Not Enforced
**Location**: `tdf-core/src/timestamp.rs:155-239`
Multi-signature timestamps not validated for chronological order.

### CVE-TDF-024: Whitelist Public Key Never Used
**Location**: `tdf-core/src/whitelist.rs:22-36`
`public_key` field exists but never validated during verification.

### CVE-TDF-025: Revocation Authority Not Verified
**Location**: `tdf-core/src/revocation.rs:31-45`
Authority field is just text, never cryptographically validated.

---

## LOW SEVERITY VULNERABILITIES

### CVE-TDF-026: Unwrap Panic in Size Estimation
**Location**: `tdf-core/src/archive.rs:198`
Production panic possible if CBOR serialization fails.

### CVE-TDF-027: Clock Skew Non-Configurable
**Location**: `tdf-cli/src/commands/verify.rs:610`
Hardcoded 5-minute skew tolerance, not configurable from CLI.

---

## DEPENDENCY ANALYSIS

### Current Dependencies (from Cargo.toml)

| Dependency | Version | Status |
|------------|---------|--------|
| ed25519-dalek | 2.1 | Secure |
| k256 | 0.13 | Secure |
| sha2 | 0.10 | Secure |
| blake3 | 1.5 | Secure |
| serde_cbor | 0.11 | Unmaintained (use ciborium) |
| zip | 0.6 | Check for updates |
| chrono | 0.4 | Known issues with local time |

### Recommendations
- Replace `serde_cbor` with `ciborium` (actively maintained)
- Update `zip` to latest version
- Consider `time` crate instead of `chrono` for security-critical timestamps

---

## OPERATIONAL SECURITY GAPS

### Key Management
1. **No key encryption at rest** - Private keys stored as raw bytes in `.signing` files
2. **No HSM integration** - Keys loaded directly from filesystem
3. **No key derivation** - Keys generated directly, no hierarchical key structure
4. **No key rotation support** - No mechanism for key versioning or succession

### Side-Channel Considerations
1. **Timing attacks** - Signature verification not constant-time in application layer
2. **Error messages** - Different error messages may leak information about key validity

### Logging & Monitoring
1. **No audit logging** - Verification operations not logged
2. **No anomaly detection** - No tracking of verification failures

---

## ATTACK CHAIN EXAMPLES

### Attack Chain 1: Complete Document Forgery
```
CVE-TDF-001 (Signature Stripping)
  + CVE-TDF-018 (Success on Failure)
  = Complete document modification undetected
```

### Attack Chain 2: Revocation Circumvention
```
CVE-TDF-003 (Timestamp Manipulation)
  + CVE-TDF-006 (Revocation Bypass)
  = Revoked keys remain valid indefinitely
```

### Attack Chain 3: RFC 3161 Authority Spoofing
```
CVE-TDF-004 (Proof Not Validated)
  + CVE-TDF-013 (Replay Attack)
  = Claim any timestamp authority signed document
```

### Attack Chain 4: Denial of Service
```
CVE-TDF-005 (Memory Exhaustion)
  + CVE-TDF-009 (CBOR Stack Overflow)
  = Crash verification infrastructure
```

---

## REMEDIATION PRIORITY

### Immediate (Block Deployment)
1. Make signature verification mandatory
2. Bind timestamps to signed content
3. Validate signatures against computed root hash
4. Add bounded readers for all file operations
5. Implement CBOR depth limits

### Short-Term (Before Production)
1. Implement actual RFC 3161 validation
2. Enforce algorithm whitelist
3. Make revocation checking mandatory
4. Add domain separators to Merkle tree
5. Fix integer overflow in size calculations

### Medium-Term (Production Hardening)
1. Add HSM integration for key management
2. Implement comprehensive audit logging
3. Add rate limiting and anomaly detection
4. Replace serde_cbor with ciborium
5. Add constant-time comparison functions

---

## VERIFICATION HARDENING CHECKLIST

```
[ ] Signature verification is MANDATORY (no --key = fail)
[ ] Timestamps are cryptographically bound to signatures
[ ] RFC 3161 proofs are properly validated via ASN.1
[ ] Merkle tree uses domain separators
[ ] All file reads have size bounds
[ ] CBOR has recursion depth limits
[ ] Algorithm whitelist is enforced
[ ] Revocation checking is mandatory
[ ] Whitelist violations are failures (not warnings)
[ ] Exit codes reflect actual verification status
```

---

## CONCLUSION

The TDF implementation has fundamental security design flaws that allow complete bypass of its core security guarantees. The most critical issues stem from **optional verification** patterns where security features exist but are not enforced.

**Deployment Recommendation**: DO NOT deploy to production until CRITICAL and HIGH severity issues are resolved.

**Estimated Remediation Effort**: 4-6 weeks for critical fixes, 2-3 months for full hardening.

---

*End of Security Audit Report*
