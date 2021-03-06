extern crate event;
extern crate string_telephone;
extern crate collections;

use std::io::net::ip::{Ipv4Addr, SocketAddr};
use event::{Events, WindowSettings, NoWindow, Update};
use string_telephone::{Server, ConnectionConfig, UserPacket, Command, PacketDisconnect, PacketConnect};
use std::rand;
use std::rand::{Rng, TaskRng};
use std::time::duration::Duration;
use std::cell::RefCell;

pub mod packet;
pub mod player;

fn update_tagged_player_validity(rng: &mut TaskRng, players: &Vec<(SocketAddr, player::Player)>, current_tagged: u16) -> u16 {
    if players.len() == 0 {
        current_tagged
    } else {
        let tagged_count = players.iter().filter(|&&(_, player)| &player.id == &current_tagged).count();
        if tagged_count == 0 {
            let index = rng.gen::<uint>() % players.len();
            let (_, player) = players[index];
            player.id
        } else {
            current_tagged
        }
    }
}

fn main() {
    let window = NoWindow::new(WindowSettings {
        title: "SuperMegaTag".to_string(),
        size: [0u32, 0u32],
        samples: 0,
        fullscreen: false,
        exit_on_esc: true,
    });
    let window = RefCell::new(window);

    let settings = ConnectionConfig {
        protocol_id: 88869,
        timeout_period: Duration::seconds(10),
        packet_deserializer: packet::deserializer,
        packet_serializer: packet::serializer
    };

    let mut server = match Server::new(SocketAddr {ip: Ipv4Addr(0, 0, 0, 0), port: 8869}, settings) {
        Ok(server) => {
            println!("Listening on {}", server.addr);
            server
        },
        Err(e) => panic!("Failed to start listening - {}", e)
    };

    let mut players:Vec<(SocketAddr, player::Player)> = vec![];
    let mut rng = rand::task_rng();
    let mut player_counter = 0;

    let mut next_broadcast = 0f64;
    let mut next_think = 0f64;
    let mut clock = 0f64;
    //let clock_rate = 0.0015;
    let broadcast_rate = 1.0 / 20.0;
    let mut tagged_player = 0;

    for e in Events::new(&window) {
        match e {
            Update(update_args) => {
                clock += update_args.dt;
                loop {
                    match server.poll() {
                        Some((Command(PacketConnect), addr_from)) => {
                            match players.iter().find(|&&(socket, _)|{ socket == addr_from}) {
                                Some(_) => {
                                    println!("{} tried to connect a second time", addr_from);
                                },
                                None => {
                                    println!("{} connected", addr_from);
                                    players.push((addr_from, player::Player::new(player_counter, (rng.gen::<u32>() % 800) as i32, (rng.gen::<u32>() % 600) as i32)));
                                    tagged_player = update_tagged_player_validity(&mut rng, &players, tagged_player);
                                    player_counter += 1;
                                }
                            }
                        },
                        Some((Command(PacketDisconnect), addr_from)) => {
                            println!("{} disconnected", addr_from);
                            players = players.into_iter().filter(|&(controller, _)| &controller != &addr_from).collect();
                            tagged_player = update_tagged_player_validity(&mut rng, &players, tagged_player);
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
                        },
                        Some(_) => (),
                        None => break
                    }
                };

                //Update the game world
                if clock >= next_think {
                    next_think = clock + 0.015;
                    //FIXME: There must be a better way of doing comparisons
                    let comparator_players = players.clone();
                    let mut pass_on_tag = None;

                    for &(_, ref mut player) in players.iter_mut() {
                        if !player.is_frozen(clock) {
                            let mut collided = player.id;
                            for &(_, ref other_player) in comparator_players.iter() {
                                if player.id != other_player.id {
                                    if player.will_collide(other_player) {
                                        collided = other_player.id;
                                        break;
                                    }
                                }
                            }
                            if collided == player.id {
                                player.think()
                            } else if player.id == tagged_player {
                                pass_on_tag = Some(collided)
                            }
                        }
                    };

                    match pass_on_tag {
                        Some(id) => {
                            for &(_, ref mut player) in players.iter_mut() {
                                if player.id == id {
                                    player.freeze(clock);
                                    break;
                                }
                            };
                            tagged_player = id;
                        },
                        None => ()
                    }
                }
                
                let culled = server.cull();
                if culled.len() > 0 {
                    println!("{} timed out", culled);
                    players = players.into_iter().filter(|&(player, _)| culled.contains(&player) == false).collect();
                    tagged_player = update_tagged_player_validity(&mut rng, &players, tagged_player);
                }

                if clock >= next_broadcast {
                    next_broadcast = clock + broadcast_rate;
                    let serialized_state: Vec<player::Player> = players.iter().map(|&(_, data)| data).collect();
                    for &(ref user, ref data) in players.iter() {
                        server.send_to(&packet::FullServerState(data.id, tagged_player, serialized_state.clone()), user);
                    }
                }
            },
            _ => ()
        }
    }
}
