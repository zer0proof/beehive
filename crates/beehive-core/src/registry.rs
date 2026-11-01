//! In-memory device bookkeeping shared by backends.

use std::collections::BTreeMap;

use crate::model::{Device, DeviceAddress};

/// A map of IEEE address to known device state.
///
/// Backends update this as interview data arrives; the CLI reads it when
/// listing devices.
#[derive(Debug, Clone, Default)]
pub struct Registry {
    devices: BTreeMap<DeviceAddress, Device>,
}

impl Registry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a registry from an iterator of devices.
    pub fn with_devices(devices: impl IntoIterator<Item = Device>) -> Self {
        let mut registry = Self::new();
        for device in devices {
            registry.upsert(device);
        }
        registry
    }

    /// Insert a device, replacing any existing entry with the same address.
    pub fn upsert(&mut self, device: Device) {
        self.devices.insert(device.address, device);
    }

    /// Look up a device by IEEE address.
    pub fn get(&self, address: DeviceAddress) -> Option<&Device> {
        self.devices.get(&address)
    }

    /// Remove a device and return it if it existed.
    pub fn remove(&mut self, address: DeviceAddress) -> Option<Device> {
        self.devices.remove(&address)
    }

    /// Iterate over all devices in address order.
    pub fn all(&self) -> impl Iterator<Item = &Device> {
        self.devices.values()
    }

    /// Number of tracked devices.
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    /// Whether the registry contains no devices.
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }
}
