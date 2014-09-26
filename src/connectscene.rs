use scene::{Scene, SceneManager};
use gamestate::GameState;
use conrod::{
    label,
    Color,
    Point,
    widget_matrix,
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

use piston::{graphics, Render, Event};
use piston::graphics::{AddColor, Draw};

pub struct ConnectScene<'r, T:'r> {
    manager: &'r mut SceneManager<T>,
    pub edit_ip: Vec<String>
}

impl <'r, T> ConnectScene<'r, T> {

    pub fn new<'r> (manager: &'r mut SceneManager<T>) -> ConnectScene<'r, T> {
       ConnectScene {
           manager: manager,
           edit_ip: vec!["127".to_string(), "0".to_string(), "0".to_string(), "1".to_string()]
       }
    }
}

impl <'r, T> Scene<GameState> for ConnectScene <'r, T> {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) {
        match e {
            &Render(args) => {
                let (uic, gl) = state.get_drawables();

                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = &graphics::Context::abs(args.width as f64, args.height as f64);


                c.rgb(1.0, 1.0, 1.0).draw(gl);
                label::draw(
                    gl,
                    uic,
                    Point::new(0f64, 0f64, 0f64), // Screen position.
                    48u32, // Font size.
                    Color::new(1.0, 0.0, 0.0, 1.0),
                    "Select a server"
                );

                widget_matrix::draw(
                    4, // cols.
                    1, // rows.
                    Point::new(0.0, 56.0, 0.0), // matrix position.
                    300.0, // width.
                    240.0, // height.
                    |num, _, _, pos, width, _| { // This is called for every widget.
                        // Now draw the widgets with the given callback.
                        uic.text_box(2 + num as u64, self.edit_ip.get_mut(num))
                            .font_size(24u32)
                            .dimensions(width, 36.0)
                            .position(pos.x, pos.y)
                            .frame(2.0, Color::black())
                            .color(Color::black())
                            .draw(gl);

                    }
                );

                uic.button(7u64)
                    .dimensions(90.0, 36.0)
                    .position(0.0, 128.0)
                    .color(Color::black())
                    .frame(2.0, Color::black())
                    .label("Connect", 24u32, Color::white())
                    .callback(|| {
                        let parsed = (from_str(self.edit_ip.get(0)), from_str(self.edit_ip.get(1)), from_str(self.edit_ip.get(2)), from_str(self.edit_ip.get(3)));
                        match parsed {
                            (Some(a), Some(b), Some(c), Some(d)) => {
                                match Client::connect(SocketAddr {ip: Ipv4Addr(0, 0, 0, 0), port: 0}, SocketAddr {ip: Ipv4Addr(a, b, c, d), port: 8869}, settings) {
                                    Ok(client) => {
                                        ()
                                    },
                                    Err(e) => fail!("Failed to connect - {}", e)
                                };
                            },
                            _ => ()
                        };
                    })
                    .draw(gl);

            },
            _ => {},
        }
    }
}
