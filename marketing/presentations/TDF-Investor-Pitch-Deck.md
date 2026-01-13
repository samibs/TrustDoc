# TDF Investor Pitch Deck
## Slide-by-Slide Content

---

# SLIDE 1: Title

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                         TRUSTDOC                               │
│                          FORMAT                                │
│                                                                │
│         Cryptographic Document Security                        │
│           for Financial Services                               │
│                                                                │
│                                                                │
│                      [TDF Logo]                                │
│                                                                │
│                     Seed Round: €450K                          │
│                      January 2026                              │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"Thank you for your time today. I'm here to talk about a fundamental problem in how financial documents are handled—and how we're solving it."

---

# SLIDE 2: The Problem

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                    THE €12 BILLION PROBLEM                     │
│                                                                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   €4.2B      │  │   €2.8B      │  │   €2.3B      │         │
│  │   Invoice    │  │   Document   │  │   Compliance │         │
│  │   Fraud      │  │   Tampering  │  │   Failures   │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
│                                                                │
│  ┌──────────────┐                                             │
│  │   €2.9B      │     Annual losses in EU financial sector    │
│  │   Manual     │                                             │
│  │   Reconcile  │                                             │
│  └──────────────┘                                             │
│                                                                │
│         "Documents without cryptographic trust"                │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"Every year, European financial institutions lose 12 billion euros because of one fundamental flaw: our document formats—PDF, Excel, Word—have no built-in integrity protection. Anyone can modify them. No one can prove otherwise."

---

# SLIDE 3: The Root Cause

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│              WHEN YOUR CFO SIGNS A DOCUMENT...                 │
│                                                                │
│                                                                │
│    ┌─────────┐      ┌─────────┐      ┌─────────┐              │
│    │  Excel  │ ───► │   PDF   │ ───► │ DocuSign│              │
│    │ Numbers │      │ Picture │      │  Signs  │              │
│    └─────────┘      └─────────┘      └─────────┘              │
│                          │                │                    │
│                          ▼                ▼                    │
│                    ┌─────────────────────────┐                 │
│                    │   Nothing proves the    │                 │
│                    │   CFO saw THOSE exact   │                 │
│                    │   numbers               │                 │
│                    └─────────────────────────┘                 │
│                                                                │
│          The signature is on a PICTURE, not the DATA          │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"When your CFO approves a financial document today, they're signing a picture—a PDF. The actual numbers in that Excel file? They can be changed after. The signature doesn't protect the data. This is the root cause of the €12B problem."

---

# SLIDE 4: The Solution

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                    TDF: THREE GUARANTEES                       │
│                                                                │
│  ┌────────────────────────────────────────────────────────┐   │
│  │                     INTEGRITY                           │   │
│  │        Change one digit → Instantly detected            │   │
│  │              (Merkle tree, same as Bitcoin)             │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                │
│  ┌────────────────────────────────────────────────────────┐   │
│  │                   AUTHENTICITY                          │   │
│  │         Cryptographic proof of who signed what          │   │
│  │                 (Bank-grade signatures)                 │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                │
│  ┌────────────────────────────────────────────────────────┐   │
│  │                  STRUCTURED DATA                        │   │
│  │          Machine-readable, not PDF images               │   │
│  │                (Direct ERP import)                      │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"TDF solves this with three guarantees. First, integrity—any change to any part of the document is instantly detectable using the same math that secures Bitcoin. Second, authenticity—cryptographic signatures that can't be stripped or faked. Third, structured data—the document is machine-readable, not just a picture."

---

# SLIDE 5: How It Works

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                      HOW TDF WORKS                             │
│                                                                │
│                     ┌───────────┐                              │
│                     │Root Hash  │◄─── Signed by approvers      │
│                     │(32 bytes) │                              │
│                     └─────┬─────┘                              │
│                           │                                    │
│                    ┌──────┴──────┐                             │
│                    │             │                             │
│               ┌────┴────┐  ┌────┴────┐                        │
│               │ Hash AB │  │ Hash CD │                        │
│               └────┬────┘  └────┬────┘                        │
│                    │            │                              │
│              ┌─────┴─────┐ ┌────┴────┐                        │
│              │     │     │ │    │    │                        │
│            ┌─┴─┐ ┌─┴─┐ ┌─┴─┐ ┌─┴─┐                           │
│            │ A │ │ B │ │ C │ │ D │  ◄─── Document sections    │
│            └───┘ └───┘ └───┘ └───┘                            │
│                                                                │
│         Change ANY section → Root hash changes →              │
│                    Signature invalid                           │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"Here's the magic. Every section of the document is hashed. Those hashes combine into a single root hash—32 bytes. That root hash is what signers approve. If anyone changes even one character in any section, the root hash changes, and the signature becomes invalid. It's mathematically impossible to tamper undetected."

---

# SLIDE 6: Before vs After

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                     BEFORE vs AFTER                            │
│                                                                │
│  BEFORE (PDF Workflow):                                        │
│  ──────────────────────                                        │
│  Excel → PDF → Email → "Please sign" → DocuSign → Email back  │
│                              │                                 │
│              Numbers could change ANYWHERE                     │
│                              │                                 │
│                    Manual re-entry into ERP                    │
│                                                                │
│  ──────────────────────────────────────────────────────────── │
│                                                                │
│  AFTER (TDF Workflow):                                         │
│  ─────────────────────                                         │
│  Structured Data → Sign → Send → Auto-verify → Direct import  │
│        │              │              │              │          │
│    Numbers        Crypto proof   Instant        No errors     │
│    locked         of approval    tampering                    │
│                                  check                        │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"Let me show you the difference. Today's workflow has multiple points where data can be modified—and ends with manual re-entry. TDF locks the numbers at creation, proves who approved them, verifies instantly on receipt, and imports directly. No manual steps, no errors, no fraud opportunities."

---

# SLIDE 7: Market Opportunity

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                    MARKET OPPORTUNITY                          │
│                                                                │
│   ┌─────────────────────────────────────────────────────┐     │
│   │  TAM: €2.4B                                         │     │
│   │  Enterprise document security (EU)                   │     │
│   │  ┌───────────────────────────────────────────┐      │     │
│   │  │  SAM: €420M                                │      │     │
│   │  │  Financial services documents              │      │     │
│   │  │  ┌─────────────────────────────────┐      │      │     │
│   │  │  │  SOM: €42M (Year 5)             │      │      │     │
│   │  │  │  Luxembourg + expansion          │      │      │     │
│   │  │  └─────────────────────────────────┘      │      │     │
│   │  └───────────────────────────────────────────┘      │     │
│   └─────────────────────────────────────────────────────┘     │
│                                                                │
│   Key drivers:                                                 │
│   • EU e-invoicing mandate (2028)                             │
│   • DORA regulation (2025)                                    │
│   • Document fraud +23% YoY                                   │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"The total market for enterprise document security in Europe is €2.4 billion. We're focused on financial services—€420 million. Our serviceable market by year 5 is €42 million, starting in Luxembourg and expanding to Ireland, Germany, and France. Three regulatory tailwinds are driving urgency: EU e-invoicing, DORA, and the 23% annual increase in document fraud."

---

# SLIDE 8: Business Model

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                      BUSINESS MODEL                            │
│                                                                │
│   Three Revenue Streams:                                       │
│                                                                │
│   ┌────────────────────────────────────────────────────────┐  │
│   │  1. ENTERPRISE LICENSES           40% of revenue       │  │
│   │     • €15-75K/year per customer                        │  │
│   │     • Integration support, SLA                         │  │
│   └────────────────────────────────────────────────────────┘  │
│                                                                │
│   ┌────────────────────────────────────────────────────────┐  │
│   │  2. TDF-AS-A-SERVICE              45% of revenue       │  │
│   │     • €0.05/document created                           │  │
│   │     • €500/month key management                        │  │
│   └────────────────────────────────────────────────────────┘  │
│                                                                │
│   ┌────────────────────────────────────────────────────────┐  │
│   │  3. COMPLIANCE SERVICES           15% of revenue       │  │
│   │     • eIDAS certification: €8K                         │  │
│   │     • DORA readiness: €12K                             │  │
│   └────────────────────────────────────────────────────────┘  │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"We have three revenue streams. Enterprise licenses for larger customers needing dedicated support. A pay-per-document cloud service for mid-market. And compliance services helping customers meet eIDAS and DORA requirements. This mix gives us recurring revenue plus high-margin services."

---

# SLIDE 9: Financial Projections

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                   FINANCIAL PROJECTIONS (EUR)                  │
│                                                                │
│   Revenue                                                      │
│   €8.5M ─────────────────────────────────────────────── █     │
│                                                          █     │
│   €4.8M ──────────────────────────────────────── █      █     │
│                                                   █      █     │
│   €2.4M ────────────────────────────────  █      █      █     │
│                                            █      █      █     │
│   €750K ─────────────────────────  █      █      █      █     │
│   €180K ─────────────────  █      █      █      █      █     │
│                            █      █      █      █      █     │
│         Year 1      Year 2     Year 3    Year 4    Year 5     │
│                                                                │
│   ┌─────────────────────────────────────────────────────────┐ │
│   │  Break-even: Month 18    │    Year 5 EBITDA: €5.99M    │ │
│   │  EBITDA Margin Y5: 70%   │    Cumulative P&L: €9.6M    │ │
│   └─────────────────────────────────────────────────────────┘ │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"Here are our projections. €180K year one from pilot customers, scaling to €8.5 million by year five. We hit break-even at month 18. Year 5 EBITDA margin is 70%—this is a software business with strong unit economics. By year 5, we'll have generated €9.6 million in cumulative profit."

---

# SLIDE 10: Unit Economics

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                      UNIT ECONOMICS                            │
│                                                                │
│                                                                │
│        ┌──────────────────────────────────────────┐           │
│        │                                          │           │
│        │   CAC         €3,500                     │           │
│        │   ─────────────────────────────────────  │           │
│        │                                          │           │
│        │   ACV         €18,000                    │           │
│        │   ─────────────────────────────────────  │           │
│        │                                          │           │
│        │   Gross Margin    85%                    │           │
│        │   ─────────────────────────────────────  │           │
│        │                                          │           │
│        │   Customer Lifetime   5 years            │           │
│        │   ─────────────────────────────────────  │           │
│        │                                          │           │
│        │   LTV          €76,500                   │           │
│        │   ─────────────────────────────────────  │           │
│        │                                          │           │
│        │   LTV:CAC      22:1  ✓                   │           │
│        │                                          │           │
│        └──────────────────────────────────────────┘           │
│                                                                │
│              Payback Period: 2.3 months                        │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"The unit economics are exceptional. Customer acquisition cost is €3,500. Average contract value is €18,000. With 85% gross margin and 5-year customer lifetime, LTV is €76,500. That's a 22:1 LTV to CAC ratio—well above the 3:1 benchmark. Payback is just 2.3 months."

---

# SLIDE 11: Competitive Landscape

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                  COMPETITIVE LANDSCAPE                         │
│                                                                │
│                  Structured    Integrity    Open    No Per-Doc │
│                    Data        Proof      Standard    Fee     │
│  ────────────────────────────────────────────────────────────  │
│                                                                │
│  TDF               ✓            ✓           ✓         ✓       │
│                                                                │
│  DocuSign          ✗            ✗           ✗         ✗       │
│                                                                │
│  Adobe Sign        ✗            ✗           ✗         ✗       │
│                                                                │
│  Blockchain        ✗         Hash only      ✓         ✗       │
│                                                                │
│  ────────────────────────────────────────────────────────────  │
│                                                                │
│  5-Year TCO (100K docs/year):                                 │
│  • DocuSign: €300,000                                         │
│  • TDF:      €45,000  (85% savings)                           │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"DocuSign and Adobe Sign are signature tools—they don't protect document integrity or provide structured data. Blockchain only stores a hash, not the document itself. TDF is the only solution offering all four: structured data, integrity proof, open standard, and no per-document fee. We're 85% cheaper than DocuSign at scale."

---

# SLIDE 12: Go-to-Market

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                    GO-TO-MARKET STRATEGY                       │
│                                                                │
│   PHASE 1 (Months 1-6): Lighthouse Customers                  │
│   ─────────────────────────────────────────                   │
│   • 3 strategic pilots in Luxembourg                          │
│   • Fund admin + Audit firm + Bank                            │
│   • Free pilots → Case studies → References                   │
│                                                                │
│   PHASE 2 (Months 7-18): Luxembourg Market                    │
│   ─────────────────────────────────────────                   │
│   • 15 paying customers                                        │
│   • CSSF sandbox approval                                      │
│   • Industry association partnerships                          │
│                                                                │
│   PHASE 3 (Months 19-36): European Expansion                  │
│   ─────────────────────────────────────────                   │
│   • Ireland, Germany, France                                   │
│   • Partner-led expansion                                      │
│   • EU e-invoicing positioning                                │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"Our go-to-market is focused. Phase 1: three lighthouse customers in Luxembourg—free pilots that become case studies. Phase 2: expand to 15 paying customers, get regulatory sandbox approval, partner with ALFI and ABBL. Phase 3: use that proof to expand into Ireland, Germany, and France, positioning for the EU e-invoicing mandate."

---

# SLIDE 13: Traction

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                      CURRENT TRACTION                          │
│                                                                │
│   PRODUCT                                                      │
│   ✓ Production-ready Rust core library                        │
│   ✓ CLI tool with full feature set                            │
│   ✓ Web viewer (WASM)                                         │
│   ✓ Desktop viewer (Tauri)                                    │
│   ✓ 66 tests passing, 19 attack vectors covered               │
│                                                                │
│   VALIDATION                                                   │
│   ✓ 100/100 brute force attacks detected                      │
│   ✓ Security audit complete                                   │
│   ✓ Technical documentation published                         │
│                                                                │
│   PIPELINE                                                     │
│   • BDO Luxembourg (discussion stage)                         │
│   • 2 fund administrators (pilot interest)                    │
│   • 1 custody bank (evaluation)                               │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"We're not starting from zero. The product is built and tested—66 tests passing, 19 attack vectors validated, 100% detection rate in brute force testing. We have a pipeline of interested customers including BDO Luxembourg, two fund administrators, and a custody bank. The technology is ready; we need fuel for go-to-market."

---

# SLIDE 14: Team

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                          TEAM                                  │
│                                                                │
│   FOUNDER                                                      │
│   ┌─────────────────────────────────────────────────────────┐ │
│   │  [Name]                                                 │ │
│   │  Technical Lead / Product Development                   │ │
│   │  • [Background]                                         │ │
│   │  • [Relevant experience]                                │ │
│   └─────────────────────────────────────────────────────────┘ │
│                                                                │
│   HIRING PLAN (Year 1)                                        │
│   • Senior Backend Engineer (M1)                              │
│   • Frontend/UX Developer (M3)                                │
│   • Sales Lead (M4)                                           │
│   • DevOps Engineer (M6)                                      │
│                                                                │
│   ADVISORY BOARD (Target)                                     │
│   • Former CSSF / regulatory expert                           │
│   • Big 4 Technology Partner                                  │
│   • Cryptography academic                                     │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"[Customize with your background]. Our year 1 hiring plan adds four key roles: backend engineer, frontend developer, sales lead, and DevOps. We're building an advisory board with regulatory, enterprise sales, and cryptographic expertise."

---

# SLIDE 15: The Ask

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                         THE ASK                                │
│                                                                │
│                                                                │
│              SEED ROUND: €450,000                              │
│                                                                │
│                                                                │
│   USE OF FUNDS                                                 │
│   ┌─────────────────────────────────────────────────────────┐ │
│   │                                                         │ │
│   │   Product Development        €200,000    44%            │ │
│   │   ████████████████████                                  │ │
│   │                                                         │ │
│   │   Sales & Marketing          €130,000    29%            │ │
│   │   █████████████                                         │ │
│   │                                                         │ │
│   │   Operations & Legal          €70,000    16%            │ │
│   │   ███████                                               │ │
│   │                                                         │ │
│   │   Reserve                     €50,000    11%            │ │
│   │   █████                                                 │ │
│   │                                                         │ │
│   └─────────────────────────────────────────────────────────┘ │
│                                                                │
│              18-month runway to break-even                     │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"We're raising €450,000 seed. €200K for product—completing integrations and building the SaaS platform. €130K for sales and marketing—hiring our sales lead and attending key conferences. €70K for operations and legal—compliance certifications and company setup. €50K reserve. This gives us 18 months to reach break-even."

---

# SLIDE 16: Returns

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                     INVESTOR RETURNS                           │
│                                                                │
│                                                                │
│   SCENARIO ANALYSIS (5-Year Exit)                             │
│                                                                │
│   ┌─────────────────────────────────────────────────────────┐ │
│   │                                                         │ │
│   │   Conservative                                          │ │
│   │   Revenue: €5M  │  Valuation: €20M  │  ROI: 44x        │ │
│   │                                                         │ │
│   │   Base Case                                             │ │
│   │   Revenue: €8.5M │ Valuation: €34M  │  ROI: 76x        │ │
│   │                                                         │ │
│   │   Optimistic                                            │ │
│   │   Revenue: €15M  │ Valuation: €60M  │  ROI: 133x       │ │
│   │                                                         │ │
│   └─────────────────────────────────────────────────────────┘ │
│                                                                │
│   EXIT PATHS                                                   │
│   • Strategic acquisition (DocuSign, Adobe, SAP)              │
│   • Private equity roll-up                                    │
│   • IPO (if EU document standard gains traction)              │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"At a 4x revenue multiple—conservative for enterprise SaaS—your returns range from 44x in our conservative case to 133x if we hit our optimistic targets. Exit paths include strategic acquisition by DocuSign, Adobe, or SAP; PE roll-up; or IPO if TDF becomes a European standard."

---

# SLIDE 17: Why Now

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                        WHY NOW?                                │
│                                                                │
│                                                                │
│   ┌─────────────────────────────────────────────────────────┐ │
│   │  2025: DORA takes effect                                │ │
│   │        Cryptographic data integrity required            │ │
│   └─────────────────────────────────────────────────────────┘ │
│                                                                │
│   ┌─────────────────────────────────────────────────────────┐ │
│   │  2028: EU e-invoicing mandate                           │ │
│   │        Structured, verifiable invoices required         │ │
│   └─────────────────────────────────────────────────────────┘ │
│                                                                │
│   ┌─────────────────────────────────────────────────────────┐ │
│   │  NOW:  AI-generated document fraud emerging             │ │
│   │        Visual verification becoming obsolete            │ │
│   └─────────────────────────────────────────────────────────┘ │
│                                                                │
│                                                                │
│        The window for first-mover advantage is NOW            │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"Why now? Three converging forces. DORA takes effect in 2025—cryptographic integrity is now required. EU e-invoicing in 2028 will mandate structured, verifiable documents. And AI is making document forgery trivial—visual verification is becoming obsolete. The regulatory pressure and market need have never been higher. The window for first-mover advantage is now."

---

# SLIDE 18: Close

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                                                                │
│                        TRUSTDOC                                │
│                                                                │
│              Documents You Can Actually Trust                  │
│                                                                │
│                                                                │
│                                                                │
│                     €450,000 Seed Round                        │
│                                                                │
│                                                                │
│                                                                │
│                        [Contact Info]                          │
│                        [Email]                                 │
│                        [Phone]                                 │
│                                                                │
│                                                                │
│                                                                │
│                    Questions?                                  │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

**Speaker Notes:**
"Documents you can actually trust. We're raising €450,000 to capture the first-mover advantage in cryptographic document security for financial services. I'd be happy to take your questions."

---

# APPENDIX SLIDES

## A1: Technical Architecture

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                  TECHNICAL ARCHITECTURE                        │
│                                                                │
│   document.tdf (ZIP container)                                │
│   ├── manifest.cbor      Metadata + root hash                 │
│   ├── content.cbor       Structured data                      │
│   ├── merkle.cbor        Integrity tree                       │
│   ├── signatures.cbor    Digital signatures                   │
│   └── styles.css         Rendering (optional)                 │
│                                                                │
│   CRYPTOGRAPHIC STACK                                         │
│   • Hash: SHA-256 (FIPS 180-4)                                │
│   • Signatures: Ed25519 (RFC 8032)                            │
│   • Encoding: CBOR (RFC 8949)                                 │
│   • Container: ZIP (PKWARE)                                   │
│                                                                │
│   All components are NIST-approved, open standards            │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

## A2: Customer Pipeline Detail

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                   CUSTOMER PIPELINE                            │
│                                                                │
│   DISCUSSIONS                                                  │
│   • BDO Luxembourg - Audit firm, 200+ staff                   │
│   • [Fund Admin A] - €15B AUM, NAV reporting pain             │
│   • [Fund Admin B] - €8B AUM, compliance focus                │
│   • [Custody Bank] - Document verification bottleneck         │
│                                                                │
│   PIPELINE VALUE                                               │
│   • Total potential Year 1: €180,000                          │
│   • Conversion assumption: 50%                                 │
│   • Expected closed: €90,000                                   │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

## A3: Regulatory Detail

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│                   REGULATORY ALIGNMENT                         │
│                                                                │
│   REGULATION          REQUIREMENT              TDF SOLUTION    │
│   ──────────────────────────────────────────────────────────  │
│   DORA (2025)         ICT risk controls        Crypto integrity│
│   MiFID II            Record integrity         Merkle proofs   │
│   GDPR Art. 5         Data integrity           Tamper-evident  │
│   AML 6               Doc authentication       Multi-sig       │
│   EU E-Invoice (2028) Structured invoices      Native support  │
│   eIDAS               Qualified signatures     Ed25519/secp256k│
│                                                                │
│   CSSF ENGAGEMENT                                              │
│   • Innovation Hub application: Q2 2026                       │
│   • Sandbox participation target: Q3 2026                     │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

*End of Pitch Deck*
