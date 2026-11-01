//! EZSP coordinator backend for Silicon Labs NCP radios.
//!
//! Supported hardware: any NCP running EmberZNet with the EZSP protocol over
//! ASHv2 — for example CC2652/CC2652P USB sticks and EFR32 based coordinators
//! (Sonoff ZBDongle-E, SLZB-06 in EZSP mode, ...). TI CC253x sticks speak ZNP
//! instead and are not handled here.
//!
//! ## Status
//!
//! This is a skeleton: the crate compiles against the real protocol crates but
//! the command methods still return [`beehive_core::Error::Coordinator`]. The
//! intended wiring is documented on [`EzspCoordinator`] and in `CONTEXT.md`.
//!
//! ## Layer stack
//!
//! ```text
//! beehive-cli
//!   -> beehive-core::Coordinator
//!     -> beehive-ezsp::EzspCoordinator
//!       -> ezsp::Builder / ezsp::Ncp          (EZSP commands + APS)
//!         -> ashv2::start(reader, writer)     (framing over serial)
//!           -> tokio_serial                    (/dev/ttyUSB0)
//! ```

mod config;
mod coordinator;

pub use config::EzspConfig;
pub use coordinator::EzspCoordinator;
