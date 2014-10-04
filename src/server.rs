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

    let mut next_broadcast = 0f64;
    let mut next_think = 0f64;
    let mut clock = 0f64;
    //let clock_rate = 0.0015;
    let broadcast_rate = 1.0 / 20.0;

    for e in EventIterator::new(&mut window, &game_iter_settings) {
        match e {
            piston::Update(update_args) => {
                clock += update_args.dt;
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
                        Some((UserPacket(packet::MovePacket(up, down, left, right)), addr_from)) => {
                            for &(controller, ref mut player) in players.iter_mut() {
                                if controller == addr_from {
                                    player.key_up = up;
                                    player.key_down = down;
                                    player.key_left = left;
                                    player.key_right = right;
                                    break;
                                }
                            };
                            //TODO: Respond to movement events
                            //Do something
                        },
                        Some(_) => (),
                        None => break
                    }
                };

                //Update the game world
                if clock >= next_think {
                    next_think = clock + 0.015;
                    for &(_, ref mut player) in players.iter_mut() {
                        player.think()
                    }
                }
                
                let culled = server.cull();
                if culled.len() > 0 {
                    println!("{} timed out", culled);
                    players = players.into_iter().filter(|&(player, _)| culled.contains(&player) == false).collect()
                }

                if clock >= next_broadcast {
                    next_broadcast = clock + broadcast_rate;
                    println!("Heartbeat! {}", clock);
                    let serialized_state: Vec<player::Player> = players.iter().map(|&(_, data)| data).collect();
                    for &(ref user, ref data) in players.iter() {
                        server.send_to(&packet::FullServerState(data.id, serialized_state.clone()), user);
                    }
                }
            },
            _ => ()
        }
    }
}
