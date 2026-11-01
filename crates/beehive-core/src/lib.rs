//! Backend-agnostic domain types and interfaces for `beehive`.
//!
//! This crate intentionally has no hardware or protocol dependencies. A
//! backend crate (for example `beehive-ezsp`) implements [`Coordinator`], and
//! the CLI wires an implementation up at runtime.
//!
//! Design notes and project state live in `CONTEXT.md` at the repository root.

pub mod coordinator;
pub mod error;
pub mod mock;
pub mod model;
pub mod registry;

pub use coordinator::{Coordinator, ScanResult};
pub use error::{Error, Result};
pub use model::{Device, DeviceAddress, Endpoint, NetworkAddress, PermitJoin};
pub use registry::Registry;
