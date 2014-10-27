use scene::{Scene, BoxedScene};
use gamestate::GameState;
use piston::{graphics, Render, Event, Update, Input};
use piston::graphics::{AddColor, Draw, AddImage, RelativeTransform2d};
use piston::input;
use string_telephone::PollDisconnected;
use packet;
use player::Player;
use connectscene::ConnectScene;

const MAX_FUZZY_DISTANCE: i32 = 8 * 8;

pub struct GameScene {
    clock: f64,
    next_broadcast: f64,
    next_think: f64,
    player_id: Option<u16>,
    tagged_id: u16,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    players: Vec<Player>,
}

impl GameScene {
    pub fn new() -> BoxedScene {
       box GameScene {
           clock: 0.0,
           next_broadcast: 0.0,
           next_think: 0.0,
           player_id: None,
           tagged_id: 0,
           players: vec![],
           up: false,
           down: false,
           left: false,
           right: false
       }
    }

    fn stitch_players(&mut self, mut new_players: Vec<Player>) {
        //We need to unify our current player position, for prediction
        match self.player_id {
            Some(id) => {
                {
                    let maybe_old_self = self.players.iter_mut().find(|player| { player.id == id });
                    let maybe_new_self = new_players.iter_mut().find(|player| { player.id == id });
                    match (maybe_old_self, maybe_new_self) {
                        (Some(old), Some(new)) => {
                            new.key_up = old.key_up;
                            new.key_down = old.key_down;
                            new.key_left = old.key_left;
                            new.key_right = old.key_right;

                            //How different do our positions need to be before we wipe prediction?
                            if old.distance_sq(new) > MAX_FUZZY_DISTANCE {
                                new.x = old.x;
                                new.y = old.y;
                            }
                        }
                        _ => ()
                    }
                }
                self.players = new_players;
            },
            None => {self.players = new_players;}
        }
    }
}

impl Scene for GameScene {

    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<BoxedScene> {
        match e {
            &Update(args) => {
                let mut maybe_scene = None;
                self.clock += args.dt;
                match state.poll_comms() {
                    Ok(packet::FullServerState(player_id, tagged_id, players)) => {
                        self.player_id = Some(player_id);
                        self.tagged_id = tagged_id;
                        self.stitch_players(players);
                    },
                    Ok(_) => (),
                    Err(PollDisconnected) => {
                        maybe_scene = Some(ConnectScene::new());
                    }
                    Err(_) => ()
                }
                
                if self.clock >= self.next_think {
                    self.next_think = self.clock + 0.015;
                    for player in self.players.iter_mut() {
                        match self.player_id {
                            Some(player_id) => {
                                if player.id == player_id {
                                    player.key_up = self.up;
                                    player.key_down = self.down;
                                    player.key_left = self.left;
                                    player.key_right = self.right;
                                }
                            },
                            None => ()
                        };

                        if !player.is_frozen(self.clock) {
                            player.think()
                        }
                    }
                }
                
                if self.clock >= self.next_broadcast {
                    self.next_broadcast = self.clock + 0.05;
                    match state.get_comms() {
                        &Some(ref mut comms) => {
                            comms.send(&packet::MovePacket(self.up, self.down, self.left, self.right));
                        },
                        &None => ()
                    }
                }
                maybe_scene
            },
            &Input(input::Press(input::Keyboard(key))) => {
                match key {
                    input::keyboard::Up => {
                        self.up = true;
                    },
                    input::keyboard::Down => {
                        self.down = true;
                    },
                    input::keyboard::Left => {
                        self.left = true;
                    },
                    input::keyboard::Right => {
                        self.right = true;
                    },
                    _ => ()
                };
                None
            },
            &Input(input::Release(input::Keyboard(key))) => {
                match key {
                    input::keyboard::Up => {
                        self.up = false;
                    },
                    input::keyboard::Down => {
                        self.down = false;
                    },
                    input::keyboard::Left => {
                        self.left = false;
                    },
                    input::keyboard::Right => {
                        self.right = false;
                    },
                    _ => ()
                };
                None
            },
            &Render(args) => {
                let (gl, player_tex, player_lit_tex, opponent_tex, opponent_lit_tex) = state.get_gl_and_assets();

                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = &graphics::Context::abs(args.width as f64, args.height as f64);
                c.rgb(0.0, 0.0, 0.0).draw(gl);

                for player in self.players.iter() {
                    let c = c.trans((player.x - 16) as f64, (player.y - 16) as f64);
                    match self.player_id {
                        Some(id) if id == player.id => {
                            if player.id == self.tagged_id {
                                c.image(player_lit_tex).draw(gl)
                            } else {
                                c.image(player_tex).draw(gl)
                            }
                        },
                        _ => {
                            if player.id == self.tagged_id {
                                c.image(opponent_lit_tex).draw(gl)
                            } else {
                                c.image(opponent_tex).draw(gl)
                            }
                        }
                    };
                };
                None
            },
            _ => None
        }
    }
}
