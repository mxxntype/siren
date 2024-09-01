use clap::Parser;
use siren::cli::Options;
use std::net::UdpSocket;

fn main() {
    let options = Options::parse();

    let mac = siren::translate_mac(options.mac_address).unwrap();
    let packet = siren::build_wakeonlan_packet(&mac);

    dbg!(&packet);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to local address");
    socket.set_broadcast(true).unwrap();

    match socket.send_to(&packet, "255.255.255.255:9") {
        Ok(_) => println!("Magic packet sent to MAC: {mac:?}"),
        Err(error) => println!("Failed to send: {error:?}"),
    }
}
