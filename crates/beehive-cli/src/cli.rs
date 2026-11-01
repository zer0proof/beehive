//! Command-line surface.

use std::path::PathBuf;

use beehive_core::DeviceAddress;
use clap::{Parser, Subcommand, ValueEnum};
use serde::Deserialize;

/// Talk to ZigBee devices through a coordinator.
#[derive(Debug, Parser)]
#[command(name = "beehive", version, about, long_about = None)]
pub struct Cli {
    /// Path to the TOML configuration file.
    #[arg(
        short,
        long,
        global = true,
        env = "BEEHIVE_CONFIG",
        default_value = "config/beehive.toml"
    )]
    pub config: PathBuf,

    /// Override the backend from the config file.
    #[arg(long, global = true, value_enum)]
    pub backend: Option<Backend>,

    /// Enable debug logging.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Command,
}

/// Which coordinator implementation to use.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    /// In-memory fake network; no hardware required.
    #[default]
    Mock,
    /// Silicon Labs NCP over EZSP/ASHv2.
    Ezsp,
}

/// Top-level commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Open (or close) a permit-join window for new devices.
    PermitJoin {
        /// Window length in seconds.
        #[arg(default_value_t = 60)]
        seconds: u16,
        /// Leave the window open until closed explicitly.
        #[arg(long)]
        forever: bool,
    },

    /// Actively scan channels for nearby ZigBee networks.
    Scan {
        /// Channels to scan (11-26). Repeat or comma-separate. Defaults to 11,15,20,25.
        #[arg(
            long = "channel",
            value_delimiter = ',',
            value_parser = clap::value_parser!(u8).range(11..=26)
        )]
        channels: Vec<u8>,
    },

    /// List devices known to the coordinator.
    Devices,

    /// Send a raw ZCL frame to a device endpoint.
    Send {
        /// IEEE address, e.g. 0x00124b001c2d3e4f.
        #[arg(value_parser = parse_device_address)]
        address: DeviceAddress,
        /// Destination endpoint (1-240).
        #[arg(value_parser = clap::value_parser!(u8).range(1..=240))]
        endpoint: u8,
        /// Cluster ID, e.g. 0x0006 for On/Off.
        #[arg(value_parser = parse_u16)]
        cluster: u16,
        /// Hex-encoded ZCL payload, e.g. 01 for "on".
        #[arg(long)]
        payload: String,
    },
}

fn parse_u16(input: &str) -> Result<u16, std::num::ParseIntError> {
    let trimmed = input.trim();
    match trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        Some(hex) => u16::from_str_radix(hex, 16),
        None => trimmed.parse(),
    }
}

fn parse_device_address(
    input: &str,
) -> Result<DeviceAddress, beehive_core::model::ParseDeviceAddressError> {
    input.parse()
}
