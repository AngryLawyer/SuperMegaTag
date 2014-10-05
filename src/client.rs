#![feature(phase)]

#[phase(plugin, link)]
extern crate conrod;
extern crate piston;
extern crate collections;
extern crate string_telephone;
extern crate opengl_graphics;
extern crate sdl2_game_window;

use opengl_graphics::Gl;
use conrod::UIContext;

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

    let mut manager = scene::SceneManager::new();
    let mut gamestate = gamestate::GameState::new(UIContext::new("Dense-Regular.otf"), Gl::new(opengl), &AssetStore::from_folder("../assets"));
    manager.set_scene(|manager| { box connectscene::ConnectScene::new(manager) });

    for ref e in EventIterator::new(&mut window, &event_settings) {
        gamestate.get_uic().handle_event(e);
        manager.handle_event(e, &mut gamestate);
    }
}
