# Learning plan

This is a **learning project**, not a product. The goal is to understand the
ZigBee protocol and become comfortable writing Rust, while building something
you can show and explain.

The measure of success is not "the CLI works." It is: *can you explain why each
byte is where it is, and why the Rust code is shaped the way it is?*

---

## How we work

You are the programmer. I am the tutor, reviewer, and rubber duck.

- **You write the code.** I do not write implementation for you unless you
  explicitly say "write this part for me."
- **I give specs, questions, hints, references, and code review.** If you ask
  "how do I do X?", expect a question or a pointer before an answer.
- **Predict, then compile.** Before running, write down what you expect. Compare.
  The surprise is where the learning is.
- **Read the compiler.** Rust's errors are unusually good. Paste the full error
  and what you tried; don't just ask for the fix.
- **Explain it back.** At the end of each step, write a short note in
  `notes/step-NN.md`: what the bytes mean, what Rust fought you on, what you'd
  do differently.
- **Commit at checkpoints.** Small commits with messages that explain *why*.

If I ever start dumping finished code again, say "tutor mode" and I'll stop.

---

## Setup

Rust is not installed on this machine yet. On Arch:

```sh
sudo pacman -S rustup
rustup default stable
```

Then from the repository root:

```sh
cargo run          # should print the skeleton line
cargo test         # no tests yet
cargo fmt
cargo clippy --all-targets -- -D warnings
```

> Docker is also available. If we need to check a build without installing
> Rust, we can use `rust:slim` with `--network host`. But for day-to-day work,
> install Rust locally so `cargo` and `rust-analyzer` are fast.

---

## The ladder

Each step is small enough to finish, and each one produces something you can
demonstrate. Steps 1–3 need **no hardware**.

| Step | Build | Rust | ZigBee |
|------|-------|------|--------|
| 1 | 802.15.4 MAC header dissector | enums, `match`, `Result`, `Option`, slicing, tests | frame control, addressing, endianness |
| 2 | NWK + APS decoder | borrowing, iterators, custom errors, `TryFrom` | network/APS layers, endpoints, clusters |
| 3 | ZCL payload decoder | traits, enums with data, generics, codecs | clusters, attributes, commands, data types |
| 4 | Serial framing + loopback tests | async/await, tasks, channels, timeouts | host↔NCP framing (ASH) |
| 5 | EZSP subset: version, network init, permit join | actor pattern, state machines, error context | commissioning |
| 6 | Device interview + On/Off control | API/CLI design, modules | discovery, binding, ZCL commands |

### Step 0 — Run the skeleton

Done when `cargo run` works and you have run `cargo fmt` and `cargo clippy` once.
No code to write.

### Step 1 — 802.15.4 MAC header dissector

**Build:** a binary that takes a hex string and prints the MAC header fields:

```sh
cargo run -- "41 98 2a 2b 1a 34 12 78 56"
```

**Output (roughly):**

```text
Frame type:     Data
Security:       off
Frame pending:  off
Ack request:    off
PAN compressed: yes
Sequence:       42
Dest PAN:       0x1a2b
Dest address:   0x1234 (short)
Source address: 0x5678 (short)
Frame version:  1 (2006)
```

**Constraints for this step:** standard library only. No `clap`, no `hex`
crate, no `nom`. The point is to feel the bytes.

**Rust to practice:** `std::env::args`, `String`/`&str`, `u16::from_le_bytes`,
enums with fields, `match`, returning `Result<_, String>` (or your own error
type), slicing a `&[u8]`, and `#[cfg(test)]` tests.

**ZigBee to learn:** the 802.15.4 frame control bits, addressing modes
(none/short/extended), PAN ID compression, sequence numbers, and why multi-byte
fields are little-endian.

**Sample vectors** (MAC headers only; payload bytes may follow):

1. `41 98 2a 2b 1a 34 12 78 56`
   Data, seq 42, PAN 0x1a2b, dest 0x1234, source 0x5678, version 1,
   PAN compression on, AR off.
2. `21 DC 01 FF FF 4F 3E 2D 1C 00 4B 12 00 2B 1A DD CC BB AA 00 4B 12 00`
   Data, seq 1, dest PAN 0xffff, dest extended `0x00124b001c2d3e4f`,
   source PAN 0x1a2b, source extended `0x00124b00aabbccdd`, AR on,
   PAN compression off, version 1.
3. `41 98 2a 2b` — truncated; must return a clear error, not panic.

Verify these against the standard rather than trusting them. If a field is
ambiguous, tell me what you think and why.

**Done when:**

- All three vectors produce the expected result.
- Malformed hex (odd digits, invalid characters, too short) returns an error.
- `cargo test`, `cargo fmt --check`, and clippy pass.
- You can explain each bit of `0x9841` out loud.

**Stretch:** support frame version 2 (2015), sequence-number suppression, and
the auxiliary security header (just report whether it is present and skip it).

### Step 2 — NWK + APS

**Build:** extend the dissector so a full frame prints NWK and APS fields:
destination/source, radius, sequence, endpoint, profile ID, cluster ID.

**Rust:** borrowed slices vs owned data, iterator chains, `TryFrom<&[u8]>`,
modelling "remaining bytes" without fighting the borrow checker.

**ZigBee:** NWK frame control and addressing, APS frame control, endpoint and
cluster multiplexing, the difference between MAC and NWK addresses.

**Done when:** a captured ZigBee frame from Wireshark/`tshark` decodes to the
same endpoint/cluster/profile that Wireshark shows.

### Step 3 — ZCL

**Build:** decode ZCL headers and a few clusters: On/Off (`0x0006`), Level
(`0x0008`), Temperature Measurement (`0x0402`). Handle Read Attributes,
Read Attributes Response, and Report Attributes.

**Rust:** traits for "decode a value", enums with payloads, maybe generics; the
start of a real type system for device data.

**ZigBee:** cluster/attribute/command model, ZCL data types, frame control
(global vs cluster-specific commands, direction, disable default response).

**Done when:** you can take a real report from a sensor and explain every byte.

### Step 4 — Serial framing

**Build:** a `Transport` that reads bytes from a serial port and yields frames.
First implement and test it against an in-memory reader; only then point it at
`/dev/ttyUSB0` (or `/dev/serial/by-id/...`).

**Rust:** `async`/`await`, `tokio`, splitting a stream, `mpsc` channels,
timeouts, cancellation, ownership across tasks.

**ZigBee:** why a host/NCP link needs framing and flow control; ASH concepts
(flags, CRC, sequence, ACK/NAK, retransmission).

**Done when:** a loopback test reassembles frames split at arbitrary byte
boundaries, and a real serial port opens without root.

### Step 5 — EZSP subset

**Build:** the smallest EZSP client that works: version negotiation, network
initialisation, and permit-join. Add commands one at a time.

**Rust:** actor pattern (one task owns the port; others send messages), request
correlation, state machines, good error context.

**ZigBee:** how a coordinator forms/restores a network and how commissioning
works. Read the EZSP reference guide; use the `ezsp`/`ashv2` crates as reference
implementations, not as dependencies, until you understand the layer.

**Done when:** `beehive permit-join 60` really opens the network.

### Step 6 — Devices

**Build:** handle device announcements, interview manufacturer/model/endpoints,
store a device table, send On/Off.

**Done when:** you can toggle a real light and explain the whole path from CLI
argument to radio byte.

---

## Resources

Start with the primary sources; blog posts are for orientation, not truth.

**Rust**

- *The Rust Programming Language* — <https://doc.rust-lang.org/book/>
- *Rust by Example* — <https://doc.rust-lang.org/rust-by-example/>
- Rustlings exercises — <https://github.com/rust-lang/rustlings>
- *Programming Rust* (Blandy, Orendorff, Tindall) — good once past the basics

**ZigBee / 802.15.4**

- IEEE 802.15.4 standard (MAC/PHY). The 2015/2020 editions; read the frame
  formats chapter closely.
- ZigBee Alliance/CSA specifications: ZigBee Base Device Behavior, ZigBee
  Cluster Library (ZCL), ZigBee specification. Free registration on the CSA
  site.
- *ZigBee Wireless Networks and Transceivers* — Shahin Farahani (older, but a
  clear fundamentals book).
- Wireshark's ZigBee dissectors: capture or open a `.pcap` and compare its
  field-by-field decoding with yours. `tshark -V` is excellent for this.
- Zigbee2MQTT docs — practical device/cluster behaviour.
- Reference implementations to read later: `zigbee-rs`, the `ezsp` and `ashv2`
  crates, and `zigbee2mqtt`.

**Hardware** (when we get there)

- See [docs/hardware.md](docs/hardware.md) for coordinator options, firmware,
  and Linux serial permissions.

---

## Keeping a portfolio trail

At each checkpoint:

1. Commit the working step.
2. Write `notes/step-NN.md`: what you built, the bytes you decoded, the Rust
   concept that clicked (or didn't), and one question you still have.
3. Add a short README section or screenshot if it is demoable.

By the end you will have a commit history that shows how you learned, not just
a finished binary. That is far more convincing in an interview than a repo you
cannot explain.
