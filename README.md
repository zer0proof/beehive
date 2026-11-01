# beehive

A Rust workspace for interacting with ZigBee devices through a USB coordinator.

> **Status:** early scaffold. The domain model, CLI, and mock backend work; the
> real EZSP hardware backend is a stub. See [CONTEXT.md](CONTEXT.md) for the
> full state and roadmap.

## What works today

The `mock` backend exercises the whole CLI without hardware:

```sh
cargo run -p beehive-cli -- devices
cargo run -p beehive-cli -- scan --channel 11,15
cargo run -p beehive-cli -- permit-join 60
cargo run -p beehive-cli -- send 0x00124b001c2d3e4f 1 0x0006 --payload 01
```

The `ezsp` backend (Silicon Labs NCPs such as CC2652/EFR32 sticks) is wired
into the CLI but returns "not implemented yet" until Phase 2 of the roadmap.

## Requirements

- Rust stable (edition 2024, MSRV 1.85). On Arch:
  `sudo pacman -S rustup && rustup default stable`
- For real hardware: a Silabs EZSP coordinator and membership in the `uucp`
  group. See [docs/hardware.md](docs/hardware.md).

## Layout

```text
crates/beehive-core   domain types, Coordinator trait, mock backend
crates/beehive-ezsp   EZSP/ASHv2 backend (stub)
crates/beehive-cli    the `beehive` binary
config/               example configuration
docs/                 architecture, hardware, decision records
CONTEXT.md            living project context (read this)
```

## Configuration

Copy the template and edit it:

```sh
cp config/beehive.example.toml config/beehive.toml
```

```toml
backend = "mock"          # or "ezsp"

[ezsp]
serial = "/dev/ttyUSB0"
baud = 115200
```

The `--backend` flag overrides the file. `--config` (or `BEEHIVE_CONFIG`)
selects a different file.

## Development

```sh
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

## Documentation

- [CONTEXT.md](CONTEXT.md) — decisions, state, roadmap, glossary
- [docs/architecture.md](docs/architecture.md) — design and data flow
- [docs/hardware.md](docs/hardware.md) — coordinators and serial setup
- [docs/decisions/](docs/decisions/) — architecture decision records
