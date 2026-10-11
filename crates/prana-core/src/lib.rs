//! # PRANA-CORE
//!
//! High-assurance domain model, input validation, and incident lifecycle state machine
//! for PRANA-VAYU (Air-Pollution Incident Response Platform).
//!
//! Developer: SARVEPALLI AUDI SIVA BHANUVARDHAN
//! Brand: AS PHENIX
//! Track: Environmental Hacks — Air

pub mod incident;
pub mod lifecycle;
pub mod severity;
pub mod validation;

pub use incident::{CreateIncidentInput, IncidentRecord, ResearchDossier};
pub use lifecycle::{LifecycleError, LifecycleState};
pub use severity::{Severity, SeverityError};
pub use validation::ValidationError;

/// Top-level validator and constructor for incident ingestion.
pub fn validate_and_create_record(
    input: CreateIncidentInput,
    incident_id: String,
    timestamp: String,
    dossier: Option<ResearchDossier>,
    provider: String,
) -> Result<IncidentRecord, ValidationError> {
    input.into_record(incident_id, timestamp, dossier, provider)
}
