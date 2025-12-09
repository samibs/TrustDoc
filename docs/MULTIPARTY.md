# Multi-Party Signature Workflows

TDF supports complex multi-party signing workflows with ordering and validation.

## Signing Orders

### Unordered Signing

All signers can sign in any order:

```rust
use tdf_core::multiparty::{MultiPartySigningSession, SigningOrder};

let session = MultiPartySigningSession::new(
    root_hash,
    SigningOrder::Unordered,
    vec!["signer1".to_string(), "signer2".to_string()],
);
```

### Ordered Signing

Signers must sign in a specific order:

```rust
let session = MultiPartySigningSession::new(
    root_hash,
    SigningOrder::Ordered(vec!["ceo".to_string(), "cfo".to_string(), "legal".to_string()]),
    vec!["ceo".to_string(), "cfo".to_string(), "legal".to_string()],
);
```

### Simultaneous Signing

All signers must sign at the same time (not yet fully implemented):

```rust
let session = MultiPartySigningSession::new(
    root_hash,
    SigningOrder::Simultaneous,
    vec!["party1".to_string(), "party2".to_string()],
);
```

## Workflow Management

### Create Workflow

```bash
tdf workflow create document.tdf \
  --order ordered \
  --signers "did:web:ceo.com,did:web:cfo.com,did:web:legal.com" \
  -o workflow.json
```

### Check Status

```bash
tdf workflow status workflow.json
```

### Add Signatures

```rust
use tdf_core::multiparty::MultiPartySigningSession;

// Load session
let mut session = // ... load from file

// Add signature
session.add_signature(signature)?;

// Check if complete
if session.is_complete() {
    let signature_block = session.to_signature_block();
    // Save to document
}
```

## Workflow Status

```rust
pub enum WorkflowStatus {
    Pending,
    InProgress { signed_count: usize, total: usize },
    Completed,
    Rejected { reason: String },
}
```

## Example: Contract Signing

```bash
# 1. Create workflow
tdf workflow create contract.tdf \
  --order ordered \
  --signers "party-a,party-b,witness" \
  -o contract.workflow.json

# 2. Party A signs
tdf create contract.json -o contract-signed-a.tdf \
  --key party-a.signing \
  --signer-id "party-a" \
  --signer-name "Party A"

# 3. Party B signs (must be after A in ordered workflow)
tdf create contract-signed-a.tdf -o contract-signed-b.tdf \
  --key party-b.signing \
  --signer-id "party-b" \
  --signer-name "Party B"

# 4. Check workflow status
tdf workflow status contract.workflow.json
```

## Validation

The workflow validates:
- Signer is in required list
- Signer hasn't already signed
- Order is respected (for ordered workflows)
- All required signers have signed (for completion)

## Integration

Workflows can be:
- Stored as JSON files
- Managed by external systems
- Integrated with document management systems
- Used for audit trails

