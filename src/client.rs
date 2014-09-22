#![feature(phase)]

#[phase(plugin, link)]
extern crate conrod;
extern crate piston;
extern crate collections;
extern crate string_telephone;
extern crate opengl_graphics;
extern crate sdl2_game_window;

use opengl_graphics::{
    Gl,
    Texture,
};

use conrod::{
    UIContext,
};

use collections::str::{Slice, Owned};
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use piston::{EventIterator, EventSettings, WindowSettings, graphics, Render};
use piston::graphics::{AddColor, Draw};
use string_telephone::{Client, ConnectionConfig, UserPacket, Command, PacketDisconnect, PacketConnect};
use sdl2_game_window::WindowSDL2;

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

    let opengl = piston::shader_version::opengl::OpenGL_3_2;
    let mut window = WindowSDL2::new(
        opengl,
        WindowSettings {
            title: "SuperMegaTag".to_string(),
            size: [800, 600],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let event_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let ref mut gl = Gl::new(opengl);

    let mut uic = UIContext::new("Dense-Regular.otf");
    for e in EventIterator::new(&mut window, &event_settings) {
        match e {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = graphics::Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
            },
            _ => {},
        }
    }
}
