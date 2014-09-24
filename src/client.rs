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
    label,
    Color,
    Point,
    widget_matrix,
    Button,
    Callable,
    Colorable,
    Drawable,
    DropDownList,
    EnvelopeEditor,
    Frameable,
    Labelable,
    NumberDialer,
    Positionable,
    Slider,
    Shapeable,
    TextBox,
    Toggle,
    XYPad,
};

use collections::str::{Slice, Owned};
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use piston::{EventIterator, EventSettings, WindowSettings, graphics, Render, Event};
use piston::graphics::{AddColor, Draw};
use string_telephone::{Client, ConnectionConfig, UserPacket, Command, PacketDisconnect, PacketConnect};
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
    let ref mut gl = Gl::new(opengl);

    let ref mut uic = UIContext::new("Dense-Regular.otf");

    let mut edit_ip = vec!["127".to_string(), "0".to_string(), "0".to_string(), "1".to_string()];

    let mut manager = scene::SceneManager::new(box connectscene::ConnectScene::<gamestate::GameState>);

    for ref e in EventIterator::new(&mut window, &event_settings) {
        uic.handle_event(e);
        match e {
            &Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = &graphics::Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                label::draw(
                    gl,
                    uic,
                    Point::new(0f64, 0f64, 0f64), // Screen position.
                    48u32, // Font size.
                    Color::new(1.0, 0.0, 0.0, 1.0),
                    "Select a server"
                );

                widget_matrix::draw(
                    4, // cols.
                    1, // rows.
                    Point::new(0.0, 56.0, 0.0), // matrix position.
                    300.0, // width.
                    240.0, // height.
                    |num, col, row, pos, width, height| { // This is called for every widget.
                        // Now draw the widgets with the given callback.
                        uic.text_box(2 + num as u64, edit_ip.get_mut(num))
                            .font_size(24u32)
                            .dimensions(width, 36.0)
                            .position(pos.x, pos.y)
                            .frame(2.0, Color::black())
                            .color(Color::black())
                            .draw(gl);

                    }
                );

                uic.button(7u64)
                    .dimensions(90.0, 36.0)
                    .position(0.0, 128.0)
                    .color(Color::black())
                    .frame(2.0, Color::black())
                    .label("Connect", 24u32, Color::white())
                    .callback(|| () )
                    .draw(gl);

            },
            _ => {},
        }
    }
}
