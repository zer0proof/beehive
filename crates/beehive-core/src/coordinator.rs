//! The backend contract.
//!
//! Every coordinator implementation — a real USB radio or the in-memory mock —
//! implements [`Coordinator`]. The CLI only ever talks to this trait, which
//! keeps protocol details out of the command layer and makes the mock usable
//! for tests and offline development.

use async_trait::async_trait;

use crate::error::Result;
use crate::model::{Device, DeviceAddress, PermitJoin};

/// A nearby ZigBee network discovered by an active scan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult {
    /// 802.15.4 channel, 11-26.
    pub channel: u8,
    /// 16-bit PAN identifier.
    pub pan_id: u16,
    /// 64-bit extended PAN identifier, when reported.
    pub extended_pan_id: Option<u64>,
    /// Whether the network currently accepts new devices.
    pub permit_join: bool,
    /// Link quality of the beacon, when reported.
    pub link_quality: Option<u8>,
    /// ZigBee stack profile, when reported.
    pub stack_profile: Option<u8>,
}

/// A ZigBee coordinator (network controller).
///
/// Implementations are expected to be cheap to share behind `&self` after
/// [`Coordinator::start`] has completed. Command methods take `&self`;
/// only lifecycle methods need `&mut self`.
#[async_trait]
pub trait Coordinator: Send + Sync {
    /// Open the transport, reset/negotiate with the NCP, and restore or form
    /// the network.
    async fn start(&mut self) -> Result<()>;

    /// Leave the network and release the transport.
    async fn stop(&mut self) -> Result<()>;

    /// Perform an active scan on the given channels.
    ///
    /// An empty slice means "use the backend's default channel mask".
    async fn scan(&self, channels: &[u8]) -> Result<Vec<ScanResult>>;

    /// Return the devices currently known to the coordinator.
    async fn devices(&self) -> Result<Vec<Device>>;

    /// Open (or close) the network to new devices.
    async fn permit_join(&self, duration: PermitJoin) -> Result<()>;

    /// Send a raw ZCL payload to one endpoint of a device.
    ///
    /// The backend is responsible for APS addressing and for reporting
    /// delivery failures.
    async fn send_zcl(
        &self,
        device: DeviceAddress,
        endpoint: u8,
        cluster: u16,
        payload: &[u8],
    ) -> Result<()>;
}
