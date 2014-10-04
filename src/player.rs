use std::fmt;
use packet::PacketSerialize;
use std::io::MemWriter;

enum PlayerFlags {
    UP = 0x0001,
    DOWN = 0x0002,
    LEFT = 0x0004,
    RIGHT = 0x0008,
    TAGGED = 0x0010
}

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

    pub fn make_playerflags(&self) -> u8 {
        (if self.key_up { UP as u8 } else { 0 }) |
        (if self.key_down { DOWN as u8 } else { 0 }) |
        (if self.key_left { LEFT as u8 } else { 0 }) |
        (if self.key_right { RIGHT as u8 } else { 0 }) |
        (if self.is_tagged { TAGGED as u8 } else { 0 }) 
    }

    pub fn read_playerflags(&mut self, flags: u8) {
        self.key_up = flags & UP as u8 > 0;
        self.key_down = flags & DOWN as u8 > 0;
        self.key_left = flags & LEFT as u8 > 0;
        self.key_right = flags & RIGHT as u8 > 0;
        self.is_tagged = flags & TAGGED as u8 > 0;
    }
}

impl fmt::Show for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {} {}", self.id, self.x, self.y)
    }
}

static error_message: &'static str = "Failed to write to PacketSerialize";

impl PacketSerialize for Player {

    fn serialize(&self) -> Vec<u8> {
        let mut w = MemWriter::new();
        w.write_be_u16(self.id).ok().expect(error_message);
        w.write_be_i32(self.x).ok().expect(error_message);
        w.write_be_i32(self.y).ok().expect(error_message);
        w.write_u8(self.make_playerflags()).ok().expect(error_message);
        w.unwrap()
    }
}
