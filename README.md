# PRANA-VAYU (प्राण-वायु)
### Hyper-Local Micro-Climate Environmental Operations Console
**Built for Bharat Builds Environmental Hacks — Air Track**

Repository: [https://github.com/Sarvepalliaudi/prana-vayu.git](https://github.com/Sarvepalliaudi/prana-vayu.git)

---

## 1. Philosophical Foundation: The Pancha Bhootas & Acute Biological Criticality
Rooted in the ancient Vedic cosmology of the **Pancha Bhootas** (*Five Primordial Elements*):
- **Prithvi (पृथ्वी - Earth / Particulate Sink):** Soil and vegetation buffer capacity.
- **Jala (जल - Water / Suppression):** Aqueous dousing to extinguish thermal lofting.
- **Agni (अग्नि - Fire / Micro-Combustion):** Point-source biomass, diesel, and municipal combustion plumes.
- **Vayu (वायु - Air / Carrier of Prana):** The non-negotiable life-force transport.
- **Akasha (आकाश - Space / Boundary Layer):** Thermal inversion ceiling encapsulating pollutants.

### The Biological Criticality Law
A human organism survives:
- **Weeks without Prithvi (food)**
- **Days without Jala (water)**
- **Hours without Agni (warmth/shelter)**
- **Under 3 minutes without clean Vayu** before irreversible biological death and neurological damage occur.

Therefore, localized air pollution cannot be treated as a passive, monthly statistical average. It must be recognized as an **acute, life-critical emergency violation**.

---

## 2. The Problem We Solve
Existing government monitoring stations (**CAAQMS**) and tracking apps like **SAMEER** track macro-city averages 15 km away, completely missing localized, micro-climate plumes where students and citizens actually breathe:
- **Campus perimeter leaf and biomass burning** by groundskeepers.
- **Morning transit bus idling at school gates** directly into pupil respiratory corridors.
- **Unmitigated construction silica dust and road cutting** without mandatory water mists.

Volunteers (**NSS / NCC cadets**) currently perform manual cleanup because they lack telemetry tools to quantify exposure or enforce regulatory compliance. **PRANA-VAYU bridges this gap.**

---

## 3. The Three-Tier Operational Model

```
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
 • Pedestrian Density          • Upwind Pedestrian Detours     Air Act Notice
 • Ringelmann Opacity          • Indoor Exertion Shifts      • Sec 37/38 Penal
 • AIDEI Dose Metric           • Phytoremediation Matrix       Injunction
```

### Tier 1: Field Observers (NSS/NCC Cadets & Student Spotters)
- Hyper-local incident logging with GPS pinning and wind bearing vector.
- Micro-source categorization (Biomass Smoldering, Diesel Transit Idling, Construction Silica Dust, DG Set Exhaust, MSW Fire).
- Pedestrian and student density at risk.
- **Acute Inhaled Dose Exposure Index (AIDEI)** calculation contrasting hyper-local micro-AQI (e.g., 680 µg/m³) against the complacent macro CAAQMS reading (e.g., 142 AQI).

### Tier 2: Institutional Operators (Campus Registrars, Principals, Hostel Wardens)
- **24-Hour Immediate Physical Countermeasures Protocol:**
  1. *Direct Aqueous Hose Suppression:* 15-minute protocol to quench smoldering biomass and halt thermal plume lofting.
  2. *Pedestrian Corridor Detour Grid:* Physical barrier cordon 120m upwind to reroute student traffic.
  3. *Indoor Physical Training Shifts:* Cease outdoor morning parade/sports drills (reducing biological ventilation rate from 18 L/min to 6 L/min).
  4. *HVAC Positive Pressure Throttling:* Seal ground-floor fresh air intakes on plume-facing building facades.
- **Botanical Engineering & Phytoremediation Architecture:**
  - High Air Pollution Tolerance Index (**APTI**) native species:
    - **Peepal (*Ficus religiosa*)**: APTI 22.4 | Particulate capture 14.2 g/m² leaf area.
    - **Neem (*Azadirachta indica*)**: APTI 24.8 | Diesel soot and NO2 tolerant.
    - **Kaner (*Nerium oleander*)**: APTI 21.0 | Low-altitude roadside bus exhaust interceptor.
    - **Bougainvillea (*Bougainvillea spectabilis*)**: Road silica barrier.
  - Multi-tier canopy buffer yielding up to 46.8% micro-PM reduction.

### Tier 3: Statutory Regulators (Municipal Ward Commissioners & State PCBs)
- Automated, legally binding **Statutory Abatement Notice**:
  - Direct citation of **Section 19(5)** of India's *Air (Prevention and Control of Pollution) Act, 1981* (prohibiting open burning of pollution-causing materials).
  - Enforcement directives under **Section 31A** and penal sanctions under **Section 37 & 38** (18 months to 6 years imprisonment).
  - Judicial grounding under **Article 21 of the Constitution of India** (*Subhash Kumar v. State of Bihar, 1991* Right to Clean Air).
  - Harmonised with international benchmarks: **US Clean Air Act (42 U.S.C. § 7401)**, **UK Environment Act 2021**, **EU Directive 2008/50/EC**, and **WHO 2021 Air Quality Guidelines**.
  - Browser-native print engine rendering formatted legal notices on official double-bordered letterhead with evidence annexure and verification stamp.

---

## 4. Technical Architecture & AWS Stack

- **Frontend:** Single-file `index.html` built with high-contrast operational design (*Anti-Vibe-Coding*), Leaflet.js with dark CartoDB tiles, directional Gaussian plume dispersion overlay, Web Audio tactical synthesizer, and print-ready CSS (`@media print`).
- **Backend (Cloud Ready):**
  - **AWS Amplify Hosting:** Automatic CI/CD linked to GitHub repository.
  - **Amazon API Gateway & AWS Lambda:** Plume ingestion and Gaussian dispersion modeling.
  - **Amazon Bedrock (Anthropic Claude 3.5 Sonnet):** Particulate chemical speciation (PAHs, Benzo[a]pyrene, Black Carbon) and dynamic legal memorandum synthesis.
  - **Amazon DynamoDB (On-Demand):** Telemetry persistence in `PranaVayu_Incidents` (`ORG#BHARAT#CAMPUS` partition key, timestamp sort key).

---

## 5. Deployment Instructions (AWS Amplify)

1. **Initialize Git & Push to GitHub:**
   ```bash
   git init
   git add .
   git commit -m "feat: initial commit of PRANA-VAYU operational console"
   git branch -M main
   git remote add origin https://github.com/Sarvepalliaudi/prana-vayu.git
   git push -u origin main
   ```

2. **Connect to AWS Amplify:**
   - Log in to the AWS Management Console.
   - Open **AWS Amplify Hosting** -> **Deploy an app** -> Select **GitHub**.
   - Choose repository: `Sarvepalliaudi/prana-vayu` and branch: `main`.
   - Build settings: Since PRANA-VAYU is a single-file self-contained web console, Amplify will automatically serve `index.html` as a static web application without any build commands.
   - Click **Save and Deploy**. Your live URL will be provisioned in under 60 seconds!

---

*PRANA-VAYU — Protecting the Sacred Breath through Vedic Science, Ground Telemetry, and Statutory Law.*
