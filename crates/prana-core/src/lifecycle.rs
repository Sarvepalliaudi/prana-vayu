use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum LifecycleError {
    #[error("Invalid state transition from {from:?} to {to:?}")]
    InvalidTransition {
        from: LifecycleState,
        to: LifecycleState,
    },
    #[error("Unknown lifecycle state: {0}")]
    UnknownState(String),
}

/// Permissible state transitions for an incident lifecycle:
/// Reported -> Validated -> Assessed -> ResponseActive -> Documented -> Resolved -> Archived
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LifecycleState {
    Reported,
    Validated,
    Assessed,
    ResponseActive,
    Documented,
    Resolved,
    Archived,
}

impl LifecycleState {
    /// Validates whether the state machine allows a transition from current state to `next`.
    pub fn can_transition_to(&self, next: LifecycleState) -> bool {
        match (self, next) {
            // Can always self-transition (idempotent updates)
            (s, n) if *s == n => true,

            // Forward happy path
            (LifecycleState::Reported, LifecycleState::Validated) => true,
            (LifecycleState::Reported, LifecycleState::Archived) => true, // False alarm / dismissed

            (LifecycleState::Validated, LifecycleState::Assessed) => true,
            (LifecycleState::Validated, LifecycleState::Archived) => true,

            (LifecycleState::Assessed, LifecycleState::ResponseActive) => true,
            (LifecycleState::Assessed, LifecycleState::Documented) => true,

            (LifecycleState::ResponseActive, LifecycleState::Documented) => true,
            (LifecycleState::ResponseActive, LifecycleState::Resolved) => true,

            (LifecycleState::Documented, LifecycleState::Resolved) => true,
            (LifecycleState::Documented, LifecycleState::Archived) => true,

            (LifecycleState::Resolved, LifecycleState::Archived) => true,

            // Backward or illegal jumps are forbidden
            _ => false,
        }
    }

    /// Transitions to the target state if legal, or returns typed error.
    pub fn transition(&mut self, next: LifecycleState) -> Result<(), LifecycleError> {
        if self.can_transition_to(next) {
            *self = next;
            Ok(())
        } else {
            Err(LifecycleError::InvalidTransition {
                from: *self,
                to: next,
            })
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LifecycleState::Reported => "Reported",
            LifecycleState::Validated => "Validated",
            LifecycleState::Assessed => "Assessed",
            LifecycleState::ResponseActive => "ResponseActive",
            LifecycleState::Documented => "Documented",
            LifecycleState::Resolved => "Resolved",
            LifecycleState::Archived => "Archived",
        }
    }
}

impl fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for LifecycleState {
    type Err = LifecycleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "reported" => Ok(LifecycleState::Reported),
            "validated" => Ok(LifecycleState::Validated),
            "assessed" => Ok(LifecycleState::Assessed),
            "responseactive" | "response_active" => Ok(LifecycleState::ResponseActive),
            "documented" => Ok(LifecycleState::Documented),
            "resolved" => Ok(LifecycleState::Resolved),
            "archived" => Ok(LifecycleState::Archived),
            other => Err(LifecycleError::UnknownState(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_forward_transitions() {
        let mut state = LifecycleState::Reported;
        assert!(state.transition(LifecycleState::Validated).is_ok());
        assert_eq!(state, LifecycleState::Validated);

        assert!(state.transition(LifecycleState::Assessed).is_ok());
        assert_eq!(state, LifecycleState::Assessed);

        assert!(state.transition(LifecycleState::ResponseActive).is_ok());
        assert_eq!(state, LifecycleState::ResponseActive);

        assert!(state.transition(LifecycleState::Resolved).is_ok());
        assert_eq!(state, LifecycleState::Resolved);

        assert!(state.transition(LifecycleState::Archived).is_ok());
        assert_eq!(state, LifecycleState::Archived);
    }

    #[test]
    fn test_invalid_backward_transitions() {
        let mut state = LifecycleState::Resolved;
        assert!(state.transition(LifecycleState::Reported).is_err());
        assert_eq!(state, LifecycleState::Resolved);
    }

    #[test]
    fn test_idempotent_transition() {
        let mut state = LifecycleState::Validated;
        assert!(state.transition(LifecycleState::Validated).is_ok());
        assert_eq!(state, LifecycleState::Validated);
    }
}
