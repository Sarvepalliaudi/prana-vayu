use prana_core::incident::{CreateIncidentInput, ResearchDossier};
use prana_core::lifecycle::{LifecycleError, LifecycleState};
use prana_core::severity::Severity;
use prana_core::validation::ValidationError;

#[test]
fn test_end_to_end_incident_intake_workflow() {
    let input = CreateIncidentInput {
        jurisdiction: "India".to_string(),
        zone: "Campus Gate 4 Perimeter".to_string(),
        source: "Biomass burning".to_string(),
        severity: "Severe".to_string(),
        notes: "Smoldering leaf litter trapped under inversion.".to_string(),
        lat: Some(10.9205),
        lng: Some(78.7410),
    };

    // 1. Validation
    let severity = input.validate().expect("Input should be valid");
    assert_eq!(severity, Severity::Severe);

    // 2. Dossier construction
    let dossier = ResearchDossier {
        particulate_profile: "Carbonaceous aerosols and combustion products.".to_string(),
        immediate_protocol: vec!["Douse with 15-minute low pressure mist.".to_string()],
        long_term_engineering: vec!["Install composting bins.".to_string()],
        statutory_memo: "Section 19(5) Air Act 1981 representation.".to_string(),
    };

    // 3. Record creation
    let mut record = input
        .into_record(
            "PV-TEST-001".to_string(),
            "2026-10-11T12:00:00Z".to_string(),
            Some(dossier),
            "prana-core-integration-test".to_string(),
        )
        .expect("Record creation should succeed");

    assert_eq!(record.incident_id, "PV-TEST-001");
    assert_eq!(record.lifecycle_state, LifecycleState::Assessed);

    // 4. State transition: Assessed -> ResponseActive
    assert!(record.lifecycle_state.can_transition_to(LifecycleState::ResponseActive));
    record.lifecycle_state.transition(LifecycleState::ResponseActive).expect("Transition to ResponseActive should succeed");
    assert_eq!(record.lifecycle_state, LifecycleState::ResponseActive);

    // 5. State transition: ResponseActive -> Documented
    record.lifecycle_state.transition(LifecycleState::Documented).expect("Transition to Documented should succeed");
    assert_eq!(record.lifecycle_state, LifecycleState::Documented);

    // 6. State transition: Documented -> Resolved
    record.lifecycle_state.transition(LifecycleState::Resolved).expect("Transition to Resolved should succeed");
    assert_eq!(record.lifecycle_state, LifecycleState::Resolved);

    // 7. Verify illegal backward transition is rejected
    let err = record.lifecycle_state.transition(LifecycleState::Reported).unwrap_err();
    assert_eq!(
        err,
        LifecycleError::InvalidTransition {
            from: LifecycleState::Resolved,
            to: LifecycleState::Reported
        }
    );
}

#[test]
fn test_coordinate_and_notes_bounds() {
    let invalid_lat = CreateIncidentInput {
        jurisdiction: "India".to_string(),
        zone: "Valid Zone".to_string(),
        source: "Biomass burning".to_string(),
        severity: "Moderate".to_string(),
        notes: "Valid notes".to_string(),
        lat: Some(95.0), // Invalid > 90
        lng: Some(78.0),
    };

    assert!(matches!(
        invalid_lat.validate(),
        Err(ValidationError::LatitudeOutOfBounds(_))
    ));
}
