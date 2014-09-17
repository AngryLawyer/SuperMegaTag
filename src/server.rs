extern crate piston;
extern crate string_telephone;
extern crate collections;

use collections::str::{Slice, Owned};
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use piston::{EventIterator, EventSettings, WindowSettings, NoWindow};
use string_telephone::{Server, ConnectionConfig, UserPacket};

struct Player {
    controller: SocketAddr,
    x: uint,
    y: uint
}

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
    let mut window = NoWindow::new(WindowSettings {
        title: "SuperMegaTag".to_string(),
        size: [0u32, 0u32],
        samples: 0,
        fullscreen: false,
        exit_on_esc: true,
    });

    let game_iter_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };

    let settings = ConnectionConfig {
        protocol_id: 88869,
        timeout_period: 10,
        packet_deserializer: deserializer,
        packet_serializer: serializer
    };

    let mut server = match Server::new(SocketAddr {ip: Ipv4Addr(0, 0, 0, 0), port: 8869}, settings) {
        Ok(server) => {
            println!("Listening on {}", server.addr);
            server
        },
        Err(e) => fail!("Failed to start listening - {}", e)
    };

    for e in EventIterator::new(&mut window, &game_iter_settings) {
        loop {
            match server.poll() {
                Some((UserPacket(packet), _)) => {
                    //Do something
                },
                Some(_) => (),
                None => break
            }
        };

        let culled = server.cull();
        if culled.len() > 0 {
            println!("{}", culled);
        }
    }
}
