//! `beehive send` — send a raw ZCL payload.

use beehive_core::{Coordinator, DeviceAddress};

use super::parse_hex;

pub async fn run(
    coordinator: &dyn Coordinator,
    address: DeviceAddress,
    endpoint: u8,
    cluster: u16,
    payload: &str,
) -> anyhow::Result<()> {
    let payload = parse_hex(payload)?;

    coordinator
        .send_zcl(address, endpoint, cluster, &payload)
        .await?;

    println!(
        "Sent {} byte(s) to {address} endpoint {endpoint} cluster 0x{cluster:04x}.",
        payload.len()
    );

    Ok(())
}
