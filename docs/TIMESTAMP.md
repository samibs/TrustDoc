# Timestamp Authority Integration

TDF supports trusted timestamping via RFC 3161 timestamp authorities.

## Manual Timestamps (Default)

By default, TDF uses manual timestamps - the current time when the document is signed. These are trusted based on the signer's credibility.

```rust
use tdf_core::timestamp::ManualTimestampProvider;

let provider = ManualTimestampProvider;
let signature = SignatureManager::sign_ed25519_with_timestamp(
    &signing_key,
    &root_hash,
    signer_id,
    signer_name,
    SignatureScope::Full,
    Some(&provider),
);
```

## RFC 3161 Timestamp Authority

For legal/compliance use cases, you can use an RFC 3161 timestamp authority (TSA).

### Setup

Enable the `rfc3161` feature:

```toml
[dependencies]
tdf-core = { path = "../tdf-core", features = ["rfc3161"] }
```

### Usage

```rust
use tdf_core::timestamp::Rfc3161TimestampProvider;

let provider = Rfc3161TimestampProvider::new("https://timestamp.digicert.com".to_string());
let signature = SignatureManager::sign_ed25519_with_timestamp(
    &signing_key,
    &root_hash,
    signer_id,
    signer_name,
    SignatureScope::Full,
    Some(&provider),
);
```

### Timestamp Verification

```rust
use tdf_core::timestamp::verify_timestamp_token;

let token = TimestampToken::from(&signature.timestamp);
let is_valid = verify_timestamp_token(&token, &root_hash)?;
```

## Timestamp Token Structure

```rust
pub struct TimestampToken {
    pub time: DateTime<Utc>,
    pub authority: String,        // TSA URL or "manual"
    pub proof: String,            // Base64-encoded RFC 3161 token
    pub algorithm: String,        // "rfc3161" or "manual"
}
```

## Public TSA Services

- DigiCert: `https://timestamp.digicert.com`
- GlobalSign: `https://timestamp.globalsign.com`
- Sectigo: `https://timestamp.sectigo.com`

## Implementation Notes

- Full RFC 3161 requires ASN.1 encoding/decoding
- Current implementation provides framework for integration
- Production use requires proper ASN.1 library (e.g., `der` or `x509-parser`)
- Async support available via `get_timestamp_async()`

