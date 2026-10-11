use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SeverityError {
    #[error("Unknown or unsupported severity: {0}. Valid severities: Moderate, High, Severe")]
    InvalidSeverity(String),
}

/// Operational severity classification for localized air pollution incidents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    Moderate = 1,
    High = 2,
    Severe = 3,
}

impl Severity {
    /// Numerical rank for threshold comparisons (1 to 3).
    pub fn rank(&self) -> u8 {
        *self as u8
    }

    /// Protocol response urgency window according to anti-vibe-coding operational guidance.
    pub fn response_window_minutes(&self) -> u32 {
        match self {
            Severity::Severe => 15,
            Severity::High => 60,
            Severity::Moderate => 360,
        }
    }

    /// Indicates whether immediate institutional supervisor escalation is mandated.
    pub fn requires_immediate_escalation(&self) -> bool {
        matches!(self, Severity::Severe | Severity::High)
    }

    /// Returns human-readable label.
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Moderate => "Moderate",
            Severity::High => "High",
            Severity::Severe => "Severe",
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Severity {
    type Err = SeverityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "moderate" => Ok(Severity::Moderate),
            "high" => Ok(Severity::High),
            "severe" => Ok(Severity::Severe),
            other => Err(SeverityError::InvalidSeverity(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_from_str() {
        assert_eq!(Severity::from_str("Moderate").unwrap(), Severity::Moderate);
        assert_eq!(Severity::from_str("moderate").unwrap(), Severity::Moderate);
        assert_eq!(Severity::from_str("HIGH").unwrap(), Severity::High);
        assert_eq!(Severity::from_str("Severe").unwrap(), Severity::Severe);
        assert!(Severity::from_str("Critical").is_err());
    }

    #[test]
    fn test_severity_rank_and_ordering() {
        assert!(Severity::Severe > Severity::High);
        assert!(Severity::High > Severity::Moderate);
        assert_eq!(Severity::Severe.rank(), 3);
        assert_eq!(Severity::High.rank(), 2);
        assert_eq!(Severity::Moderate.rank(), 1);
    }

    #[test]
    fn test_escalation_rules() {
        assert!(Severity::Severe.requires_immediate_escalation());
        assert!(Severity::High.requires_immediate_escalation());
        assert!(!Severity::Moderate.requires_immediate_escalation());
    }

    #[test]
    fn test_response_windows() {
        assert_eq!(Severity::Severe.response_window_minutes(), 15);
        assert_eq!(Severity::High.response_window_minutes(), 60);
        assert_eq!(Severity::Moderate.response_window_minutes(), 360);
    }
}
