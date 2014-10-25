#![feature(phase)]

#[phase(plugin, link)]
extern crate conrod;
extern crate piston;
extern crate collections;
extern crate string_telephone;
extern crate opengl_graphics;
extern crate sdl2_game_window;

use opengl_graphics::Gl;
use conrod::UiContext;

use piston::{EventIterator, EventSettings, WindowSettings, AssetStore};
use string_telephone::ConnectionConfig;
use sdl2_game_window::WindowSDL2;

pub mod scene;
pub mod connectscene;
pub mod gamestate;
pub mod gamescene;
pub mod packet;
pub mod player;

fn main() {

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

    let mut gamestate = gamestate::GameState::new(UiContext::new("Dense-Regular.otf", None), Gl::new(opengl), &AssetStore::from_folder("../assets"));
    let mut current_scene = connectscene::ConnectScene::new();

    for ref e in EventIterator::new(&mut window, &event_settings) {
        gamestate.get_uic().handle_event(e);
        match current_scene.handle_event(e, &mut gamestate) {
            Some(scene) => current_scene = scene,
            None => ()
        };
    }
}
