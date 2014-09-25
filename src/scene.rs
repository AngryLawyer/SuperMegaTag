use piston::{Event, Update};

pub trait Scene<T> {
    fn handle_event(&mut self, e: &Event, state: &mut T);
    fn should_transition(&self) -> Option<Box<Scene<T> + 'static>>;
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

    pub fn handle_event(&mut self, e: &Event, state: &mut T) {
        self.current_scene.handle_event(e, state);
        match e {
            &Update(_) => {
                let maybe_new_scene = self.current_scene.should_transition();
                match maybe_new_scene {
                    Some(scene) => {
                        self.set_scene(scene)
                    },
                    None => ()
                }
            },
            _ => ()
        }
    }
}
