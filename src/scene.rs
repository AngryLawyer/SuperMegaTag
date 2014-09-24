use piston::{Event};

pub trait Scene<T> {
    fn handle_event(&mut self, e: &Event, state: &mut T);
}

pub struct SceneManager <T> {
    current_scene: Box<Scene<T> + 'static>
}

impl <T> SceneManager <T> {
    pub fn new(initial_scene: Box<Scene<T> + 'static>) -> SceneManager<T> {
        SceneManager {
            current_scene: initial_scene
        }
    }

    pub fn set_scene(&mut self, new_scene: Box<Scene<T> + 'static>) {
        self.current_scene = new_scene
    }
}
