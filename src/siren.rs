use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;
use color_eyre::{eyre::Report, owo_colors::OwoColorize};
use siren::{Mac, Packet};
use std::net::UdpSocket;

#[derive(Parser, Clone)]
#[command(version, author, about)]
#[command(styles(Styles::styled()
    .usage(AnsiColor::Magenta.on_default()  | Effects::BOLD)
    .header(AnsiColor::Magenta.on_default() | Effects::BOLD)))]
struct Options {
    /// The MAC address of the device that needs to be waken up.
    pub mac_address: String,
}

/// The Wake-on-LAN magic packet is usually broadcasted to ports 0, 7 and 9.
/// For some reason, can't send packets to port 0, so only 7 and 9 for now.
// TODO: Figure out what's with port 0.
const COMMON_WOL_TARGETS: [&str; 2] = ["255.255.255.255:7", "255.255.255.255:9"];

fn main() -> Result<(), Report> {
    let _ = color_eyre::install()
        .inspect_err(|error| eprintln!("Couldn't install color_eyre: {error:?}"));
    let options = Options::parse();
    let mac = options.mac_address.parse::<Mac>()?;
    let packet = Packet::new_with_mac(&mac);
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    for addr in COMMON_WOL_TARGETS {
        match socket.send_to(packet.as_bytes(), addr) {
            Err(error) => println!("Couldn't send packet: {error:?}"),
            Ok(total_bytes_sent) => println!(
                "Sent Wake-on-LAN packet with MAC {mac} to {addr} (Total {n} bytes)",
                mac = options.mac_address.bold(),
                n = total_bytes_sent
            ),
        }
    }

    Ok(())
}
