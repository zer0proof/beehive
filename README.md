# beehive

A learning project: **ZigBee from the bytes up, in Rust.**

This is not a product. The goal is to understand the protocol and become
comfortable with Rust by building the pieces by hand — MAC frames, network and
APS headers, ZCL payloads, then a real coordinator link.

## Start here

1. Read [LEARNING.md](LEARNING.md) — the plan, the working agreement, and the
   steps.
2. Read [CONTEXT.md](CONTEXT.md) — current state and decisions.
3. Install Rust (`sudo pacman -S rustup && rustup default stable`).
4. Run `cargo run`.

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
  backend, EZSP stub). It is the answer key: look at it *after* attempting a
  step, not before.

## References

- [docs/hardware.md](docs/hardware.md) — coordinators, firmware, serial setup
- [docs/decisions/0001-learning-first.md](docs/decisions/0001-learning-first.md)
  — why the project was reset to a skeleton
