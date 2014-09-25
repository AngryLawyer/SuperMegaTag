#![feature(phase)]

#[phase(plugin, link)]
extern crate conrod;
extern crate piston;
extern crate collections;
extern crate string_telephone;
extern crate opengl_graphics;
extern crate sdl2_game_window;

use opengl_graphics::{
    Gl
};
use conrod::UIContext;

use collections::str::{Slice, Owned};
use piston::{EventIterator, EventSettings, WindowSettings};
use string_telephone::ConnectionConfig;
use sdl2_game_window::WindowSDL2;

pub mod scene;
pub mod connectscene;
pub mod gamestate;

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

    /*let mut client = match Client::connect(SocketAddr {ip: Ipv4Addr(0, 0, 0, 0), port: 0}, SocketAddr {ip: Ipv4Addr(127, 0, 0, 1), port: 8869}, settings) {
        Ok(client) => {
            client
        },
        Err(e) => fail!("Failed to connect - {}", e)
    };*/

    let opengl = piston::shader_version::opengl::OpenGL_3_2;
    let mut window = WindowSDL2::new(
        opengl,
        WindowSettings {
            title: "SuperMegaTag".to_string(),
            size: [800, 600],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );

    let event_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };

    let mut manager = scene::SceneManager::new(box connectscene::ConnectScene::<gamestate::GameState>::new());
    let mut gamestate = gamestate::GameState::new(UIContext::new("Dense-Regular.otf"), Gl::new(opengl));

    for ref e in EventIterator::new(&mut window, &event_settings) {
        gamestate.get_uic().handle_event(e);
        manager.handle_event(e, &mut gamestate);
    }
}
