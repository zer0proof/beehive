# ADR 0001 — Learning first

- **Status:** accepted; hardware assumption updated by ADR 0002
- **Date:** 2026-11-01
- **Deciders:** project owner

## Context

The project started as "build a Rust tool for interacting with ZigBee devices."
An initial scaffold was produced quickly: a three-crate workspace, domain model,
`Coordinator` trait, mock backend, CLI with subcommands, and an EZSP backend
stub. It compiled, had tests, and passed fmt/clippy.

The owner then clarified the real goal: **understand the ZigBee protocol, learn
Rust, and build a portfolio piece** — not ship a product. The finished scaffold
worked against that goal: it answered questions the owner wanted to answer, and
left little room for meaningful contribution.

## Decision

1. Treat the project as **learning-first**. Hand-write protocol parsing before
   using libraries that hide the bytes.
2. Adopt a **tutor-mode working agreement**: the owner writes code; the agent
   provides specs, hints, review, and tests, and implements only when asked.
3. **Archive** the existing scaffold on branch `reference/scaffold` as an answer
   key rather than deleting it.
4. Reset `main` to a **minimal one-crate skeleton** with no dependencies.
5. Build up in small steps, starting with an 802.15.4 MAC dissector that needs
   no hardware.

## Consequences

**Positive**

- The owner makes the design and parsing decisions; the learning is real.
- Steps 1–3 need no coordinator hardware.
- The reference branch still provides Rust idioms and a comparison point.
- The commit history shows the learning process, which is useful for a
  portfolio.

**Negative / accepted**

- Slower than reusing the scaffold.
- We will re-solve some problems the scaffold already solved.
- The archived code may drift out of date; it is a snapshot, not maintained.

## Alternatives rejected

- **Keep the scaffold and add features.** Faster, but preserves the imbalance:
  too much finished code, too few decisions.
- **Delete everything and start from an empty directory.** Unnecessary; the
  workspace config, hardware notes, and reference implementation are useful.
- **Use `ezsp`/`ashv2` immediately.** Teaches those crates' APIs, not ZigBee.
  They can be consulted later, or adopted once the protocol is understood.
