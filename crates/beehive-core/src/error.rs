//! Error types shared by every `beehive` crate.

use std::time::Duration;

use thiserror::Error;

use crate::model::DeviceAddress;

/// Convenience alias used throughout the workspace.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors produced by the core layer.
///
/// Backends should convert their protocol-specific failures into
/// [`Error::Coordinator`] (with context in the message) or wrap them in
/// [`Error::Backend`] when the original error should stay inspectable.
#[derive(Debug, Error)]
pub enum Error {
    /// An operation required a started coordinator.
    #[error("coordinator is not connected")]
    NotConnected,

    /// The requested device is not known to the coordinator.
    #[error("device {0} was not found")]
    DeviceNotFound(DeviceAddress),

    /// Configuration was syntactically valid but cannot be used.
    #[error("invalid configuration: {0}")]
    Config(String),

    /// The backing coordinator reported a failure.
    #[error("coordinator error: {0}")]
    Coordinator(String),

    /// An operation did not finish in time.
    #[error("operation timed out after {0:?}")]
    Timeout(Duration),

    /// A backend-specific error, preserved for inspection.
    #[error(transparent)]
    Backend(Box<dyn std::error::Error + Send + Sync>),
}
