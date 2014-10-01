use scene::{Scene, SceneManager};
use gamestate::GameState;
use piston::{graphics, Render, Event, Update};

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
    }
}
