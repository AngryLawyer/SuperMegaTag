use scene::Scene;
use gamestate::GameState;
use conrod::{
    UIContext,
    label,
    Color,
    Point,
    widget_matrix,
    Button,
    Callable,
    Colorable,
    Drawable,
    DropDownList,
    EnvelopeEditor,
    Frameable,
    Labelable,
    NumberDialer,
    Positionable,
    Slider,
    Shapeable,
    TextBox,
    Toggle,
    XYPad,
};

use piston::{EventIterator, EventSettings, WindowSettings, graphics, Render, Event};
use piston::graphics::{AddColor, Draw};

pub struct ConnectScene<T> {
    pub edit_ip: Vec<String>
}

impl <T> ConnectScene<T> {

    pub fn new() -> ConnectScene<T> {
       ConnectScene {
           edit_ip: vec!["127".to_string(), "0".to_string(), "0".to_string(), "1".to_string()]
       }
    }
}

impl <T> Scene<GameState> for ConnectScene <T> {
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
                    |num, col, row, pos, width, height| { // This is called for every widget.
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
                    .callback(|| () )
                    .draw(gl);

            },
            _ => {},
        }
    }
}
