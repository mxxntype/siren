use clap::Parser;
use color_eyre::owo_colors::OwoColorize;
use siren::{cli::Options, Mac, Packet};
use std::net::UdpSocket;

fn main() {
    let _ = color_eyre::install()
        .inspect_err(|error| eprintln!("Couldn't install color_eyre: {error:?}"));
    let options = Options::parse();

    let mac = options.mac_address.as_str().parse::<Mac>().unwrap();
    let packet = Packet::new_with_mac(&mac);
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to local address");
    socket
        .set_broadcast(true)
        .expect("Could not set SO_BROADCAST");

    for addr in ["255.255.255.255:7", "255.255.255.255:9"] {
        match socket.send_to(packet.as_bytes(), addr) {
            Err(error) => println!("Couldn't send packet: {error:?}"),
            Ok(_) => println!(
                "Sent Wake-on-LAN packet with MAC {} to {addr}",
                options.mac_address.bold()
            ),
        }
    }
}
