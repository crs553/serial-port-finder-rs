use clap::Parser;
use serialport::{SerialPortType, available_ports};

/// Tool to find serial ports by USB VID/PID
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// USB Vendor ID (hex), e.g. 0x2341
    #[arg(long, value_parser = parse_hex_u16)]
    vid: u16,

    /// USB Product ID (hex), e.g. 0x0043
    #[arg(long, value_parser = parse_hex_u16)]
    pid: u16,
}

fn parse_hex_u16(s: &str) -> Result<u16, String> {
    u16::from_str_radix(s.trim_start_matches("0x"), 16)
        .map_err(|_| format!("Invalid hex u16: {}", s))
}

fn main() {
    let args = Args::parse();
    let target_vid = args.vid;
    let target_pid = args.pid;

    match available_ports() {
        Ok(mut ports) => {
            ports.sort_by_key(|i| i.port_name.clone());
            let mut found = false;

            for p in ports {
                if let SerialPortType::UsbPort(info) = &p.port_type {
                    if info.vid == target_vid && info.pid == target_pid {
                        println!("Matching device found:");
                        println!("    Device path: {}", p.port_name);
                        println!("    VID: {:04x}", info.vid);
                        println!("    PID: {:04x}", info.pid);
                        println!(
                            "    Serial Number: {}",
                            info.serial_number.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "    Manufacturer: {}",
                            info.manufacturer.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "    Product: {}",
                            info.product.as_ref().map_or("", String::as_str)
                        );
                        found = true;
                    }
                }
            }

            if !found {
                println!(
                    "No matching device found for VID 0x{:04x} and PID 0x{:04x}",
                    target_vid, target_pid
                );
            }
        }
        Err(e) => {
            eprintln!("Error listing serial ports: {:?}", e);
        }
    }
}
