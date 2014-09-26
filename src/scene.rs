use piston::{Event, Update};

pub trait Scene<T> {
    fn handle_event(&mut self, e: &Event, state: &mut T);
}

pub struct SceneManager <T> {
    current_scene: Option<Box<Scene<T> + 'static>>,
}

impl <T> SceneManager <T> {
    pub fn new() -> SceneManager<T> {
        SceneManager {
            current_scene: None
        }
    }

    pub fn set_scene(&mut self, new_scene: Box<Scene<T> + 'static>) {
        self.current_scene = Some(new_scene)
    }

    pub fn handle_event(&mut self, e: &Event, state: &mut T) {
        match self.current_scene {
            Some(ref mut current_scene) => {
               current_scene.handle_event(e, state);
            },
            None => ()
        }
    }

    pub fn initialize(&mut self, constructor: |&mut SceneManager<T>| -> Box<Scene<T> + 'static>) {
        let constructed = constructor(self);
        self.set_scene(constructed);
    }
}
