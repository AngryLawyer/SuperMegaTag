use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::IoError;
use std::time::duration::Duration;
use string_telephone::{Client, ConnectionConfig, ClientConnectionConfig};
use scene::{Scene,BoxedScene};
use gamescene::GameScene;
use gamestate::GameState;
use graphics::{Context, AddColor, Draw};
use conrod::{
    Label,
    Color,
    WidgetMatrix,
    Button,
    Callable,
    Colorable,
    Drawable,
    Frameable,
    Labelable,
    Positionable,
    Shapeable,
    TextBox,
};

use event::{Render, Event, Update};
use packet;

enum ConnectState {
    Disconnected,
    Connecting(Receiver<Result<Client<packet::Packet>, IoError>>)
}

pub struct ConnectScene {
    pub edit_ip: Vec<String>,
    try_connect: ConnectState,
    error_message: Option<String>
}

impl ConnectScene {

    pub fn new() -> BoxedScene {
       box ConnectScene {
           edit_ip: vec!["127".to_string(), "0".to_string(), "0".to_string(), "1".to_string()],
           try_connect: Disconnected,
           error_message: None
       }
    }
    

    pub fn is_connecting(&self) -> bool {
        match self.try_connect { Disconnected => false, _ => true}
    }
}

impl Scene for ConnectScene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<BoxedScene> {
        match e {
            &Update(_) => {
                let mut maybe_scene = None;
                let should_disconnect = match self.try_connect {
                    Connecting(ref socket) => {
                        match socket.try_recv() {
                            Ok(Ok(comms)) => {
                                println!("Connected");
                                maybe_scene = Some(GameScene::new());
                                state.set_comms(comms);
                                true
                            },
                            Ok(Err(e)) => {
                                println!("Couldn't connect");
                                self.error_message = Some(e.desc.to_string());
                                true 
                            },
                            _ => false
                        }
                    },
                    _ => false
                };
                if should_disconnect {
                    self.try_connect = Disconnected;
                }
                maybe_scene
            },
            &Render(args) => {
                let (uic, gl) = state.get_drawables();

                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);

                let initial_pos = [((600u32 / 2) - 150) as f64, 0.0];

                //Title label
                uic.label("Select a server")
                    .position(initial_pos[0], initial_pos[1])
                    .size(48u32)
                    .color(Color::black())
                    .draw(gl);

                uic.widget_matrix(4, 1)
                    .position(initial_pos[0], initial_pos[1] + 56.0)
                    .dimensions(300.0, 240.0)
                    .each_widget(|uic, num, _, _, pos, dims| {
                        // Now draw the widgets with the given callback.
                        uic.text_box(2 + num as u64, self.edit_ip.get_mut(num).unwrap())
                            .font_size(24u32)
                            .dimensions(dims[0], 36.0)
                            .position(pos[0], pos[1])
                            .frame(2.0)
                            .color(Color::black())
                            .draw(gl);

                    });

                if self.is_connecting() {
                    uic.label("Connecting...")
                        .position(initial_pos[0], initial_pos[1] + 128.0)
                        .size(48u32)
                        .color(Color::black())
                        .draw(gl);
                } else {
                    uic.button(7u64)
                        .dimensions(90.0, 36.0)
                        .position(initial_pos[0], initial_pos[1] + 128.0)
                        .color(Color::black())
                        .label_color(Color::white())
                        .frame(2.0)
                        .label("Connect")
                        .callback(|| {
                            if self.is_connecting() == false {
                                let parsed = (from_str(self.edit_ip[0].as_slice()), from_str(self.edit_ip[1].as_slice()), from_str(self.edit_ip[2].as_slice()), from_str(self.edit_ip[3].as_slice()));

                                match parsed {
                                    (Some(a), Some(b), Some(c), Some(d)) => {
                                        self.error_message = None;
                                        let (tx, rx) = channel();
                                        self.try_connect = Connecting(rx);
                                        spawn(proc() {
                                            let settings = ConnectionConfig {
                                                protocol_id: 88869,
                                                timeout_period: Duration::seconds(10),
                                                packet_deserializer: packet::deserializer,
                                                packet_serializer: packet::serializer
                                            };

                                            let client_settings = ClientConnectionConfig::new(3, Duration::seconds(5));

                                            println!("Connecting");
                                            tx.send(Client::connect(SocketAddr {ip: Ipv4Addr(0, 0, 0, 0), port: 0}, SocketAddr {ip: Ipv4Addr(a, b, c, d), port: 8869}, settings, client_settings));
                                        });
                                    },
                                    _ => ()
                                };
                            }
                        })
                        .draw(gl);
                }

                match self.error_message {
                    Some(ref error) => {
                        uic.label(error.as_slice())
                            .position(initial_pos[0], initial_pos[1] + 128.0 + 36.0)
                            .size(48u32)
                            .color(Color::black())
                            .draw(gl);
                        }
                    None => ()
                }
                None
            },
            _ => None,
        }
    }
}
