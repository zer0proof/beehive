//! `beehive permit-join` — open the network to new devices.

use beehive_core::{Coordinator, PermitJoin};

pub async fn run(coordinator: &dyn Coordinator, duration: PermitJoin) -> anyhow::Result<()> {
    coordinator.permit_join(duration).await?;

    if duration.is_off() {
        println!("Permit join closed.");
    } else if duration.duration() == std::time::Duration::MAX {
        println!("Permit join open until closed explicitly.");
    } else {
        println!(
            "Permit join open for {} seconds.",
            duration.duration().as_secs()
        );
    }

    Ok(())
}
