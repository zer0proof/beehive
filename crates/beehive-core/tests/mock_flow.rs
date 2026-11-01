//! Integration test: the mock coordinator satisfies the public contract.

use beehive_core::{Coordinator, DeviceAddress, PermitJoin, mock::MockCoordinator};

#[tokio::test]
async fn mock_flow_works_end_to_end() {
    let mut coordinator = MockCoordinator::new();

    coordinator.start().await.expect("start");

    let networks = coordinator.scan(&[11, 15]).await.expect("scan");
    assert_eq!(networks.len(), 2);
    assert!(networks.iter().all(|network| network.permit_join));

    let devices = coordinator.devices().await.expect("devices");
    assert_eq!(devices.len(), 2);

    coordinator
        .permit_join(PermitJoin::seconds(60))
        .await
        .expect("permit join");

    coordinator
        .send_zcl(devices[0].address, 1, 0x0006, &[0x01])
        .await
        .expect("send zcl");

    let missing: DeviceAddress = "0x0000000000000001".parse().unwrap();
    assert!(
        coordinator
            .send_zcl(missing, 1, 0x0006, &[0x01])
            .await
            .is_err()
    );

    coordinator.stop().await.expect("stop");
}
