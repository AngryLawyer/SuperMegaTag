use conrod::{
    UIContext,
};

use opengl_graphics::{
    Gl
};

use string_telephone::{Client, PollFailResult, PollDisconnected};
use packet;

pub struct GameState {
    uic: UIContext,
    gl: Gl,
    comms: Option<Client<packet::Packet>>
}

impl GameState {
    pub fn new(uic: UIContext, gl: Gl) -> GameState {
        GameState {
            uic: uic,
            gl: gl,
            comms: None
        }
    }

    pub fn set_comms(&mut self, comms: Client<packet::Packet>) {
        self.comms = Some(comms);
    }

    pub fn poll_comms(&mut self) -> Result<packet::Packet, PollFailResult> {
        match self.comms {
            Some(ref mut comms) => comms.poll(),
            None => Err(PollDisconnected)
        }
    }

    pub fn get_comms(&mut self) -> &mut Option<Client<packet::Packet>> {
        &mut self.comms
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
