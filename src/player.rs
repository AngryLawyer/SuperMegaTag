use std::fmt;

pub struct Player {
    id: u16,
    x: i32,
    y: i32,
    key_up: bool,
    key_down: bool,
    key_left: bool,
    key_right: bool,
    is_tagged: bool
}

impl Player {
    pub fn new (id: u16, x: i32, y: i32) -> Player {
        Player {
            id: id,
            x: x,
            y: y,
            key_up: false,
            key_down: false,
            key_left: false,
            key_right: false,
            is_tagged: false
        }
    }

    pub fn think(&mut self) {
        if self.key_up && self.y > 0 {
            self.y -= 1;
        } else if self.key_down && self.y < 600 {
            self.y += 1;
        }

        if self.key_left && self.x > 0 {
            self.x -= 1;
        } else if self.key_right && self.x < 800 {
            self.x += 1;
        }
    }
}

impl fmt::Show for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {} {}", self.id, self.x, self.y)
    }
}
