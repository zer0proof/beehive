//! Core ZigBee data types.
//!
//! These types model the parts of a ZigBee network that the CLI and future
//! APIs care about. They are deliberately protocol-agnostic: EZSP/ZNP details
//! stay inside the backend crates.

use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// IEEE (EUI-64) address of a ZigBee device.
///
/// ZigBee tooling usually displays this as `0x00124b001c2d3e4f`; the parser
/// also accepts colon- or dash-separated forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeviceAddress([u8; 8]);

impl DeviceAddress {
    /// Build an address from its big-endian bytes.
    pub const fn new(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }

    /// Borrow the big-endian bytes.
    pub const fn as_bytes(&self) -> &[u8; 8] {
        &self.0
    }

    /// Interpret the address as a big-endian integer.
    pub const fn as_u64(&self) -> u64 {
        u64::from_be_bytes(self.0)
    }
}

impl fmt::Display for DeviceAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl From<[u8; 8]> for DeviceAddress {
    fn from(bytes: [u8; 8]) -> Self {
        Self::new(bytes)
    }
}

impl From<u64> for DeviceAddress {
    fn from(value: u64) -> Self {
        Self::new(value.to_be_bytes())
    }
}

/// Returned when a string cannot be parsed as a [`DeviceAddress`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseDeviceAddressError;

impl fmt::Display for ParseDeviceAddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            "expected a 64-bit IEEE address, e.g. 0x00124b001c2d3e4f or 00:12:4b:00:1c:2d:3e:4f",
        )
    }
}

impl std::error::Error for ParseDeviceAddressError {}

impl FromStr for DeviceAddress {
    type Err = ParseDeviceAddressError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();
        let trimmed = trimmed
            .strip_prefix("0x")
            .or_else(|| trimmed.strip_prefix("0X"))
            .unwrap_or(trimmed);

        let digits: String = trimmed
            .chars()
            .filter(|c| !matches!(c, ':' | '-' | ' ' | '_'))
            .collect();

        if digits.len() != 16 || !digits.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ParseDeviceAddressError);
        }

        let mut bytes = [0u8; 8];
        for (index, byte) in bytes.iter_mut().enumerate() {
            let start = index * 2;
            *byte = u8::from_str_radix(&digits[start..start + 2], 16)
                .map_err(|_| ParseDeviceAddressError)?;
        }

        Ok(Self(bytes))
    }
}

/// 16-bit network address assigned by the coordinator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NetworkAddress(u16);

impl NetworkAddress {
    /// Address used before a device has been assigned one.
    pub const UNKNOWN: Self = Self(0xffff);

    /// Broadcast to every router and end device.
    pub const BROADCAST: Self = Self(0xfffe);

    /// Create an address from a raw 16-bit value.
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Return the raw 16-bit value.
    pub const fn as_u16(self) -> u16 {
        self.0
    }
}

impl fmt::Display for NetworkAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04x}", self.0)
    }
}

impl From<u16> for NetworkAddress {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

/// A device endpoint: one application instance on a ZigBee node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Endpoint {
    /// Endpoint number, 1-240.
    pub id: u8,
    /// Application profile, usually `0x0104` (Home Automation).
    pub profile_id: u16,
    /// Device identifier within the profile.
    pub device_id: u16,
    /// Clusters the device accepts commands on (server side).
    pub input_clusters: Vec<u16>,
    /// Clusters the device sends commands on (client side).
    pub output_clusters: Vec<u16>,
}

/// A ZigBee node known to the coordinator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
    /// Stable IEEE address.
    pub address: DeviceAddress,
    /// Short address on the current network.
    pub network_address: NetworkAddress,
    /// Manufacturer name, once discovered.
    pub manufacturer: Option<String>,
    /// Model identifier, once discovered.
    pub model: Option<String>,
    /// Endpoints exposed by the device.
    pub endpoints: Vec<Endpoint>,
    /// Unix timestamp of the last message from the device, if known.
    pub last_seen_unix: Option<u64>,
}

impl Device {
    /// Human-readable label, falling back to the IEEE address.
    pub fn label(&self) -> String {
        match (self.manufacturer.as_deref(), self.model.as_deref()) {
            (Some(manufacturer), Some(model)) => format!("{manufacturer} {model}"),
            (Some(manufacturer), None) => manufacturer.to_owned(),
            (None, Some(model)) => model.to_owned(),
            (None, None) => self.address.to_string(),
        }
    }
}

/// How long the coordinator should allow new devices to join.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PermitJoin(Duration);

impl PermitJoin {
    /// Close the network to new devices.
    pub const OFF: Self = Self(Duration::ZERO);

    /// Open the network for the given number of seconds.
    pub fn seconds(seconds: u16) -> Self {
        Self(Duration::from_secs(u64::from(seconds)))
    }

    /// Open the network until explicitly closed.
    pub const fn forever() -> Self {
        Self(Duration::MAX)
    }

    /// The requested window.
    pub const fn duration(self) -> Duration {
        self.0
    }

    /// Whether this represents a closed network.
    pub const fn is_off(self) -> bool {
        self.0.is_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_prefixed_address() {
        let address: DeviceAddress = "0x00124b001c2d3e4f".parse().unwrap();
        assert_eq!(address.as_u64(), 0x0012_4b00_1c2d_3e4f);
    }

    #[test]
    fn parses_separated_address() {
        let address: DeviceAddress = "00:12:4b:00:1c:2d:3e:4f".parse().unwrap();
        assert_eq!(address.to_string(), "0x00124b001c2d3e4f");
    }

    #[test]
    fn rejects_short_address() {
        assert!("0x1234".parse::<DeviceAddress>().is_err());
    }
}
