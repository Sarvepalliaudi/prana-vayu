//! PRANA-VAYU Systems Server & Incident Triage Binary
//!
//! Provides a zero-dependency HTTP/1.1 microserver exposing the pure Rust domain core
//! (`prana-core`) for incident validation, risk triaging, and lifecycle state transitions.
//!
//! Developer: SARVEPALLI AUDI SIVA BHANUVARDHAN
//! Brand: AS PHENIX

use prana_core::incident::{CreateIncidentInput, IncidentRecord, ResearchDossier};
use prana_core::lifecycle::LifecycleState;
use prana_core::severity::Severity;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
struct HealthResponse<'a> {
    status: &'a str,
    engine: &'a str,
    version: &'a str,
    author: &'a str,
    brand: &'a str,
    uptime_seconds: u64,
    incidents_count: usize,
}

#[derive(Deserialize)]
struct TransitionRequest {
    incident_id: String,
    target_state: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

struct ServerState {
    start_time: SystemTime,
    incidents: Vec<IncidentRecord>,
}

fn seed_incidents() -> Vec<IncidentRecord> {
    vec![
        IncidentRecord {
            incident_id: "PV-RUST-001".to_string(),
            timestamp: "2026-10-11T12:00:00Z".to_string(),
            jurisdiction: "India".to_string(),
            zone: "Samayapuram University Perimeter, Gate 4".to_string(),
            source: "Biomass burning".to_string(),
            severity: Severity::Severe,
            notes: "Dense leaf and mixed plastic smoldering in stormwater ditch upwind of student hostel block B intake.".to_string(),
            lat: Some(10.9205),
            lng: Some(78.7410),
            lifecycle_state: LifecycleState::ResponseActive,
            analysis_provider: "prana-core-native-rust".to_string(),
            analysis_status: "completed".to_string(),
            research_dossier: Some(ResearchDossier {
                particulate_profile: "Combustion products include inhalable fine particles, carbon monoxide, and volatile organic vapors. No concentration measured by this platform.".to_string(),
                immediate_protocol: vec![
                    "Maintain safe distance; deploy 15-minute low pressure water mist dousing if safe.".to_string(),
                    "Reroute pedestrians 120m upwind.".to_string(),
                ],
                long_term_engineering: vec![
                    "Evaluate composting solutions and plant high-APTI native vegetative buffers (Peepal, Neem, Kaner).".to_string(),
                ],
                statutory_memo: "Section 19(5) Air (Prevention and Control of Pollution) Act, 1981 prohibits unauthorized open burning in air pollution control areas.".to_string(),
            }),
        },
        IncidentRecord {
            incident_id: "PV-RUST-002".to_string(),
            timestamp: "2026-10-11T11:15:00Z".to_string(),
            jurisdiction: "India".to_string(),
            zone: "North Bus Terminal Departure Bay".to_string(),
            source: "Vehicle idling".to_string(),
            severity: Severity::High,
            notes: "Four inter-campus diesel buses idling consecutively with tailpipes aimed at sheltered morning student boarding queues.".to_string(),
            lat: Some(10.9250),
            lng: Some(78.7450),
            lifecycle_state: LifecycleState::Validated,
            analysis_provider: "prana-core-native-rust".to_string(),
            analysis_status: "completed".to_string(),
            research_dossier: Some(ResearchDossier {
                particulate_profile: "Diesel particulate matter (DPM) consisting of elemental carbon cores with adsorbed polycyclic aromatic hydrocarbons (PAHs).".to_string(),
                immediate_protocol: vec![
                    "Mandate immediate engine cut-off for transit vehicles halted over 90 seconds.".to_string(),
                    "Shift passenger boarding queue 45m upwind.".to_string(),
                ],
                long_term_engineering: vec![
                    "Install shore-power electrical hookups for auxiliary air conditioning in transit bays.".to_string(),
                ],
                statutory_memo: "Central Motor Vehicles Rules rule 115 emission standards and local anti-idling municipal bylaws apply.".to_string(),
            }),
        },
    ]
}

fn build_dossier(source: &str, severity: &Severity) -> ResearchDossier {
    let (profile, protocols, engineering, statutory) = match source {
        "Biomass burning" => (
            "Combustion products include inhalable fine particles, carbon monoxide, and volatile organic vapors. No concentration measured by this platform.",
            vec![
                "Maintain safe distance; deploy 15-minute low pressure water mist dousing if safe.".to_string(),
                "Reroute pedestrians 120m upwind.".to_string(),
            ],
            vec![
                "Evaluate composting solutions and plant high-APTI native vegetative buffers (Peepal, Neem, Kaner).".to_string(),
            ],
            "Section 19(5) Air (Prevention and Control of Pollution) Act, 1981 prohibits unauthorized open burning in air pollution control areas."
        ),
        "Vehicle idling" => (
            "Diesel particulate matter (DPM) and unburned hydrocarbon aerosols. Exhaust contains respirable ultrafine soot.",
            vec![
                "Enforce 90-second anti-idling cutoff for queued transit fleets.".to_string(),
                "Position queue marshals upwind of active coach bays.".to_string(),
            ],
            vec![
                "Erect physical acoustic and vegetative particulate baffles along boarding platforms.".to_string(),
            ],
            "Central Motor Vehicles Rules (CMVR) Rule 115 emission norms apply."
        ),
        "Silica dust" => (
            "Respirable crystalline silica (SiO2) micro-particles generated through dry abrasive masonry and pavement cutting.",
            vec![
                "Cease dry cutting immediately; mandate continuous water-mist shroud operation.".to_string(),
                "Distribute N95/FFP2 respirators to all workers within 25m perimeter.".to_string(),
            ],
            vec![
                "Procure vacuum-assisted HEPA dust extraction tools for all subsequent masonry work.".to_string(),
            ],
            "Construction & Demolition Waste Management Rules, 2016 and BOCW Act safety regulations."
        ),
        _ => (
            "Unspeciated particulate and combustion aerosol emissions. No calibrated numerical metric measured.",
            vec![
                "Maintain 60m upwind safety separation perimeter.".to_string(),
                "Notify site environmental supervisor.".to_string(),
            ],
            vec![
                "Investigate root cause and install localized capture ventilation.".to_string(),
            ],
            "Refer to applicable municipal air quality bylaws and national ambient air quality guidelines."
        )
    };

    ResearchDossier {
        particulate_profile: profile.to_string(),
        immediate_protocol: protocols,
        long_term_engineering: engineering,
        statutory_memo: format!("{} (Severity: {:?})", statutory, severity),
    }
}

fn handle_client(mut stream: TcpStream, state: Arc<Mutex<ServerState>>) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();

    if reader.read_line(&mut request_line).is_err() || request_line.is_empty() {
        return;
    }

    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }

    let method = parts[0];
    let path = parts[1];

    let mut content_length = 0usize;
    let mut line = String::new();
    while reader.read_line(&mut line).is_ok() && line != "\r\n" && line != "\n" && !line.is_empty() {
        let lower = line.to_lowercase();
        if lower.starts_with("content-length:") {
            if let Some(val) = line.split(':').nth(1) {
                content_length = val.trim().parse::<usize>().unwrap_or(0);
            }
        }
        line.clear();
    }

    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        let _ = reader.read_exact(&mut body);
    }

    // CORS Preflight
    if method == "OPTIONS" {
        let resp = "HTTP/1.1 204 No Content\r\n\
                    Access-Control-Allow-Origin: *\r\n\
                    Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
                    Access-Control-Allow-Headers: Content-Type, Accept, Authorization\r\n\
                    Content-Length: 0\r\n\r\n";
        let _ = stream.write_all(resp.as_bytes());
        return;
    }

    let (status_code, status_text, body_json) = match (method, path) {
        ("GET", "/health") => {
            let s = state.lock().unwrap();
            let uptime = s.start_time.elapsed().map(|d| d.as_secs()).unwrap_or(0);
            let resp = HealthResponse {
                status: "ok",
                engine: "prana-core-rust",
                version: env!("CARGO_PKG_VERSION"),
                author: "SARVEPALLI AUDI SIVA BHANUVARDHAN",
                brand: "AS PHENIX",
                uptime_seconds: uptime,
                incidents_count: s.incidents.len(),
            };
            (200, "OK", serde_json::to_string(&resp).unwrap())
        }

        ("GET", "/incidents") => {
            let s = state.lock().unwrap();
            (200, "OK", serde_json::to_string(&s.incidents).unwrap())
        }

        ("POST", "/incidents") => {
            match serde_json::from_slice::<CreateIncidentInput>(&body) {
                Ok(input) => {
                    match input.validate() {
                        Ok(severity) => {
                            let dossier = build_dossier(&input.source, &severity);
                            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                            let incident_id = format!("PV-RUST-{:X}", now);
                            let timestamp = format!("{}", now);

                            match input.into_record(
                                incident_id,
                                timestamp,
                                Some(dossier),
                                "prana-core-native-rust".to_string(),
                            ) {
                                Ok(record) => {
                                    let mut s = state.lock().unwrap();
                                    s.incidents.insert(0, record.clone());
                                    (201, "Created", serde_json::to_string(&record).unwrap())
                                }
                                Err(e) => (400, "Bad Request", serde_json::to_string(&ErrorResponse { error: e.to_string() }).unwrap())
                            }
                        }
                        Err(e) => (400, "Bad Request", serde_json::to_string(&ErrorResponse { error: e.to_string() }).unwrap())
                    }
                }
                Err(e) => (400, "Bad Request", serde_json::to_string(&ErrorResponse { error: format!("Malformed JSON: {}", e) }).unwrap())
            }
        }

        ("POST", "/incidents/transition") => {
            match serde_json::from_slice::<TransitionRequest>(&body) {
                Ok(req) => {
                    let mut s = state.lock().unwrap();
                    if let Some(pos) = s.incidents.iter().position(|x| x.incident_id == req.incident_id) {
                        let target: Result<LifecycleState, _> = req.target_state.parse();
                        match target {
                            Ok(target_state) => {
                                match s.incidents[pos].lifecycle_state.transition(target_state) {
                                    Ok(()) => {
                                        (200, "OK", serde_json::to_string(&s.incidents[pos]).unwrap())
                                    }
                                    Err(e) => (422, "Unprocessable Entity", serde_json::to_string(&ErrorResponse { error: e.to_string() }).unwrap())
                                }
                            }
                            Err(_) => (400, "Bad Request", serde_json::to_string(&ErrorResponse { error: format!("Unknown target state: {}", req.target_state) }).unwrap())
                        }
                    } else {
                        (404, "Not Found", serde_json::to_string(&ErrorResponse { error: "Incident not found".to_string() }).unwrap())
                    }
                }
                Err(e) => (400, "Bad Request", serde_json::to_string(&ErrorResponse { error: format!("Malformed JSON: {}", e) }).unwrap())
            }
        }

        _ => (404, "Not Found", serde_json::to_string(&ErrorResponse { error: format!("Endpoint not found: {} {}", method, path) }).unwrap())
    };

    let response = format!(
        "HTTP/1.1 {} {}\r\n\
        Content-Type: application/json; charset=utf-8\r\n\
        Access-Control-Allow-Origin: *\r\n\
        Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
        Access-Control-Allow-Headers: Content-Type, Accept, Authorization\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n{}",
        status_code,
        status_text,
        body_json.len(),
        body_json
    );

    let _ = stream.write_all(response.as_bytes());
}

fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("127.0.0.1:{}", port);

    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[FATAL] Could not bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    };

    let state = Arc::new(Mutex::new(ServerState {
        start_time: SystemTime::now(),
        incidents: seed_incidents(),
    }));

    println!("==================================================================");
    println!(" PRANA-VAYU (प्राण-वायु) — High-Assurance Rust Systems Engine");
    println!(" Brand: AS PHENIX | Developer: SARVEPALLI AUDI SIVA BHANUVARDHAN");
    println!(" Status: ACTIVE | Bound to http://{}", addr);
    println!(" REST Endpoints:");
    println!("   - GET  /health               (Server status & engine invariants)");
    println!("   - GET  /incidents            (List triaged incident records)");
    println!("   - POST /incidents            (Validate input & execute triage)");
    println!("   - POST /incidents/transition (FSM lifecycle state transition)");
    println!("==================================================================");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state_clone = Arc::clone(&state);
                thread::spawn(move || {
                    handle_client(stream, state_clone);
                });
            }
            Err(e) => {
                eprintln!("[WARN] Connection failed: {}", e);
            }
        }
    }
}
