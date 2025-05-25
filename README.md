# serial-port-finder-rs

Basic serial port finder that returns information of the device,
if found based on the set vid and pid.
To get vid/pid on linux use lsusb

## Build

```bash
cargo build --release
```

## Run

In the root directory for the project:

```bash
cd target/release
```

To run:

```bash
serial-port-finder --vid 2341 --pid 0042
```

In the above example if an arduino Mega is plugged into the system via usb:

```bash
Matching device found:
    Device path: /dev/ttyACM0
    VID: 2341
    PID: 0042
    Serial Number: 5513931353535130E011
    Manufacturer: Arduino (www.arduino.cc)
    Product: 0042
```
