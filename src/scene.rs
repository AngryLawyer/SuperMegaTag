use piston::{Event, Update};
use gamestate::GameState;

pub trait Scene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState);
}

pub struct SceneManager  {
    current_scene: Option<Box<Scene + 'static>>,
}

impl  SceneManager  {
    pub fn new() -> SceneManager {
        SceneManager {
            current_scene: None
        }
    }

    pub fn set_scene(&mut self, new_scene: Box<Scene + 'static>) {
        self.current_scene = Some(new_scene)
    }

    pub fn handle_event(&mut self, e: &Event, state: &mut GameState) {
        match self.current_scene {
            Some(ref mut current_scene) => {
               current_scene.handle_event(e, state);
            },
            None => ()
        }
    }

    pub fn initialize(&mut self, constructor: |&mut SceneManager| -> Box<Scene + 'static>) {
        let constructed = constructor(self);
        self.set_scene(constructed);
    }
}
