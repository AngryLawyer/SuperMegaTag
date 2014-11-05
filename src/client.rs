#![feature(phase)]

#[phase(plugin, link)]
extern crate conrod;
extern crate event;
extern crate input;
extern crate collections;
extern crate string_telephone;
extern crate opengl_graphics;
extern crate sdl2_window;
extern crate shader_version;
extern crate graphics;

use opengl_graphics::Gl;
use conrod::UiContext;

use event::{Events, WindowSettings};
use sdl2_window::Sdl2Window;
use std::cell::RefCell;

pub mod scene;
pub mod connectscene;
pub mod gamestate;
pub mod gamescene;
pub mod packet;
pub mod player;

fn main() {

    let opengl = shader_version::opengl::OpenGL_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "SuperMegaTag".to_string(),
            size: [800, 600],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );
    let window = RefCell::new(window);

    let mut gamestate = gamestate::GameState::new(UiContext::new("Dense-Regular.otf", None), Gl::new(opengl), &Path::new("../assets"));
    let mut current_scene = connectscene::ConnectScene::new();

    for ref e in Events::new(&window) {
        gamestate.get_uic().handle_event(e);
        match current_scene.handle_event(e, &mut gamestate) {
            Some(scene) => current_scene = scene,
            None => ()
        };
    }
}
