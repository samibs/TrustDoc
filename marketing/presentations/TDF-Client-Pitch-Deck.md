# TDF Client Pitch Deck
## For Financial Services Decision Makers

---

# SLIDE 1: Title

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                         TRUSTDOC                               │
│                                                                │
│          Stop Trusting Documents.                              │
│          Start Verifying Them.                                 │
│                                                                │
│                                                                │
│              Cryptographic Document Security                   │
│                for Financial Services                          │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

# SLIDE 2: The Question

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                                                                │
│                                                                │
│       "Can you prove the CFO approved                          │
│              THESE exact numbers?"                             │
│                                                                │
│                                                                │
│                                                                │
│          If the answer is "not really"...                      │
│                                                                │
│          ...you have a €12 billion problem.                    │
│                                                                │
│                                                                │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

# SLIDE 3: The Reality

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│              TODAY'S DOCUMENT WORKFLOW                         │
│                                                                │
│                                                                │
│    Excel          PDF           Email         Sign             │
│   ┌─────┐       ┌─────┐       ┌─────┐       ┌─────┐           │
│   │     │  ──►  │     │  ──►  │     │  ──►  │     │           │
│   └─────┘       └─────┘       └─────┘       └─────┘           │
│                                                                │
│      │             │             │             │               │
│      ▼             ▼             ▼             ▼               │
│   Numbers       Numbers       Numbers      Signature           │
│   created       could         could be     on a                │
│                 change        modified     PICTURE             │
│                                                                │
│                                                                │
│         Nothing connects the signature to the data             │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

# SLIDE 4: Real Consequences

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│               REAL-WORLD CONSEQUENCES                          │
│                                                                │
│                                                                │
│   ┌─────────────────────────────────────────────────────────┐ │
│   │  "The invoice looked legitimate. We paid €1.4M          │ │
│   │   to the wrong account."                                │ │
│   │                               - Luxembourg Fund Admin   │ │
│   └─────────────────────────────────────────────────────────┘ │
│                                                                │
│   ┌─────────────────────────────────────────────────────────┐ │
│   │  "Two versions of the contract exist. Both have         │ │
│   │   valid signatures. Legal fees: €800K."                 │ │
│   │                               - Asset Manager           │ │
│   └─────────────────────────────────────────────────────────┘ │
│                                                                │
│   ┌─────────────────────────────────────────────────────────┐ │
│   │  "During the regulatory exam, we couldn't prove         │ │
│   │   which version of the statements we audited."          │ │
│   │                               - Audit Partner           │ │
│   └─────────────────────────────────────────────────────────┘ │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

# SLIDE 5: The TDF Solution

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                    THE TDF DIFFERENCE                          │
│                                                                │
│                                                                │
│   ╔═══════════════════════════════════════════════════════╗   │
│   ║                                                       ║   │
│   ║   1. INTEGRITY                                        ║   │
│   ║      Change one digit → Instantly detected            ║   │
│   ║                                                       ║   │
│   ╠═══════════════════════════════════════════════════════╣   │
│   ║                                                       ║   │
│   ║   2. AUTHENTICITY                                     ║   │
│   ║      Cryptographic proof of who approved what         ║   │
│   ║                                                       ║   │
│   ╠═══════════════════════════════════════════════════════╣   │
│   ║                                                       ║   │
│   ║   3. STRUCTURED DATA                                  ║   │
│   ║      Machine-readable → Direct system import          ║   │
│   ║                                                       ║   │
│   ╚═══════════════════════════════════════════════════════╝   │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

# SLIDE 6: How It Works (Simple)

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                    HOW TDF WORKS                               │
│                                                                │
│                                                                │
│      1. CREATE                    2. SIGN                      │
│      ┌─────────────┐              ┌─────────────┐              │
│      │ Structured  │              │  CFO signs  │              │
│      │ data locked │     ──►      │  the HASH   │              │
│      │ into TDF    │              │  of data    │              │
│      └─────────────┘              └─────────────┘              │
│                                          │                     │
│                                          ▼                     │
│      4. IMPORT                    3. VERIFY                    │
│      ┌─────────────┐              ┌─────────────┐              │
│      │ Data goes   │              │  Recipient  │              │
│      │ directly    │     ◄──      │  checks:    │              │
│      │ into ERP    │              │  tampered?  │              │
│      └─────────────┘              └─────────────┘              │
│                                                                │
│                                                                │
│           If ANYTHING changed → Verification fails             │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

# SLIDE 7: Before / After

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                     YOUR WORKFLOW                              │
│                                                                │
│   BEFORE                          AFTER                        │
│   ──────                          ─────                        │
│                                                                │
│   45 min to verify document       5 min auto-verify            │
│                                                                │
│   Manual data re-entry            Direct import                │
│                                                                │
│   3.2% error rate                 0% error rate                │
│                                                                │
│   "Which version?" disputes       Single source of truth       │
│                                                                │
│   €3-5 per signature              €0.05 per document           │
│                                                                │
│   Hope nothing changed            KNOW nothing changed         │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

# SLIDE 8: ROI Example

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│              ROI: MID-SIZE FUND ADMINISTRATOR                  │
│                                                                │
│                                                                │
│   ANNUAL SAVINGS                                               │
│   ┌─────────────────────────────────────────────────────────┐ │
│   │  Staff time reduction              €60,000              │ │
│   │  Error elimination                 €22,500              │ │
│   │  Fraud prevention                  €35,000              │ │
│   │  Signature cost savings             €8,400              │ │
│   │  ─────────────────────────────────────────              │ │
│   │  TOTAL SAVINGS                    €125,900              │ │
│   └─────────────────────────────────────────────────────────┘ │
│                                                                │
│   TDF ANNUAL COST                      €15,000                │
│                                                                │
│   ════════════════════════════════════════════                │
│                                                                │
│                    ROI: 739%                                   │
│                    Payback: 6 weeks                            │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

# SLIDE 9: Compliance Ready

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                   COMPLIANCE READY                             │
│                                                                │
│                                                                │
│   ┌──────────────┬─────────────────┬─────────────────────┐    │
│   │ REGULATION   │ REQUIREMENT     │ TDF SOLUTION        │    │
│   ├──────────────┼─────────────────┼─────────────────────┤    │
│   │ DORA 2025    │ ICT integrity   │ Cryptographic hash  │    │
│   ├──────────────┼─────────────────┼─────────────────────┤    │
│   │ MiFID II     │ Record keeping  │ Tamper-evident      │    │
│   ├──────────────┼─────────────────┼─────────────────────┤    │
│   │ GDPR         │ Data integrity  │ Merkle tree proof   │    │
│   ├──────────────┼─────────────────┼─────────────────────┤    │
│   │ AML 6        │ Authentication  │ Digital signatures  │    │
│   ├──────────────┼─────────────────┼─────────────────────┤    │
│   │ E-Invoice 28 │ Structured data │ Native support      │    │
│   └──────────────┴─────────────────┴─────────────────────┘    │
│                                                                │
│                                                                │
│          TDF is built for the regulatory future                │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

# SLIDE 10: vs DocuSign

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                    TDF vs DOCUSIGN                             │
│                                                                │
│                                                                │
│                            DocuSign        TDF                 │
│   ───────────────────────────────────────────────────         │
│                                                                │
│   Content integrity           ✗            ✓                   │
│                                                                │
│   Structured data             ✗            ✓                   │
│                                                                │
│   Offline verification        ✗            ✓                   │
│                                                                │
│   Open standard               ✗            ✓                   │
│                                                                │
│   Per-document fee         €3-5           €0.05                │
│                                                                │
│   ───────────────────────────────────────────────────         │
│                                                                │
│   5-Year TCO (100K docs)   €300K          €45K                 │
│                                                                │
│                                                                │
│                      85% cost savings                          │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

# SLIDE 11: Implementation

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                   IMPLEMENTATION PATH                          │
│                                                                │
│                                                                │
│   PHASE 1              PHASE 2              PHASE 3            │
│   Pilot                Department           Enterprise         │
│   ┌────────┐           ┌────────┐           ┌────────┐         │
│   │ 4 weeks│    ──►    │3 months│    ──►    │6 months│         │
│   └────────┘           └────────┘           └────────┘         │
│                                                                │
│   • 1 doc type         • All doc types      • All departments  │
│   • 5 users            • 20 users           • Full rollout     │
│   • Measure ROI        • Integrate ERP      • External parties │
│                                                                │
│   Cost: FREE           Cost: €15K           Cost: €40K         │
│                                                                │
│                                                                │
│          Start with zero risk, scale based on results          │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

# SLIDE 12: Next Steps

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                      NEXT STEPS                                │
│                                                                │
│                                                                │
│                                                                │
│        ┌─────────────────────────────────────────────┐        │
│        │                                             │        │
│        │   1. DEMO (Today)                           │        │
│        │      See TDF create, sign, verify           │        │
│        │                                             │        │
│        │   2. ROI ASSESSMENT (This Week)             │        │
│        │      Custom calculation for your volumes    │        │
│        │                                             │        │
│        │   3. PILOT PROPOSAL (Next Week)             │        │
│        │      4-week free pilot, defined scope       │        │
│        │                                             │        │
│        └─────────────────────────────────────────────┘        │
│                                                                │
│                                                                │
│                   Ready to see it in action?                   │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

*End of Client Pitch Deck*
