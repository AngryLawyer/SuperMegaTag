use std::fmt;
use packet::PacketSerialize;
use std::io::{MemWriter, MemReader, IoError};

pub enum PlayerFlags {
    UP = 0x0001,
    DOWN = 0x0002,
    LEFT = 0x0004,
    RIGHT = 0x0008
}

#[deriving(Clone)]
pub struct Player {
    pub id: u16,
    pub x: i32,
    pub y: i32,
    pub key_up: bool,
    pub key_down: bool,
    pub key_left: bool,
    pub key_right: bool,
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
            key_right: false
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
        (if self.key_right { RIGHT as u8 } else { 0 }) 
    }

    pub fn read_playerflags(&mut self, flags: u8) {
        self.key_up = flags & UP as u8 > 0;
        self.key_down = flags & DOWN as u8 > 0;
        self.key_left = flags & LEFT as u8 > 0;
        self.key_right = flags & RIGHT as u8 > 0;
    }

    pub fn deserialize(reader: &mut MemReader) -> Result<Player, IoError> {
        let id = try!(reader.read_be_u16());
        let x = try!(reader.read_be_i32());
        let y = try!(reader.read_be_i32());
        let flags = try!(reader.read_u8());
        let mut player = Player::new(id, x, y);
        player.read_playerflags(flags);
        Ok(player)
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
