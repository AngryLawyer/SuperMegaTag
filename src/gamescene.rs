use scene::{Scene, SceneManager};
use gamestate::GameState;
use piston::{graphics, Render, Event, Update};
use piston::graphics::{AddColor, Draw};

pub struct GameScene<'r> {
    manager: &'r mut SceneManager
}

impl <'r> GameScene<'r> {

    pub fn new<'r>(manager: &'r mut SceneManager) -> GameScene<'r> {
       GameScene {
           manager: manager,
       }
    }
}

impl <'r> Scene for GameScene <'r> {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) {
        match e {
            &Render(args) => {
                let gl = state.get_gl();

                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = &graphics::Context::abs(args.width as f64, args.height as f64);


                c.rgb(1.0, 1.0, 1.0).draw(gl);
            },
            _ => ()
        }
    }
}
