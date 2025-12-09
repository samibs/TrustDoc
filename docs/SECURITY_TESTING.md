# Security Testing Documentation

## Overview

This document describes the comprehensive security test suite for the TDF (TrustDoc Financial) format. The tests are designed to verify that the format can detect and prevent various attack scenarios including tampering, signature forgery, hash manipulation, and format validation attacks.

## Test Structure

### Unit Tests (`security_tests.rs`)

Located in `tdf-core/tests/security_tests.rs`, these tests focus on individual security components:

#### Tampering Detection Tests
- **`test_tamper_content_modification`**: Verifies that modifying document content is detected
- **`test_tamper_manifest_modification`**: Verifies that modifying manifest metadata is detected
- **`test_tamper_merkle_tree_manipulation`**: Verifies that tampering with the Merkle tree is detected
- **`test_tamper_styles_modification`**: Verifies that modifying CSS styles is detected
- **`test_tamper_add_malicious_file`**: Tests handling of malicious files added to ZIP

#### Signature Attack Tests
- **`test_signature_wrong_key_verification`**: Verifies that signatures fail with wrong keys
- **`test_signature_replacement_attack`**: Tests detection of signature replacement
- **`test_signature_modification_attack`**: Tests detection of signature byte manipulation
- **`test_signature_replay_attack`**: Verifies that signatures cannot be reused across documents

#### Hash Attack Tests
- **`test_hash_collision_resistance`**: Verifies different content produces different hashes
- **`test_merkle_tree_ordering_matters`**: Tests Merkle tree determinism
- **`test_root_hash_substitution_attack`**: Verifies detection of root hash manipulation

#### Format Validation Tests
- **`test_missing_required_file`**: Tests handling of missing required files
- **`test_malformed_cbor_attack`**: Tests handling of malformed CBOR data
- **`test_malformed_merkle_tree_attack`**: Tests handling of malformed Merkle tree
- **`test_empty_file_attack`**: Tests handling of empty files

#### Multi-Algorithm Tests
- **`test_secp256k1_signature_verification`**: Tests secp256k1 signature support
- **`test_blake3_hash_algorithm`**: Tests Blake3 hash algorithm
- **`test_cross_algorithm_attack`**: Tests that wrong algorithm keys fail verification

### End-to-End Tests (`e2e_security_tests.rs`)

Located in `tdf-core/tests/e2e_security_tests.rs`, these tests cover complete attack scenarios:

#### E2E Tampering Scenarios
- **`test_e2e_content_tampering_detection`**: Full workflow test of content tampering detection
- **`test_e2e_man_in_the_middle_attack`**: Tests MITM attack scenario with re-signing
- **`test_e2e_multi_party_signature_attack`**: Tests multi-party signing with compromised key
- **`test_e2e_key_compromise_scenario`**: Tests behavior when signing key is compromised
- **`test_e2e_timestamp_manipulation_attack`**: Tests timestamp backdating attempts
- **`test_e2e_zip_bomb_attack`**: Tests handling of ZIP decompression bombs
- **`test_e2e_path_traversal_attack`**: Tests path traversal in ZIP files
- **`test_e2e_complete_workflow_under_attack`**: Complete document lifecycle with multiple attacks

## Running the Tests

### Run All Security Tests
```bash
cargo test --test security_tests --test e2e_security_tests
```

### Run Specific Test
```bash
cargo test --test security_tests test_tamper_content_modification
```

### Run with Output
```bash
cargo test --test security_tests -- --nocapture
```

## Test Coverage

### Attack Vectors Covered

1. **Content Tampering**
   - ✅ Content modification
   - ✅ Manifest modification
   - ✅ Styles modification
   - ✅ Merkle tree manipulation

2. **Signature Attacks**
   - ✅ Wrong key verification
   - ✅ Signature replacement
   - ✅ Signature modification
   - ✅ Signature replay
   - ✅ Key compromise

3. **Hash Attacks**
   - ✅ Hash collision resistance
   - ✅ Root hash substitution
   - ✅ Merkle tree manipulation

4. **Format Attacks**
   - ✅ Missing required files
   - ✅ Malformed CBOR
   - ✅ Malformed Merkle tree
   - ✅ Empty files
   - ✅ ZIP bombs
   - ✅ Path traversal

5. **Multi-Party Scenarios**
   - ✅ Multiple signers
   - ✅ Compromised signer
   - ✅ Unauthorized signature addition

6. **Algorithm Tests**
   - ✅ Ed25519 signatures
   - ✅ secp256k1 signatures
   - ✅ SHA-256 hashing
   - ✅ Blake3 hashing
   - ✅ Cross-algorithm attacks

## Security Guarantees Tested

### Integrity
- ✅ Any modification to content is detectable
- ✅ Any modification to metadata is detectable
- ✅ Merkle tree integrity is verified
- ✅ Hash mismatches are detected

### Authentication
- ✅ Signatures verify with correct keys
- ✅ Signatures fail with wrong keys
- ✅ Signature replay is prevented
- ✅ Multi-party signatures are validated

### Format Validation
- ✅ Required files are validated
- ✅ File format is validated (CBOR, ZIP)
- ✅ Malformed data is rejected
- ✅ Path traversal is handled

## Expected Test Results

All security tests should **PASS** when the TDF format is functioning correctly. A failing test indicates a security vulnerability that must be addressed.

### Test Assertions

- **Tampering tests**: Should detect modifications and fail verification
- **Signature tests**: Should reject invalid signatures
- **Hash tests**: Should detect hash mismatches
- **Format tests**: Should reject malformed data

## Continuous Integration

These tests should be run:
- On every commit
- Before every release
- As part of security audits
- When adding new features

## Reporting Issues

If a security test fails:
1. **DO NOT** ignore the failure
2. Document the failure
3. Investigate the root cause
4. Fix the vulnerability
5. Add additional tests if needed
6. Update this documentation

## Future Enhancements

Potential additional tests to add:
- [ ] Performance under attack (DoS resistance)
- [ ] Side-channel attack resistance
- [ ] Timing attack resistance
- [ ] Certificate validation tests
- [ ] Timestamp authority validation
- [ ] Revocation list checks
- [ ] Large file handling
- [ ] Concurrent access tests

## References

- TDF Specification: `docs/SPEC.md`
- Security Considerations: `docs/SPEC.md#security-considerations`
- Implementation Status: `docs/IMPLEMENTATION_STATUS.md`

