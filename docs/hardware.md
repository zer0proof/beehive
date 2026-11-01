# Hardware notes

## Supported coordinators (planned)

The `ezsp` backend targets **Silicon Labs EmberZNet** NCPs speaking EZSP over
ASHv2. Typical options:

| Device | Chip | Notes |
|--------|------|-------|
| Sonoff ZBDongle-E | EFR32MG21 | Ships with EZSP coordinator firmware; 115200 or 460800 baud. |
| SLZB-06 / SLZB-06M | CC2652P / EFR32 | Can run in EZSP mode; check the firmware mode. |
| Generic CC2652P USB stick | CC2652P | Must run **EZSP coordinator** firmware (not ZNP). |
| Sonoff ZBDongle-P | CC2652P | Often flashed with Z-Stack/ZNP firmware — **not** supported by this backend. |

Not supported by `beehive-ezsp`:

- **CC2531 / CC2530** sticks running Z-Stack (ZNP protocol).
- Devices that only expose Zigbee2MQTT's MQTT or HTTP interface.

Those need a different backend (e.g. `beehive-znp`) or a different architecture.

## Firmware

The NCP must run coordinator firmware. For flashing EZSP coordinator firmware,
the usual community tools are:

- `universal-silabs-flasher` (Python) for Silabs/EFR32 and CC2652 sticks.
- `flash_cc2531` only if you are deliberately using ZNP hardware.

Record the exact stick, chip, firmware version, and baud rate in `CONTEXT.md`
once known — it matters for debugging.

## Serial access on Linux (Arch)

1. Find the device:

   ```sh
   ls -l /dev/serial/by-id/
   dmesg | tail -20
   ```

   Prefer the stable `/dev/serial/by-id/...` path over `/dev/ttyUSB0` in
   `config/beehive.toml`.

2. Check the owning group:

   ```sh
   ls -l /dev/ttyUSB0
   ```

   On Arch the group is usually **`uucp`**. Add yourself and re-login:

   ```sh
   sudo usermod -aG uucp "$USER"
   ```

3. Verify access without root:

   ```sh
   test -r /dev/ttyUSB0 && test -w /dev/ttyUSB0 && echo "ok"
   ```

## Baud rate

EZSP NCPs commonly use **115200**. Some flashed sticks use **460800** or
**230400**. If `start()` fails during negotiation, try the other rates. The
value lives under `[ezsp] baud` in the config file.

## Troubleshooting checklist

- Permission denied → group membership / udev rule.
- Port disappears → flaky USB cable or hub; use a powered hub.
- Handshake fails → wrong baud, wrong firmware mode (ZNP instead of EZSP), or
  the stick is in bootloader mode.
- Network not restored → NCP may have been reflashed and lost its keys; check
  `ezsp` restore/network-init flow.

## Safety

- Do not run two coordinators/hosts against the same stick at once.
- Back up network keys before reflashing.
- ZigBee is a radio: keep the stick away from USB 3.0 noise if possible, or use
  an extension cable.
