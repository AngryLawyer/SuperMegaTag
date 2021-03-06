use player::{Player, UP, DOWN, LEFT, RIGHT};
use std::io::{MemWriter, MemReader};

pub trait PacketSerialize {
    fn serialize(&self) -> Vec<u8>;
}

pub enum Packet {
    FullServerState(u16, u16, Vec<Player>),
    MovePacket(bool, bool, bool, bool)
}

pub fn deserializer(message: &Vec<u8>) -> Option<Packet> {
    let mut r = MemReader::new(message.clone());

    match r.read_u8() {
        Ok(0) => {
            match r.read_be_u16() {
                Ok(player_id) => {
                    match r.read_be_u16() {
                        Ok(tagged_id) => {
                            let mut result = vec![];
                            while r.eof() == false {
                                match Player::deserialize(&mut r) {
                                    Ok(player) => result.push(player),
                                    Err(_) => break
                                }
                            }
                            Some(FullServerState(player_id, tagged_id, result))
                        },
                        Err(_) => None
                    }
                },
                Err(_) => None
            }
        },
        Ok(1) => {
            match r.read_u8() {
                Ok(flags) => {
                    Some(MovePacket(
                        flags & UP as u8 > 0,
                        flags & DOWN as u8 > 0,
                        flags & LEFT as u8 > 0,
                        flags & RIGHT as u8 > 0,
                    ))
                },
                Err(_) => None
            }
        }
        _ => None
    }
}

static ERROR_MESSAGE: &'static str = "Failed to serialize packet";

pub fn serializer(packet: &Packet) -> Vec<u8> {
    let mut w = MemWriter::new();
    match packet {
        &FullServerState(player_id, tagged_id, ref players) => {
            w.write_u8(0).ok().expect(ERROR_MESSAGE);
            w.write_be_u16(player_id).ok().expect(ERROR_MESSAGE);
            w.write_be_u16(tagged_id).ok().expect(ERROR_MESSAGE);
            for &player in players.iter() {
                w.write(player.serialize().as_slice()).ok().expect(ERROR_MESSAGE);
            }
        },
        &MovePacket(up, down, left, right) => {
            let flags = (if up { UP as u8} else { 0u8 }) | (if down { DOWN as u8} else { 0u8 }) | (if left { LEFT as u8} else { 0u8 }) | (if right { RIGHT as u8} else { 0u8 });
            w.write_u8(1).ok().expect(ERROR_MESSAGE);
            w.write_u8(flags).ok().expect(ERROR_MESSAGE);
        }
    };
    w.unwrap()
}
