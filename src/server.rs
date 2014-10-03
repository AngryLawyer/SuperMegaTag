extern crate piston;
extern crate string_telephone;
extern crate collections;

use std::io::net::ip::{Ipv4Addr, SocketAddr};
use piston::{EventIterator, EventSettings, WindowSettings, NoWindow};
use string_telephone::{Server, ConnectionConfig, UserPacket, Command, PacketDisconnect, PacketConnect};
use std::rand;
use std::rand::Rng;

pub mod packet;
pub mod player;

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
        packet_deserializer: packet::deserializer,
        packet_serializer: packet::serializer
    };

    let mut server = match Server::new(SocketAddr {ip: Ipv4Addr(0, 0, 0, 0), port: 8869}, settings) {
        Ok(server) => {
            println!("Listening on {}", server.addr);
            server
        },
        Err(e) => fail!("Failed to start listening - {}", e)
    };

    let mut players:Vec<(SocketAddr, player::Player)> = vec![];
    let mut rng = rand::task_rng();
    let mut player_counter = 0;

    for e in EventIterator::new(&mut window, &game_iter_settings) {
        match e {
            piston::Update(update_args) => {
                loop {
                    match server.poll() {
                        Some((Command(PacketConnect), addr_from)) => {
                            println!("{} connected", addr_from);
                            //TODO: Check this player doesn't already exist
                            players.push((addr_from, player::Player::new(player_counter, (rng.gen::<u32>() % 800) as i32, (rng.gen::<u32>() % 600) as i32)));
                            player_counter += 1;
                        },
                        Some((Command(PacketDisconnect), addr_from)) => {
                            println!("{} disconnected", addr_from);
                            players = players.into_iter().filter(|&(controller, _)| &controller != &addr_from).collect()
                        },
                        Some((UserPacket(packet), _)) => {
                            //Do something
                        },
                        Some(_) => (),
                        None => break
                    }
                };

                //Update the game world
                for &(_, ref mut player) in players.iter_mut() {
                    player.think()
                }
                println!("{}", players);
                
                let culled = server.cull();
                if culled.len() > 0 {
                    println!("{} timed out", culled);
                    players = players.into_iter().filter(|&(player, _)| culled.contains(&player) == false).collect()
                }
            },
            _ => ()
        }
    }
}
