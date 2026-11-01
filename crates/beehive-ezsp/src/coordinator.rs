//! The EZSP [`Coordinator`] implementation.

use async_trait::async_trait;
use beehive_core::{
    Coordinator, Device, DeviceAddress, PermitJoin, ScanResult,
    error::{Error, Result},
};
use tracing::debug;

use crate::config::EzspConfig;

/// Coordinator backed by a Silicon Labs EZSP network co-processor.
///
/// ## Intended wiring (next implementation step)
///
/// 1. Open the serial port with `tokio_serial::new(serial, baud).open_native_async()`.
/// 2. Split it and pass the halves to `ashv2::start(reader, writer)`; spawn the
///    returned transmitter/receiver futures with `tokio::spawn`.
/// 3. Build an `ezsp::Client`, negotiate the EZSP version, then construct an
///    `ezsp::Builder` to restore or form the network and register endpoints.
/// 4. Store the resulting `ezsp::Ncp` (and the spawned `JoinHandle`s) in this
///    struct so the command methods can use it.
///
/// The exact `ezsp`/`ashv2` APIs are version-sensitive. Pin and verify them
/// against <https://docs.rs/ezsp> and <https://docs.rs/ashv2> when wiring this
/// up, and record the version in `CONTEXT.md`.
#[derive(Debug)]
pub struct EzspCoordinator {
    config: EzspConfig,
    started: bool,
}

impl EzspCoordinator {
    /// Create a backend from configuration. No I/O happens until
    /// [`Coordinator::start`].
    pub fn new(config: EzspConfig) -> Self {
        Self {
            config,
            started: false,
        }
    }

    /// The configuration this instance was built with.
    pub fn config(&self) -> &EzspConfig {
        &self.config
    }

    /// Whether [`Coordinator::start`] has completed successfully.
    pub fn is_started(&self) -> bool {
        self.started
    }

    fn not_implemented(&self, operation: &str) -> Error {
        Error::Coordinator(format!(
            "EZSP backend is not implemented yet ({operation}); \
             see the wiring notes on EzspCoordinator"
        ))
    }
}

#[async_trait]
impl Coordinator for EzspCoordinator {
    async fn start(&mut self) -> Result<()> {
        debug!(
            serial = %self.config.serial,
            baud = self.config.baud,
            "opening EZSP coordinator"
        );

        // TODO: serial open -> ashv2::start -> ezsp::Builder -> store Ncp.
        Err(self.not_implemented("start"))
    }

    async fn stop(&mut self) -> Result<()> {
        self.started = false;
        Ok(())
    }

    async fn scan(&self, _channels: &[u8]) -> Result<Vec<ScanResult>> {
        Err(self.not_implemented("scan"))
    }

    async fn devices(&self) -> Result<Vec<Device>> {
        Err(self.not_implemented("devices"))
    }

    async fn permit_join(&self, _duration: PermitJoin) -> Result<()> {
        Err(self.not_implemented("permit_join"))
    }

    async fn send_zcl(
        &self,
        _device: DeviceAddress,
        _endpoint: u8,
        _cluster: u16,
        _payload: &[u8],
    ) -> Result<()> {
        Err(self.not_implemented("send_zcl"))
    }
}
