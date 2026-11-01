//! Configuration for the EZSP backend.

use serde::Deserialize;

/// Default serial device used by most USB coordinators on Linux.
pub const DEFAULT_SERIAL: &str = "/dev/ttyUSB0";

/// Default EZSP baud rate for Silabs NCPs.
pub const DEFAULT_BAUD: u32 = 115_200;

/// Settings for connecting to an EZSP network co-processor.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct EzspConfig {
    /// Path to the serial device, e.g. `/dev/ttyUSB0`.
    pub serial: String,
    /// Serial baud rate. Most flashed NCPs use 115200; some use 460800.
    pub baud: u32,
}

impl Default for EzspConfig {
    fn default() -> Self {
        Self {
            serial: DEFAULT_SERIAL.to_owned(),
            baud: DEFAULT_BAUD,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_usable() {
        let config = EzspConfig::default();
        assert_eq!(config.serial, "/dev/ttyUSB0");
        assert_eq!(config.baud, 115_200);
    }
}
