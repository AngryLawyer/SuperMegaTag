use collections::str::{Slice, Owned};

pub enum Packet {
    ServerStatusPacket,
    MovePacket
}

pub fn deserializer(message: &Vec<u8>) -> Packet {
    match String::from_utf8_lossy(message.as_slice()) {
        Slice(slice) => slice.to_string(),
        Owned(item) => item
    }
}

pub fn serializer(packet: &Packet) -> Vec<u8> {
    packet.clone().into_bytes()
}
