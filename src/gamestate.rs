use conrod::{
    UIContext,
};

use opengl_graphics::{
    Gl,
    Texture,
};

pub struct GameState {
    uic: UIContext,
    gl: Gl
}

impl GameState {
    pub fn new(uic: UIContext, gl: Gl) -> GameState {
        GameState {
            uic: uic,
            gl: gl
        }
    }

    pub fn get_drawables(&mut self) -> (&mut UIContext, &mut Gl) {
        (&mut self.uic, &mut self.gl)
    }

    pub fn get_uic(&mut self) -> &mut UIContext {
        &mut self.uic
    }

    pub fn get_gl(&mut self) -> &mut Gl {
        &mut self.gl
    }
}
