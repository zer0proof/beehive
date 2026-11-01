//! An in-memory [`Coordinator`] for tests, demos, and offline development.
//!
//! The mock returns a small fixed network and logs every command instead of
//! touching hardware. It is the default backend for `beehive` so the CLI can
//! be exercised without a USB stick.

use async_trait::async_trait;
use tracing::{debug, info};

use crate::coordinator::{Coordinator, ScanResult};
use crate::error::{Error, Result};
use crate::model::{Device, DeviceAddress, Endpoint, NetworkAddress, PermitJoin};
use crate::registry::Registry;

/// A fake coordinator backed by an in-memory device registry.
#[derive(Debug)]
pub struct MockCoordinator {
    devices: Registry,
}

impl Default for MockCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl MockCoordinator {
    /// Create a mock coordinator seeded with two sample devices.
    pub fn new() -> Self {
        Self {
            devices: Registry::with_devices(sample_devices()),
        }
    }

    /// Create an empty mock coordinator with no devices.
    pub fn empty() -> Self {
        Self {
            devices: Registry::new(),
        }
    }
}

#[async_trait]
impl Coordinator for MockCoordinator {
    async fn start(&mut self) -> Result<()> {
        info!("mock coordinator started (no hardware)");
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("mock coordinator stopped");
        Ok(())
    }

    async fn scan(&self, channels: &[u8]) -> Result<Vec<ScanResult>> {
        let channels = if channels.is_empty() {
            &[11, 15, 20, 25][..]
        } else {
            channels
        };

        // Deterministic fake result: every requested channel sees one network.
        let results = channels
            .iter()
            .map(|&channel| ScanResult {
                channel,
                pan_id: 0x1a2b,
                extended_pan_id: Some(0x0012_4b00_1c2d_3e4f),
                permit_join: true,
                link_quality: Some(180),
                stack_profile: Some(2),
            })
            .collect::<Vec<_>>();

        debug!(count = results.len(), ?channels, "mock scan complete");
        Ok(results)
    }

    async fn devices(&self) -> Result<Vec<Device>> {
        Ok(self.devices.all().cloned().collect())
    }

    async fn permit_join(&self, duration: PermitJoin) -> Result<()> {
        if duration.is_off() {
            info!("mock permit-join closed");
        } else {
            info!(
                seconds = duration.duration().as_secs(),
                "mock permit-join opened"
            );
        }
        Ok(())
    }

    async fn send_zcl(
        &self,
        device: DeviceAddress,
        endpoint: u8,
        cluster: u16,
        payload: &[u8],
    ) -> Result<()> {
        if self.devices.get(device).is_none() {
            return Err(Error::DeviceNotFound(device));
        }

        debug!(
            %device,
            endpoint,
            cluster = format!("0x{cluster:04x}"),
            payload = %hex(payload),
            "mock ZCL send"
        );
        Ok(())
    }
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;

    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

fn sample_devices() -> Vec<Device> {
    vec![
        Device {
            address: DeviceAddress::new([0x00, 0x12, 0x4b, 0x00, 0x1c, 0x2d, 0x3e, 0x4f]),
            network_address: NetworkAddress::new(0x1a2b),
            manufacturer: Some("IKEA of Sweden".to_owned()),
            model: Some("TRADFRI bulb E27 WS opal 1000lm".to_owned()),
            endpoints: vec![Endpoint {
                id: 1,
                profile_id: 0x0104,
                device_id: 0x0101,
                input_clusters: vec![0x0000, 0x0003, 0x0004, 0x0006, 0x0008],
                output_clusters: Vec::new(),
            }],
            last_seen_unix: None,
        },
        Device {
            address: DeviceAddress::new([0x00, 0x12, 0x4b, 0x00, 0xaa, 0xbb, 0xcc, 0xdd]),
            network_address: NetworkAddress::new(0x2b3c),
            manufacturer: Some("LUMI".to_owned()),
            model: Some("lumi.sensor_motion.aq2".to_owned()),
            endpoints: vec![Endpoint {
                id: 1,
                profile_id: 0x0104,
                device_id: 0x0107,
                input_clusters: vec![0x0000, 0x0001, 0x0406],
                output_clusters: Vec::new(),
            }],
            last_seen_unix: None,
        },
    ]
}
