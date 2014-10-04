use player::Player;
use std::io::{MemWriter, MemReader};

pub trait PacketSerialize {
    fn serialize(&self) -> Vec<u8>;
}

pub enum Packet {
    FullServerState(Vec<Player>),
    MovePacket
}

pub fn deserializer(message: &Vec<u8>) -> Option<Packet> {
    let mut r = MemReader::new(message.clone());
    match r.read_u8() {
        Ok(0) => {
            let mut result = vec![];
            while r.eof() == false {
                match Player::deserialize(&mut r) {
                    Ok(player) => result.push(player),
                    Err(_) => break
                }
            }
            Some(FullServerState(result))
        },
        _ => None
    }
}

pub fn serializer(packet: &Packet) -> Vec<u8> {
    let mut w = MemWriter::new();
    match packet {
        &FullServerState(ref players) => {
            w.write_u8(0);
            for &player in players.iter() {
                w.write(player.serialize().as_slice());
            }
        },
        _ => ()
    };
    w.unwrap()
}
