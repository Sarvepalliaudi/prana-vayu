use crate::lifecycle::LifecycleState;
use crate::severity::Severity;
use crate::validation::*;
use serde::{Deserialize, Serialize};

/// Structured environmental research dossier output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResearchDossier {
    pub particulate_profile: String,
    pub immediate_protocol: Vec<String>,
    pub long_term_engineering: Vec<String>,
    pub statutory_memo: String,
}

/// Core domain representation of an air pollution incident record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncidentRecord {
    pub incident_id: String,
    pub timestamp: String,
    pub jurisdiction: String,
    pub zone: String,
    pub source: String,
    pub severity: Severity,
    pub notes: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<f64>,
    pub lifecycle_state: LifecycleState,
    pub analysis_provider: String,
    pub analysis_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub research_dossier: Option<ResearchDossier>,
}

/// DTO for creating a new incident from unvalidated inputs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateIncidentInput {
    pub jurisdiction: String,
    pub zone: String,
    pub source: String,
    pub severity: String,
    pub notes: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

impl CreateIncidentInput {
    /// Validates all input boundaries according to PRANA-VAYU rules.
    pub fn validate(&self) -> Result<Severity, ValidationError> {
        validate_jurisdiction(&self.jurisdiction)?;
        validate_zone(&self.zone)?;
        validate_source(&self.source)?;
        validate_notes(&self.notes)?;
        validate_coordinates(self.lat, self.lng)?;

        self.severity
            .parse::<Severity>()
            .map_err(|_| ValidationError::InvalidSource(format!("Invalid severity: {}", self.severity)))
    }

    /// Converts validated input into a complete IncidentRecord with initial LifecycleState::Reported.
    pub fn into_record(
        self,
        incident_id: String,
        timestamp: String,
        dossier: Option<ResearchDossier>,
        provider: String,
    ) -> Result<IncidentRecord, ValidationError> {
        let severity = self.validate()?;

        let status = if dossier.is_some() {
            "completed"
        } else {
            "pending"
        };

        let initial_state = if dossier.is_some() {
            LifecycleState::Assessed
        } else {
            LifecycleState::Reported
        };

        Ok(IncidentRecord {
            incident_id,
            timestamp,
            jurisdiction: self.jurisdiction.trim().to_string(),
            zone: self.zone.trim().to_string(),
            source: self.source.trim().to_string(),
            severity,
            notes: self.notes.trim().to_string(),
            lat: self.lat,
            lng: self.lng,
            lifecycle_state: initial_state,
            analysis_provider: provider,
            analysis_status: status.to_string(),
            research_dossier: dossier,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_incident_happy_path() {
        let input = CreateIncidentInput {
            jurisdiction: "India".to_string(),
            zone: "Hostel Block 3 Quadrangle".to_string(),
            source: "Biomass Burning".to_string(),
            severity: "Severe".to_string(),
            notes: "Leaves and dry plastic waste burning in ditch upwind of dorms".to_string(),
            lat: Some(13.0827),
            lng: Some(80.2707),
        };

        let record = input
            .into_record(
                "PV-2026-TEST".to_string(),
                "2026-10-11T12:00:00Z".to_string(),
                None,
                "deterministic_fallback".to_string(),
            )
            .expect("Should construct record successfully");

        assert_eq!(record.incident_id, "PV-2026-TEST");
        assert_eq!(record.severity, Severity::Severe);
        assert_eq!(record.lifecycle_state, LifecycleState::Reported);
        assert_eq!(record.analysis_status, "pending");
    }

    #[test]
    fn test_create_incident_with_dossier() {
        let input = CreateIncidentInput {
            jurisdiction: "India".to_string(),
            zone: "Engineering Gate A".to_string(),
            source: "Vehicle Idling".to_string(),
            severity: "High".to_string(),
            notes: "6 commercial transport buses idling with diesel exhaust".to_string(),
            lat: None,
            lng: None,
        };

        let dossier = ResearchDossier {
            particulate_profile: "Diesel Particulate Matter (DPM), Elemental Carbon".to_string(),
            immediate_protocol: vec![
                "Reroute arrival gate 150m upwind".to_string(),
                "Enforce immediate engine shutdown".to_string(),
            ],
            long_term_engineering: vec!["Install staggered electric charging bays".to_string()],
            statutory_memo: "Section 31A statutory notice".to_string(),
        };

        let record = input
            .into_record(
                "PV-2026-DIESEL".to_string(),
                "2026-10-11T12:30:00Z".to_string(),
                Some(dossier),
                "gemini-3.8-flash".to_string(),
            )
            .expect("Should construct record");

        assert_eq!(record.lifecycle_state, LifecycleState::Assessed);
        assert_eq!(record.analysis_provider, "gemini-3.8-flash");
        assert_eq!(record.analysis_status, "completed");
        assert!(record.research_dossier.is_some());
    }

    #[test]
    fn test_serialization_round_trip() {
        let input = CreateIncidentInput {
            jurisdiction: "US".to_string(),
            zone: "Warehouse Loading Bay".to_string(),
            source: "Silica Dust".to_string(),
            severity: "Moderate".to_string(),
            notes: "Dry cutting of concrete pavement without misting".to_string(),
            lat: Some(37.7749),
            lng: Some(-122.4194),
        };

        let record = input
            .into_record(
                "PV-2026-US".to_string(),
                "2026-10-11T13:00:00Z".to_string(),
                None,
                "deterministic_fallback".to_string(),
            )
            .unwrap();

        let json = serde_json::to_string(&record).expect("Must serialize");
        assert!(json.contains("PV-2026-US"));
        assert!(json.contains("Moderate"));

        let deserialized: IncidentRecord = serde_json::from_str(&json).expect("Must deserialize");
        assert_eq!(record, deserialized);
    }
}
