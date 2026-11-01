# beehive

A learning project: **ZigBee from the bytes up, in Rust.**

This is not a product. The goal is to understand the protocol and become
comfortable with Rust by building the pieces by hand — MAC frames, network and
APS headers, ZCL payloads, then a real coordinator link.

**Target hardware:** TI **CC2531** USB dongles running Z-Stack Home 1.2
coordinator firmware, using the **ZNP/MT** protocol over serial (not EZSP).

## Start here

1. Read [LEARNING.md](LEARNING.md) — the plan, the working agreement, and the
   steps.
2. Install Rust (`sudo pacman -S rustup && rustup default stable`).
3. Run `cargo run`.

`CONTEXT.md` (current state and decisions) and `AGENTS.md` (agent instructions)
are local-only and deliberately not committed.

```text
beehive: skeleton only. Read LEARNING.md and take Step 1.
```

## Current step

**Step 1 — 802.15.4 MAC header dissector**, standard library only.
See the assignment and sample frames in
[LEARNING.md](LEARNING.md#step-1--802154-mac-header-dissector).

## Branches

- `main` — the learning skeleton. You build here.
- `reference/scaffold` — an earlier, fully working implementation (CLI, mock
  backend, **EZSP** stub). Since the target is CC2531/ZNP, treat it only as a
  Rust/architecture reference; its protocol layer does not apply.

## References

- [docs/hardware.md](docs/hardware.md) — CC2531 firmware, flashing, serial setup
- [docs/decisions/0001-learning-first.md](docs/decisions/0001-learning-first.md)
  — why the project was reset to a skeleton
- [docs/decisions/0002-target-ti-cc2531-znp.md](docs/decisions/0002-target-ti-cc2531-znp.md)
  — why the target is CC2531/ZNP instead of EZSP
