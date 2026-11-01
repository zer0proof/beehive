# Hardware notes — TI CC2531

The target hardware is a **Texas Instruments CC2531 USB dongle** running
**Z-Stack coordinator firmware**. That firmware exposes the **ZNP / MT**
(ZigBee Network Processor / Monitor-and-Test) protocol over a USB CDC serial
port.

This is a different protocol from EZSP/ASHv2 (Silicon Labs). Anything in the
archived `reference/scaffold` branch that talks EZSP does **not** apply to this
hardware; it is only a Rust-structure reference.

## What the CC2531 gives you

| Property | Value |
|----------|-------|
| Radio | 2.4 GHz IEEE 802.15.4 |
| MCU | TI CC2531 (8051 core, USB) |
| Coordinator stack | Z-Stack Home 1.2 (typical) |
| Host protocol | ZNP / MT over USB CDC serial |
| Serial device | usually `/dev/ttyACM0` (verify) |
| Serial settings | 115200 baud, 8N1, **no** RTS/CTS |
| Antenna | PCB antenna (short range) |
| Direct children | 20 with the default Home 1.2 firmware |

### Honest limitations

Zigbee2MQTT lists the CC2531 as **not recommended** for production networks:
it needs a CC Debugger to flash, it has poor range, and it is not powerful
enough for networks of roughly 20+ devices. None of that matters for learning,
and it is a great target for understanding ZNP because the protocol is simple.

If you have more than one CC2531, keep one as the coordinator and flash another
with **sniffer firmware**. A second stick capturing over-the-air frames is
extremely useful for Steps 1–3.

## Firmware

Use the **Z-Stack Home 1.2** coordinator firmware, not Z-Stack 3.0.x (the
CC2531 is generally considered too small for 3.0).

- Firmware collection:
  <https://github.com/Koenkk/Z-Stack-firmware>
- Default firmware (1–15 devices):
  `coordinator/Z-Stack_Home_1.2/bin/default/CC2531_DEFAULT_20211115.zip`
- Source-routing firmware (larger networks, but only 5 direct children):
  `coordinator/Z-Stack_Home_1.2/bin/source_routing/`

Useful direct link from the Zigbee2MQTT flashing guide:
`https://github.com/Koenkk/Z-Stack-firmware/raw/Z-Stack_Home_1.2_20211115/20211116/coordinator/Z-Stack_Home_1.2/bin/default/CC2531_DEFAULT_20211115.zip`

### Flashing

The CC2531 needs a **CC Debugger** (or clone) plus a **CC2531 downloader
cable**, or one of the alternative methods (Raspberry Pi, Arduino, ESP8266/32)
described in the Zigbee2MQTT guide:

- <https://www.zigbee2mqtt.io/guide/adapters/flashing/flashing_the_cc2531.html>

Some sticks arrive pre-flashed with coordinator firmware. Check before buying a
debugger: plug it in and look for a serial device. If it enumerates but does not
answer ZNP commands, it may still be running router or sniffer firmware.

Record the exact firmware name/version in `CONTEXT.md` once known. Step 5's
`SYS_VERSION` request will report the stack version.

## Serial access on Linux (Arch)

1. Plug in the stick and find the stable path:

   ```sh
   ls -l /dev/serial/by-id/
   dmesg | tail -20
   ```

   The CC2531 usually appears as `/dev/ttyACM0`. Prefer the
   `/dev/serial/by-id/...` path once we add configuration.

2. Check the owning group:

   ```sh
   ls -l /dev/ttyACM0
   ```

   On Arch the group is usually **`uucp`**:

   ```sh
   sudo usermod -aG uucp "$USER"
   ```

   Log out and back in, then verify:

   ```sh
   test -r /dev/ttyACM0 && test -w /dev/ttyACM0 && echo ok
   ```

3. Open the port with **115200 8N1, no flow control**.

If the device does not appear as `/dev/ttyACM*`, it is probably not running
coordinator firmware, or the USB cable/port is bad.

## ZNP / MT frame format (preview)

This is what Step 4 will implement. A host↔NCP frame is:

```text
SOF(0xFE) | Len | Cmd0 | Cmd1 | Data[Len] | FCS
```

- `Len` is the number of data bytes (not counting the header or FCS).
- `Cmd0` packs the type and subsystem: `(type << 5) | subsystem`.
- `Cmd1` is the command ID within that subsystem.
- `FCS` is the XOR of every byte from `Len` through the end of `Data`
  (i.e. everything after `SOF`).
- Minimum frame length is 5 bytes.

Types: `POLL = 0`, `SREQ = 1`, `AREQ = 2`, `SRSP = 3`.

Common subsystems: `SYS = 1`, `MAC = 2`, `NWK = 3`, `AF = 4`, `ZDO = 5`,
`SAPI = 6`, `UTIL = 7`, `APP_CNF = 15`.

We will derive and test this ourselves rather than taking it on faith. A good
reference implementation to compare against later is Zigbee2MQTT's
`zigbee-herdsman` Z-Stack adapter:
<https://github.com/Koenkk/zigbee-herdsman/tree/master/src/adapter/z-stack>

## Troubleshooting checklist

- **Permission denied** → group membership / udev rule.
- **No `/dev/ttyACM*`** → wrong firmware, bad cable, or a charge-only USB port.
- **Port appears then disappears** → flaky cable/hub; try a powered hub or a
  short cable.
- **No response to ZNP frames** → wrong baud, wrong firmware (router/sniffer
  instead of coordinator), or the stick is in bootloader mode after flashing.
- **Bad range** → use a USB extension cable to move the stick away from the
  machine and USB 3.0 noise.

## Safety

- Do not run two hosts against the same coordinator at once.
- Back up network keys before reflashing; reflashing coordinator firmware
  usually wipes them.
- ZigBee is a radio: keep the stick away from USB 3.0 ports if possible.
