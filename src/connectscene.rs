use piston::{Event};
use scene::Scene;
use gamestate::GameState;

pub struct ConnectScene<T>;

impl <T> Scene<GameState> for ConnectScene <T> {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) {
    }
}
