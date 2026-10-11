# PRANA-VAYU (प्राण-वायु)
### Hyper-Local Micro-Climate Air Pollution Incident Response & Environmental Operations Console
**Built for Bharat Builds Environmental Hacks — Air Track**  
**Repository:** [https://github.com/Sarvepalliaudi/prana-vayu.git](https://github.com/Sarvepalliaudi/prana-vayu.git)  
**Developer:** SARVEPALLI AUDI SIVA BHANUVARDHAN  
**Brand:** AS PHENIX  
**Profiles & Links:** [LinkedIn](https://www.linkedin.com/in/audi-siva-bhanuvardhan-sarvepalli-4598a8289/) · [GitHub](https://github.com/Sarvepalliaudi/) · [Portfolio](https://sarvepalliaudi.github.io/asphenixnewprotofolio/) · [AS PHENIX Space](https://asphenix-rrq5sx5.gamma.site/)

---

## 1. Philosophical Foundation: The Pancha Bhootas & Acute Biological Criticality

PRANA-VAYU is rooted in the Vedic cosmology of the **Pancha Bhootas** (*Five Primordial Elements*):
- **Prithvi (पृथ्वी - Earth / Particulate Sink):** Soil and vegetation buffer capacity.
- **Jala (जल - Water / Aqueous Suppression):** Liquid mist dousing to quench smoldering embers and halt thermal plume lofting.
- **Agni (अग्नि - Fire / Micro-Combustion Plumes):** Point-source biomass, transit diesel, and municipal combustion emissions.
- **Vayu (वायु - Air / Carrier of Prana):** The non-negotiable life-force transport medium.
- **Akasha (आकाश - Space / Boundary Layer):** Thermal inversion ceiling encapsulating toxic localized plumes.

### The Biological Criticality Law
A human organism survives:
- **Weeks without Prithvi (food)**
- **Days without Jala (water)**
- **Hours without Agni (warmth/shelter)**
- **Under 3 minutes without clean Vayu** before irreversible biological death and cellular neurological damage occur.

Therefore, localized air pollution cannot be dismissed as a passive, monthly macro statistical average. It must be triaged, mitigated, and represented as an **acute, life-critical violation**.

---

## 2. The Problem We Solve: The 15-Kilometer Blind Spot

Current government monitoring networks (**CAAQMS**) and tracking apps like **SAMEER** track macro-city averages up to 15 km away, completely missing the hyper-local micro-climate plumes where students, patients, and citizens actually breathe:
1. **Campus perimeter leaf & green waste burning** by groundskeepers under morning inversion layers.
2. **Morning transit bus idling at school gates** directly discharging carcinogenic diesel soot into pupil respiratory corridors.
3. **Unmitigated construction silica dust & road cutting** without mandatory water suppression mists.

Volunteers (**NSS / NCC cadets**) currently perform manual cleanup work without telemetry or legal tools to quantify exposure or trigger administrative interventions. **PRANA-VAYU bridges this operational divide.**

---

## 3. Multi-Language Engineering Architecture

PRANA-VAYU adopts a high-assurance, multi-language architecture where each programming language handles its specialized operational domain:

```text
prana-vayu/
├── apps/
│   └── web/                     # TypeScript / Vite operational frontend contracts
│       ├── src/
│       │   ├── services/api.ts  # Typed AWS API Gateway & Lambda client
│       │   └── types/incident.ts# Strict TypeScript interface contracts
│       ├── package.json
│       └── vite.config.ts
├── crates/
│   └── prana-core/              # Rust domain model & validation engine
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs           # Top-level API exports
│           ├── incident.rs      # IncidentRecord & ResearchDossier schemas
│           ├── lifecycle.rs     # Strict state machine (Reported -> Resolved)
│           ├── severity.rs      # Severity ranking & response window determination
│           └── validation.rs    # Geographic coordinate symmetry & boundary checks
├── services/
│   └── api/                     # Python 3.12 AWS Lambda microservice
│       ├── lambda_function.py   # REST API handler, DynamoDB persistence, Dual AI engine
│       └── tests/
│           └── test_lambda.py   # 10 comprehensive unit & mock event tests
├── infrastructure/
│   └── template.yaml            # AWS SAM template (HTTP API v2, Lambda, DynamoDB)
├── docs/
│   └── architecture.md          # In-depth architectural & security specification
├── index.html                   # High-contrast, zero-dependency command console (Web Audio + Leaflet)
├── lambda_function.py           # Root Lambda handler for SAM CodeUri flexibility
├── test_lambda.py               # Root unit test runner
├── template.yaml                # Root AWS SAM template
├── amplify.yml                  # AWS Amplify Hosting CI/CD definition
├── Cargo.toml                   # Root Rust workspace definition
└── README.md
```

| Component | Language | Role | Key Properties |
|---|---|---|---|
| **Domain Engine** | **Rust 2021** | Type safety, input boundary sanitization, incident lifecycle state machine, Serde normalization. | Zero-panic, explicit `thiserror` types, comprehensive unit test suite in `crates/prana-core`. |
| **Backend Microservice** | **Python 3.12** | AWS Lambda event handling, DynamoDB on-demand storage, Google Gemini & Amazon Bedrock AI routing. | Least-privilege IAM, explicit CORS, fails closed with HTTP 503, defensive fallbacks. |
| **Type Contracts** | **TypeScript 5.4** | Typed API contracts matching Rust and Python models. | Strict schemas for `IncidentRecord`, `ResearchDossier`, and `PollutionSource`. |
| **Operational Console** | **HTML5 / CSS3 / ES2022** | Responsive command dashboard with Leaflet GIS, CartoDB Dark Matter tiles, Web Audio synthesis, and regulatory letterhead print engine. | 100% responsive across Mobile, Tablet, Laptop, and 4K Ultra-Wide displays. |
| **Cloud Infrastructure** | **AWS SAM (YAML)** | Serverless HTTP API Gateway v2, Python 3.12 Lambda, encrypted DynamoDB table with PITR. | Pay-per-request billing, zero idle cost. |

---

## 4. The Three-Tier Operational Model

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                       PRANA-VAYU OPERATIONAL ARCHITECTURE                  │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
       ┌──────────────────────────────┼─────────────────────────────┐
       ▼                              ▼                             ▼
┌──────────────┐              ┌──────────────┐              ┌──────────────┐
│    TIER 1    │              │    TIER 2    │              │    TIER 3    │
│Field Spotter │              │Institutional │              │  Statutory   │
│  (NSS / NCC) │              │  Operators   │              │  Regulators  │
└──────┬───────┘              └──────┬───────┘              └──────┬───────┘
       │                             │                             │
 • GPS Plume Pinning           • 15-Min Hose Dousing         • Section 19(5)
 • Pedestrian Cordon           • Upwind Transit Detours        Air Act Notice
 • Dual Lat/Lng Bounds         • Indoor Exertion Shifts      • Sec 31A / 37 Penal
 • Null Sensor Policy          • Botanical APTI Buffers        Sanction Review
```

### Tier 1: Field Observers (NSS/NCC Cadets & Student Spotters)
- Hyper-local incident reporting with GPS coordinate acquisition or interactive map pinning.
- Source tagging: *Biomass Burning*, *Vehicle Idling*, *Silica Dust*, *Kitchen Exhaust*, *Perimeter Waste Combustion*.
- Bounded notes (3–3000 chars) capturing smoke drift, nearby hostels, and school corridors.
- Non-confrontational safety rule: Field spotters **never approach fires or confront operators**.

### Tier 2: Institutional Operators (Campus Registrars, Principals, Wardens)
- **24-Hour Immediate Physical Countermeasures Protocol:**
  1. *Aqueous Mist Dousing:* 15-minute low-pressure water hose dousing to extinguish smoldering leaves and stop thermal lofting.
  2. *Upwind Transit Detour:* 120-meter barrier cordon rerouting student foot traffic away from downwind plume corridors.
  3. *Ventilation Isolation:* Throttle ground-floor HVAC fresh air dampers on plume-facing building facades.
- **Botanical Phytoremediation Architecture:**
  - High Air Pollution Tolerance Index (**APTI**) native species:
    - **Peepal (*Ficus religiosa*)**: APTI 22.4 | Particulate capture 14.2 g/m² leaf area.
    - **Neem (*Azadirachta indica*)**: APTI 24.8 | Diesel soot and NO2 tolerant.
    - **Kaner (*Nerium oleander*)**: APTI 21.0 | Low-altitude roadside bus exhaust interceptor.
    - **Bougainvillea (*Bougainvillea spectabilis*)**: Coarse road silica dust barrier.

### Tier 3: Statutory Regulators (Municipal Ward Commissioners & State PCBs)
- Automated, high-contrast **Statutory Abatement Notice**:
  - Grounded in **Section 19(5)** of India's *Air (Prevention and Control of Pollution) Act, 1981* (prohibiting open burning of pollution-causing materials).
  - Enforcement directives under **Section 31A** and penal sanctions under **Sections 37 & 38**.
  - Constitutional foundation under **Article 21 of the Constitution of India** (*Subhash Kumar v. State of Bihar, 1991* Right to Clean Air).
  - Browser-native letterhead print engine with official draft stamp, dual sign-off lines, and evidence annexure.

---

## 5. Strict Environmental Data Integrity Principles

1. **Zero Fabricated Sensor Readings:**  
   Unlike superficial mock applications that invent arbitrary numbers (e.g., `PM2.5: 412 µg/m³`), PRANA-VAYU explicitly flags all sensor fields (`PM2.5`, `PM10`, `NOx`, `VOCs`) as **`NULL / UNMEASURED`** unless an accredited physical optical particle counter is integrated.
2. **Defensive Volunteer Safety:**  
   Under no circumstances does the system advise volunteers or student spotters to confront suspected polluters or enter hazardous smoke.
3. **Draft Legal Representations:**  
   Generated statutory documents are watermarked as **"DRAFT FOR ADMINISTRATIVE REVIEW — NOT AN ENFORCEMENT ORDER"** to maintain legal integrity until signed by authorized institutional officials.

---

## 6. Real Dual AI Engine Backend

The Python Lambda microservice features a resilient dual-provider architecture:
1. **Google Gemini Interactions API (`gemini-3.8-flash`):** Connected via `https://generativelanguage.googleapis.com/v1beta/interactions` (`Api-Revision: 2026-05-20`).
2. **Amazon Bedrock (`anthropic.claude-3-5-sonnet-20240620-v1:0`):** Invoked via `bedrock-runtime` with scoped IAM permissions.
3. **Deterministic Qualitative Fallback:** If both cloud AI providers are unavailable, offline, or unconfigured, the system immediately executes a deterministic rule-based generator that returns schema-compliant particulate speciation and legal routing memos without failing the request.

---

## 7. Local Testing & Verification Suite

### Python Backend Unit Tests
Execute the comprehensive 10-test suite locally (runs with zero external network calls):
```bash
python test_lambda.py
# or:
python services/api/tests/test_lambda.py
```
**Test Coverage:**
- `test_options_request`: Validates 200 OK and explicit CORS headers.
- `test_method_not_allowed`: Rejects unsupported HTTP verbs with 405.
- `test_validation_malformed_json`: Rejects invalid JSON syntax with 400.
- `test_validation_invalid_jurisdiction`: Enforces allowed jurisdiction enumeration with 400.
- `test_validation_coordinates_asymmetry`: Fails closed when latitude is provided without longitude.
- `test_validation_coordinates_out_of_range`: Rejects coordinates outside `[-90, 90]` or `[-180, 180]`.
- `test_post_success_with_deterministic_fallback`: Persists records to DynamoDB when AI models fail.
- `test_gemini_interactions_api_success`: Verifies correct parsing of Gemini model output.
- `test_get_incidents`: Verifies DynamoDB scanning, limiting to 50 items, and descending timestamp sort.
- `test_dynamodb_error_fails_closed`: Returns 503 without leaking stack traces or internal secrets.

### Rust Domain Core Tests
```bash
cargo test --manifest-path crates/prana-core/Cargo.toml
```

---

## 8. Deployment Guide

### A. AWS SAM Cloud Backend Deployment
1. Build the serverless application:
   ```bash
   sam build
   ```
2. Deploy to AWS:
   ```bash
   sam deploy --guided
   ```
   *Stack Name:* `prana-vayu-backend`  
   *AWS Region:* `us-east-1` (or your preferred region)  
   *BedrockModelId:* `anthropic.claude-3-5-sonnet-20240620-v1:0`

3. Copy the output `PranaVayuApiUrl` and paste it into the **AWS API Gateway endpoint** bar in `index.html`.

### B. AWS Amplify Static Hosting
1. Commit and push the repository to GitHub:
   ```bash
   git add .
   git commit -m "feat: complete PRANA-VAYU multi-language platform"
   git push -u origin main
   ```
2. Log in to the **AWS Management Console** -> **AWS Amplify Hosting**.
3. Select **Deploy an app** -> **GitHub** -> `Sarvepalliaudi/prana-vayu` (`main` branch).
4. Amplify uses the root `amplify.yml` to automatically host `index.html` on a global CDN with SSL enabled in under 60 seconds!

---

*PRANA-VAYU (प्राण-वायु) — Protecting the Sacred Breath through Vedic Science, Ground Telemetry, and Statutory Law.*  
*Brand: AS PHENIX · Lead Architect: SARVEPALLI AUDI SIVA BHANUVARDHAN*
