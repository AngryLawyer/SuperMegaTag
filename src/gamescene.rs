use scene::{Scene, SceneManager};
use gamestate::GameState;
use piston::{graphics, Render, Event, Update};

pub struct GameScene<'r, T:'r> {
    manager: &'r mut SceneManager<T>
}

impl <'r, T> GameScene<'r, T> {

    pub fn new<'r>(manager: &'r mut SceneManager<T>) -> GameScene<'r, T> {
       GameScene {
           manager: manager,
       }
    }
}

impl <'r, T> Scene<GameState> for GameScene <'r, T> {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) {
    }
}
