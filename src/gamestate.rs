use conrod::UiContext;
use opengl_graphics::{Gl, Texture};

use string_telephone::{Client, PollFailResult, PollDisconnected};
use packet;
use piston::AssetStore;

pub struct GameState {
    uic: UiContext,
    gl: Gl,
    comms: Option<Client<packet::Packet>>,

    player_tex: Texture,
    player_lit_tex: Texture,
    opponent_tex: Texture,
    opponent_lit_tex: Texture
}

pub fn load_texture(asset_store: &AssetStore, path: &str) -> Texture {
    let image = asset_store.path(path).unwrap();
    Texture::from_path(&image).unwrap()
}

impl GameState {
    pub fn new(uic: UiContext, gl: Gl, asset_store: &AssetStore) -> GameState {

        GameState {
            uic: uic,
            gl: gl,
            comms: None,
            player_tex: load_texture(asset_store, "player.png"),
            player_lit_tex: load_texture(asset_store, "player-lit.png"),
            opponent_tex: load_texture(asset_store, "opponent.png"),
            opponent_lit_tex: load_texture(asset_store, "opponent-lit.png"),
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

    pub fn get_drawables(&mut self) -> (&mut UiContext, &mut Gl) {
        (&mut self.uic, &mut self.gl)
    }

    pub fn get_uic(&mut self) -> &mut UiContext {
        &mut self.uic
    }

    pub fn get_gl(&mut self) -> &mut Gl {
        &mut self.gl
    }

    pub fn get_gl_and_assets(&mut self) -> (&mut Gl, &Texture, &Texture, &Texture, &Texture) {
        (&mut self.gl, &self.player_tex, &self.player_lit_tex, &self.opponent_tex, &self.opponent_lit_tex)
    }
}
