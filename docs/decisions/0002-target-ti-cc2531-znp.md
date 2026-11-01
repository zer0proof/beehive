# ADR 0002 — Target TI CC2531 with ZNP/MT, not EZSP

- **Status:** accepted
- **Date:** 2026-11-01
- **Deciders:** project owner
- **Related:** ADR 0001 (learning first)

## Context

The learning plan was written while the hardware was unspecified. The initial
scaffold assumed a Silicon Labs NCP (CC2652/EFR32) speaking **EZSP over
ASHv2**, because those are the common modern coordinators and the `ezsp` /
`ashv2` crates exist for them.

The owner actually has **TI CC2531** USB dongles. The CC2531 runs TI Z-Stack
and exposes the **ZNP / MT (Monitor and Test)** protocol over USB CDC serial.

These are different host protocols:

| | Silabs | TI |
|---|---|---|
| Host protocol | EZSP | ZNP / MT |
| Framing | ASHv2 | UNPI (`0xFE` SOF, length, FCS) |
| Typical port | `/dev/ttyUSB*` | `/dev/ttyACM*` |
| Our hardware | — | CC2531 |

## Decision

1. Target **TI CC2531** with **Z-Stack Home 1.2 coordinator firmware**.
2. Implement **ZNP/MT over serial** for the host side.
3. Treat EZSP/ASHv2 and Silicon Labs coordinators as **out of scope**.
4. Update `LEARNING.md`, `CONTEXT.md`, and `docs/hardware.md` accordingly.
5. Keep the archived scaffold as a Rust/structure reference only; its EZSP
   backend does not apply to this hardware.

## Consequences

**Positive**

- Matches the hardware the owner has; no purchase required.
- UNPI/ZNP framing is simpler than ASHv2, which is good for learning.
- A spare CC2531 can be flashed as a sniffer for real over-the-air captures.
- There is no maintained Rust ZNP crate, so implementing it is genuine work
  (and a stronger portfolio piece) rather than gluing a library.

**Negative / accepted**

- The CC2531 is old and limited: PCB antenna, poor range, and Zigbee2MQTT marks
  it "not recommended" for networks of roughly 20+ devices.
- Z-Stack Home 1.2 is an older stack; some Zigbee 3.0 behaviour is limited.
- Firmware may need flashing with a CC Debugger, and the exact build is
  currently unknown.

## Alternatives rejected

- **Buy a Silabs coordinator** to match the scaffold. Adds cost and does not
  use hardware the owner already has.
- **Use Zigbee2MQTT as a backend.** Hides the protocol, which defeats the
  learning goal.
- **Target both EZSP and ZNP now.** Unnecessary scope; ZNP alone is enough to
  learn the host protocol and the ZigBee layers.
