extern crate collections;
extern crate string_telephone;

use collections::str::{Slice, Owned};
use std::io::net::ip::{Ipv4Addr, SocketAddr};
//use piston::{EventIterator, EventSettings, WindowSettings, NoWindow};
use string_telephone::{Client, ConnectionConfig, UserPacket, Command, PacketDisconnect, PacketConnect};

fn deserializer(message: &Vec<u8>) -> String {
    match String::from_utf8_lossy(message.as_slice()) {
        Slice(slice) => slice.to_string(),
        Owned(item) => item
    }
}

fn serializer(packet: &String) -> Vec<u8> {
    packet.clone().into_bytes()
}

fn main() {
    let settings = ConnectionConfig {
        protocol_id: 88869,
        timeout_period: 10,
        packet_deserializer: deserializer,
        packet_serializer: serializer
    };

    let mut client = match Client::connect(SocketAddr {ip: Ipv4Addr(0, 0, 0, 0), port: 0}, SocketAddr {ip: Ipv4Addr(127, 0, 0, 1), port: 8869}, settings) {
        Ok(client) => {
            client
        },
        Err(e) => fail!("Failed to connect - {}", e)
    };
}
