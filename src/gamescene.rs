use scene::{Scene, SceneManager};
use gamestate::GameState;
use piston::{graphics, Render, Event, Update};
use piston::graphics::{AddColor, Draw, AddEllipse, AddBorder};
use string_telephone::{Client, PollDisconnected};
use packet;
use player::Player;
use connectscene::ConnectScene;

pub struct GameScene<'r> {
    manager: &'r mut SceneManager,
    clock: f64,
    players: Vec<Player>
}

impl <'r> GameScene<'r> {

    pub fn new<'r>(manager: &'r mut SceneManager) -> GameScene<'r> {
       GameScene {
           manager: manager,
           clock: 0.0,
           players: vec![]
       }
    }
}
//let clock_rate = 0.0015;

impl <'r> Scene for GameScene <'r> {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) {
        match e {
            &Update(args) => {
                self.clock += args.dt;
                match state.poll_comms() {
                    Ok(packet::FullServerState(players)) => {
                        self.players = players;
                    },
                    Ok(_) => (),
                    Err(PollDisconnected) => {
                        self.manager.set_scene(|manager| box ConnectScene::new(manager));
                    }
                    Err(_) => ()
                }
            },
            &Render(args) => {
                let gl = state.get_gl();

                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = &graphics::Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);

                for player in self.players.iter() {
                    c.rgb(0.0, 0.0, 0.0).circle(player.x as f64, player.y as f64, 10.0).draw(gl);
                }

            },
            _ => ()
        }
    }
}
