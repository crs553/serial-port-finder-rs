# USB Serial Port Finder 🔌

A small Rust CLI tool that lists and identifies USB serial ports
matching a specific Vendor ID (VID) and Product ID (PID).

This is especially useful for identifying devices like Arduinos,
embedded boards,or USB serial adapters connected to your system.

## Features

Filter the available ports on your system using vid and pid
Returns a detailed display of:

- Device path
- vid
- pid
- Serial Number
- Manufacturer
- Product

Basic serial port finder that returns information of the device,
if found based on the set vid and pid.
To get vid/pid on linux use lsusb

## Build

```bash
    cargo build --release
```

## Example use case

In the root directory for the project:

```bash
    cd target/release
```

To run:

```bash
    serial-port-finder --vid 2341 --pid 0042
```

In the above example if an arduino Mega is plugged into the system via usb,
the following will be returned:

```bash
    Matching device found:
        Device path: /dev/ttyACM0
        VID: 2341
        PID: 0042
        Serial Number: 5513931353535130E011
        Manufacturer: Arduino (www.arduino.cc)
        Product: 0042
```
