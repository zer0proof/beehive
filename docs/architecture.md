# Architecture

## Goals

- Drive a USB ZigBee coordinator from Rust with no external broker.
- Keep protocol details isolated so backends are swappable.
- Make the whole CLI testable without hardware.

## Layers

```text
CLI (beehive-cli)
  │  clap parsing, config, output
  ▼
Domain (beehive-core)
  │  Coordinator trait, model types, Registry, Error
  ▼
Backend (beehive-ezsp)
  │  EzspCoordinator
  ▼
Protocol crates (ezsp, ashv2)
  │
  ▼
Serial (tokio-serial) ──► USB NCP ──► 802.15.4 ──► devices
```

### `beehive-core`

Pure data and contracts. No serial, no `ezsp`, no Tokio runtime required by
consumers (Tokio is a dev-dependency for tests only).

The `Coordinator` trait:

```rust
#[async_trait]
pub trait Coordinator: Send + Sync {
    async fn start(&mut self) -> Result<()>;
    async fn stop(&mut self) -> Result<()>;
    async fn scan(&self, channels: &[u8]) -> Result<Vec<ScanResult>>;
    async fn devices(&self) -> Result<Vec<Device>>;
    async fn permit_join(&self, duration: PermitJoin) -> Result<()>;
    async fn send_zcl(&self, device: DeviceAddress, endpoint: u8, cluster: u16, payload: &[u8]) -> Result<()>;
}
```

Lifecycle takes `&mut self`; everything else takes `&self` so the coordinator
can be shared once started (behind a lock or channels inside the backend).

### `beehive-ezsp`

Implements `Coordinator` for Silicon Labs NCPs:

1. Open the serial device with `tokio-serial`.
2. Split it into reader/writer and call `ashv2::start`.
3. Spawn the ASHv2 transmitter/receiver futures.
4. Negotiate EZSP and configure the stack with `ezsp::Builder`.
5. Keep the resulting `ezsp::Ncp` plus task handles in the struct.

The stub documents this sequence on `EzspCoordinator`. When implementing,
expect to add an internal event loop/task that receives NCP callbacks
(device announcements, incoming messages) and updates the `Registry`.

### `beehive-cli`

Owns `clap` parsing, TOML config, backend selection, and table output. It
depends only on the `Coordinator` trait plus the concrete backends used for
construction. Adding a backend means adding a variant to `Backend` and a match
arm in `build_coordinator`.

## Concurrency model

- One Tokio multi-thread runtime, created by `#[tokio::main]`.
- The backend owns serial I/O tasks. Trait methods should communicate with
  those tasks (channels/actor pattern) rather than locking the port directly.
- `start()` is the only place that performs long setup; command methods should
  be safe to call concurrently once started.

## Error strategy

- `beehive_core::Error` is the shared vocabulary (`NotConnected`,
  `DeviceNotFound`, `Config`, `Coordinator`, `Timeout`, `Backend`).
- Backends translate protocol errors at the boundary; library callers never see
  `ezsp` types.
- The CLI converts to `anyhow`, prints with `{:#}` for context chains, and
  returns a non-zero exit code.

## Testing strategy

- Unit tests for parsing and domain invariants (address parsing, hex payloads).
- Integration test (`crates/beehive-core/tests/mock_flow.rs`) drives the mock
  through the full trait.
- Hardware tests are manual for now; later, a recorded/replayed EZSP trace
  could make the backend testable offline.

## Extension points

- **New protocol backend:** implement `Coordinator`, add a `Backend` variant.
- **JSON output:** add a global `--json` flag and serialize the model types
  (they already derive `serde`).
- **Daemon/API:** wrap a started `Coordinator` in an HTTP server; the trait
  object is the natural boundary.
- **Device definitions:** map clusters/attributes to typed commands; consider
  `zigbee2mqtt-types` for generated defs.
