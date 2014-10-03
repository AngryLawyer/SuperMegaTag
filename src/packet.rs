use player::Player;

pub enum Packet {
    FullServerState(Vec<Player>),
    MovePacket
}

pub fn deserializer(message: &Vec<u8>) -> Option<Packet> {
    None
}

pub fn serializer(packet: &Packet) -> Vec<u8> {
    vec![]
}
