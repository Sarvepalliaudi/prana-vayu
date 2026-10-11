use thiserror::Error;

pub const MAX_ZONE_LEN: usize = 180;
pub const MAX_NOTES_LEN: usize = 3000;
pub const MIN_NOTES_LEN: usize = 3;

pub const VALID_JURISDICTIONS: &[&str] = &["India", "US", "UK", "EU", "Global"];
pub const VALID_SOURCES: &[&str] = &[
    "Biomass Burning",
    "Vehicle Idling",
    "Silica Dust",
    "Kitchen Exhaust",
    "Perimeter Waste Combustion",
];

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("Missing required field: {0}")]
    MissingRequiredField(&'static str),

    #[error("Zone description length ({0}) exceeds maximum allowed ({MAX_ZONE_LEN})")]
    ZoneTooLong(usize),

    #[error("Zone description cannot be empty")]
    ZoneEmpty,

    #[error("Incident notes length ({0}) must be between {MIN_NOTES_LEN} and {MAX_NOTES_LEN} chars")]
    NotesLengthInvalid(usize),

    #[error("Latitude ({0}) must be between -90.0 and 90.0 degrees")]
    LatitudeOutOfBounds(f64),

    #[error("Longitude ({0}) must be between -180.0 and 180.0 degrees")]
    LongitudeOutOfBounds(f64),

    #[error("Asymmetric coordinates provided: both latitude and longitude must be present together")]
    AsymmetricCoordinates,

    #[error("Invalid jurisdiction: '{0}'. Permitted: India, US, UK, EU, Global")]
    InvalidJurisdiction(String),

    #[error("Invalid pollution source: '{0}'")]
    InvalidSource(String),

    #[error("Invalid incident identifier format")]
    InvalidId,
}

/// Validates geographic latitude and longitude bounds.
pub fn validate_coordinates(
    lat_opt: Option<f64>,
    lng_opt: Option<f64>,
) -> Result<(), ValidationError> {
    match (lat_opt, lng_opt) {
        (None, None) => Ok(()),
        (Some(_), None) | (None, Some(_)) => Err(ValidationError::AsymmetricCoordinates),
        (Some(lat), Some(lng)) => {
            if !(-90.0..=90.0).contains(&lat) || lat.is_nan() {
                return Err(ValidationError::LatitudeOutOfBounds(lat));
            }
            if !(-180.0..=180.0).contains(&lng) || lng.is_nan() {
                return Err(ValidationError::LongitudeOutOfBounds(lng));
            }
            Ok(())
        }
    }
}

/// Validates micro-climate zone string.
pub fn validate_zone(zone: &str) -> Result<(), ValidationError> {
    let trimmed = zone.trim();
    if trimmed.is_empty() {
        return Err(ValidationError::ZoneEmpty);
    }
    if trimmed.chars().count() > MAX_ZONE_LEN {
        return Err(ValidationError::ZoneTooLong(trimmed.chars().count()));
    }
    Ok(())
}

/// Validates notes / observation text.
pub fn validate_notes(notes: &str) -> Result<(), ValidationError> {
    let len = notes.trim().chars().count();
    if !(MIN_NOTES_LEN..=MAX_NOTES_LEN).contains(&len) {
        return Err(ValidationError::NotesLengthInvalid(len));
    }
    Ok(())
}

/// Validates jurisdiction enumeration.
pub fn validate_jurisdiction(jurisdiction: &str) -> Result<(), ValidationError> {
    if VALID_JURISDICTIONS.iter().any(|&j| j.eq_ignore_ascii_case(jurisdiction.trim())) {
        Ok(())
    } else {
        Err(ValidationError::InvalidJurisdiction(jurisdiction.to_string()))
    }
}

/// Validates pollution source enumeration.
pub fn validate_source(source: &str) -> Result<(), ValidationError> {
    if VALID_SOURCES.iter().any(|&s| s.eq_ignore_ascii_case(source.trim())) {
        Ok(())
    } else {
        Err(ValidationError::InvalidSource(source.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates_valid() {
        assert!(validate_coordinates(Some(12.9716), Some(77.5946)).is_ok());
        assert!(validate_coordinates(Some(-90.0), Some(180.0)).is_ok());
        assert!(validate_coordinates(None, None).is_ok());
    }

    #[test]
    fn test_coordinates_invalid() {
        assert_eq!(
            validate_coordinates(Some(12.9716), None),
            Err(ValidationError::AsymmetricCoordinates)
        );
        assert_eq!(
            validate_coordinates(Some(90.1), Some(77.0)),
            Err(ValidationError::LatitudeOutOfBounds(90.1))
        );
        assert_eq!(
            validate_coordinates(Some(12.0), Some(-180.1)),
            Err(ValidationError::LongitudeOutOfBounds(-180.1))
        );
    }

    #[test]
    fn test_zone_validation() {
        assert!(validate_zone("North Campus Gate 4").is_ok());
        assert_eq!(validate_zone("   "), Err(ValidationError::ZoneEmpty));
        let too_long = "a".repeat(181);
        assert_eq!(validate_zone(&too_long), Err(ValidationError::ZoneTooLong(181)));
    }

    #[test]
    fn test_notes_validation() {
        assert!(validate_notes("Dry leaves burning near student hostel").is_ok());
        assert_eq!(validate_notes("a"), Err(ValidationError::NotesLengthInvalid(1)));
        let too_long = "x".repeat(3001);
        assert_eq!(validate_notes(&too_long), Err(ValidationError::NotesLengthInvalid(3001)));
    }

    #[test]
    fn test_jurisdiction_validation() {
        assert!(validate_jurisdiction("India").is_ok());
        assert!(validate_jurisdiction("india").is_ok());
        assert!(validate_jurisdiction("US").is_ok());
        assert!(validate_jurisdiction("Mars").is_err());
    }

    #[test]
    fn test_source_validation() {
        assert!(validate_source("Biomass Burning").is_ok());
        assert!(validate_source("Vehicle Idling").is_ok());
        assert!(validate_source("Nuclear Fallout").is_err());
    }
}
