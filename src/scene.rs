use piston::{Event, Update};
use gamestate::GameState;

pub trait Scene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<Box<Scene + 'static>>;
}
