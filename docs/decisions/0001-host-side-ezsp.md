# ADR 0001 — Host-side EZSP over ASHv2, behind a core trait

- **Status:** accepted
- **Date:** 2026-11-01
- **Deciders:** project owner

## Context

`beehive` needs to interact with ZigBee devices from a Linux host using Rust.
Several paths were considered:

1. Talk to Zigbee2MQTT over MQTT or its HTTP/WebSocket API.
2. Drive a USB coordinator directly from Rust.
3. Run a ZigBee stack on embedded hardware and expose it over a network.

Direct USB was chosen because it keeps the system self-contained (no broker or
external service) and gives full control of the radio. The next question was
which crate implements the host side.

An early assumption was that the crates.io `zigbee` crate was a host-side
driver. It is not: `zigbee` v0.1.0 is a `no_std` embedded stack for running the
protocol on a device. The host-side crates are `ezsp` (protocol) and `ashv2`
(serial framing) on top of `tokio-serial`.

## Decision

- Implement a direct USB backend using **`ezsp` + `ashv2`** for Silicon Labs
  EmberZNet NCPs (CC2652/EFR32).
- Put the backend behind a `beehive_core::Coordinator` trait so the CLI and
  domain model stay protocol-agnostic.
- Ship a **mock** implementation as the default so development and tests do not
  need hardware.
- Treat TI Z-Stack/ZNP (CC253x) as a future, separate backend.

## Consequences

**Positive**

- Self-contained, Rust-native, no MQTT broker.
- Core and CLI are testable without a radio.
- A second backend can be added without touching the CLI's command logic.

**Negative / risks**

- `ezsp`/`ashv2` are young, fast-moving crates; APIs may change. Pin versions
  and verify against docs.rs.
- Hardware coverage is narrower than Zigbee2MQTT's.
- We own network lifecycle, key persistence, and device interviews.

## Alternatives rejected

- **Zigbee2MQTT over MQTT/HTTP** — simpler, but adds an external dependency and
  defeats the "direct" goal.
- **Embedded `zigbee` stack on a device** — wrong layer; does not drive a USB
  NCP from a host.
- **ZNP-only implementation** — would exclude the more common Silabs sticks.
