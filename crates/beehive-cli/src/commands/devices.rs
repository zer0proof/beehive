//! `beehive devices` — list known devices.

use beehive_core::Coordinator;

pub async fn run(coordinator: &dyn Coordinator) -> anyhow::Result<()> {
    let devices = coordinator.devices().await?;

    if devices.is_empty() {
        println!("No devices known yet. Try `beehive permit-join` first.");
        return Ok(());
    }

    println!(
        "{:<20} {:<8} {:<8} {:<48} ENDPOINTS",
        "IEEE ADDRESS", "SHORT", "PROFILE", "LABEL"
    );
    for device in &devices {
        let profile = device
            .endpoints
            .first()
            .map(|endpoint| format!("0x{:04x}", endpoint.profile_id))
            .unwrap_or_else(|| "-".to_owned());
        let endpoints = device
            .endpoints
            .iter()
            .map(|endpoint| endpoint.id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        println!(
            "{:<20} {:<8} {:<8} {:<48} {}",
            device.address.to_string(),
            device.network_address.to_string(),
            profile,
            device.label(),
            endpoints
        );
    }

    Ok(())
}
