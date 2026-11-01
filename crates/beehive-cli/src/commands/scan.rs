//! `beehive scan` — scan channels for nearby networks.

use beehive_core::Coordinator;

/// Default channels scanned when the user does not pass `--channel`.
const DEFAULT_CHANNELS: &[u8] = &[11, 15, 20, 25];

pub async fn run(coordinator: &dyn Coordinator, channels: &[u8]) -> anyhow::Result<()> {
    let channels = if channels.is_empty() {
        DEFAULT_CHANNELS
    } else {
        channels
    };

    let networks = coordinator.scan(channels).await?;

    if networks.is_empty() {
        println!("No networks found on channels {channels:?}.");
        return Ok(());
    }

    println!(
        "{:<8} {:<8} {:<20} {:<12} LQI",
        "CHANNEL", "PAN ID", "EXTENDED PAN ID", "PERMIT JOIN"
    );

    for network in &networks {
        let extended = network
            .extended_pan_id
            .map(|value| format!("0x{value:016x}"))
            .unwrap_or_else(|| "-".to_owned());
        let permit = if network.permit_join { "yes" } else { "no" };
        let lqi = network
            .link_quality
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_owned());

        println!(
            "{:<8} {:<8} {:<20} {:<12} {}",
            network.channel,
            format!("0x{:04x}", network.pan_id),
            extended,
            permit,
            lqi
        );
    }

    Ok(())
}
