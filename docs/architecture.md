# PRANA-VAYU (प्राण-वायु) — System Architecture

**Developer:** SARVEPALLI AUDI SIVA BHANUVARDHAN  
**Brand:** AS PHENIX  
**Track:** Bharat Builds Environmental Hacks — Air

---

## 1. Multi-Language Layering Architecture

| Layer | Language / Technology | Directory | Core Responsibility |
|---|---|---|---|
| **Domain Engine & Rule Core** | **Rust 2021 (Cargo)** | `crates/prana-core/` | Zero-panic type safety, boundary validation, severity ranking, incident lifecycle state machine, Serde data normalization. |
| **Cloud Microservice & API** | **Python 3.12 (AWS Lambda)** | `services/api/` & `lambda_function.py` | API Gateway event routing, server-side payload sanitation, DynamoDB persistence, dual Bedrock/Gemini AI synthesis with deterministic fallback. |
| **Frontend Contracts & Service** | **TypeScript 5.4 (Vite)** | `apps/web/` | Typed API clients, interface schemas, strict contracts matching Rust and Python models. |
| **Operational Web Console** | **HTML5, Vanilla CSS3, ES2022** | `index.html` | High-contrast Anti-Vibe-Coding dashboard, Leaflet GIS with CartoDB Dark Matter tiles, real-time search & filters, Web Audio telemetry feedback, print engine. |
| **Infrastructure as Code** | **AWS SAM / CloudFormation** | `infrastructure/` & `template.yaml` | HTTP API Gateway v2, Python 3.12 Lambda, on-demand DynamoDB table, scoped IAM execution roles. |
| **CI/CD Hosting** | **AWS Amplify** | `amplify.yml` | Automatic branch deployment with Zero-Config static hosting. |

---

## 2. End-to-End Incident Lifecycle

```
[ Field Spotter / NSS / NCC ]
              │ (Coordinates, Source, Zone, Severity, Notes)
              ▼
   ┌─────────────────────────────────────────────────────────────┐
   │                  RUST DOMAIN ENGINE (prana-core)            │
   │  • Input Boundary Validation (Zone ≤180, Notes ≤3000)       │
   │  • Geographic Coordinate Symmetry [-90..90, -180..180]      │
   │  • Severity Ranking & Response Window Determination         │
   │  • Lifecycle State Transition Machine                       │
   └─────────────────────────────────────────────────────────────┘
              │
              ▼
   ┌─────────────────────────────────────────────────────────────┐
   │             AWS LAMBDA API GATEWAY (Python 3.12)            │
   │  • Explicit CORS & Request Bounding                         │
   │  • Server-Side Validation                                   │
   │  • AI Provider Routing:                                     │
   │      1. Google Gemini Interactions API (gemini-3.8-flash)   │
   │      2. Amazon Bedrock (Anthropic Claude 3.5 Sonnet)        │
   │      3. Deterministic Qualitative Fallback                  │
   └─────────────────────────────────────────────────────────────┘
              │
              ▼
   ┌─────────────────────────────────────────────────────────────┐
   │                PERSISTENCE & OPERATIONAL SYNC               │
   │  • Amazon DynamoDB (On-Demand, Encrypted, PITR)             │
   │  • Local Storage Resilience with Export / Import            │
   └─────────────────────────────────────────────────────────────┘
              │
              ▼
   ┌─────────────────────────────────────────────────────────────┐
   │                 OPERATIONAL DASHBOARD                       │
   │  • Interactive Leaflet Map with Micro-Plume Radii           │
   │  • Search & Multi-Filter Registry                           │
   │  • Environmental Dossier & Chemical Speciation              │
   │  • Formal Printable Legal Notice (Section 19(5) Air Act)    │
   └─────────────────────────────────────────────────────────────┘
```

---

## 3. Strict Environmental Data Integrity Principles

1. **Zero Fabricated Sensor Readings:**  
   Unlike superficial mock platforms, PRANA-VAYU never invents fake numerical readings (e.g. `PM2.5: 412 µg/m³`). Field spotter inputs are visual and qualitative observations. If no certified reference monitor is attached, numerical fields are strictly `null`.
2. **Defensive Volunteer Safety:**  
   Volunteers never approach fires, enter hazardous smoke corridors, or confront alleged violators. The system prescribes physical detours and institutional administrative escalation only.
3. **Draft Legal Representations:**  
   Generated regulatory notices under India's *Air (Prevention and Control of Pollution) Act, 1981* are explicitly watermarked as drafts for authorized administrative sign-off before statutory transmission.
