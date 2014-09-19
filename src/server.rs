extern crate piston;
extern crate string_telephone;
extern crate collections;

use collections::str::{Slice, Owned};
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use piston::{EventIterator, EventSettings, WindowSettings, NoWindow};
use string_telephone::{Server, ConnectionConfig, UserPacket, Command, PacketDisconnect, PacketConnect};
use std::fmt;
use std::rand;
use std::rand::Rng;

struct Player {
    controller: SocketAddr,
    x: i32,
    y: i32,

    keyUp: bool,
    keyDown: bool,
    keyLeft: bool,
    keyRight: bool
}

impl Player {
    pub fn new (controller: SocketAddr, x: i32, y: i32) -> Player {
        Player {
            controller: controller,
            x: x,
            y: y,
            keyUp: false,
            keyDown: false,
            keyLeft: false,
            keyRight: false
        }
    }

    pub fn move(&mut self) {
        if self.keyUp && self.y > 0 {
            self.y -= 1;
        } else if self.keyDown && self.y < 600 {
            self.y += 1;
        }

        if self.keyLeft && self.x > 0 {
            self.x -= 1;
        } else if self.keyRight && self.x < 800{
            self.x += 1;
        }
    }
}

impl fmt::Show for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {} {}", self.controller, self.x, self.y)
    }
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

    let mut players:Vec<Player> = vec![];
    let mut rng = rand::task_rng();

    for e in EventIterator::new(&mut window, &game_iter_settings) {
        match e {
            piston::Update(update_args) => {
                loop {
                    match server.poll() {
                        Some((Command(PacketConnect), addr_from)) => {
                            println!("{} connected", addr_from);
                            players.push(Player::new(addr_from, (rng.gen::<u32>() % 800) as i32, (rng.gen::<u32>() % 600) as i32));
                        },
                        Some((Command(PacketDisconnect), addr_from)) => {
                            println!("{} disconnected", addr_from);
                            players = players.into_iter().filter(|&player| &player.controller != &addr_from).collect()
                        },
                        Some((UserPacket(packet), _)) => {
                            //Do something
                        },
                        Some(_) => (),
                        None => break
                    }
                };

                //Update the game world
                for player in players.iter_mut() {
                    player.move()
                }
                println!("{}", players);
                
                let culled = server.cull();
                if culled.len() > 0 {
                    println!("{} timed out", culled);
                    players = players.into_iter().filter(|&player| culled.contains(&player.controller) == false).collect()
                }
            },
            _ => ()
        }
    }
}
