use core::convert::{TryFrom, TryInto};
use std::{
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

use async_stream::stream;
use futures::Stream;
use rand::{prelude::SliceRandom, thread_rng, Rng};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver},
    oneshot,
};

pub type SmallPacket = [u8; 3];
pub type LargePacket = [u8; 10];

#[derive(Debug)]
pub struct Packet {
    buf: Vec<u8>,
    packet_type: PacketType,
}

impl Packet {
    pub fn new(buf: Vec<u8>, packet_type: PacketType) -> Self {
        Self { buf, packet_type }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PacketType {
    Small,
    Large,
    Dynamic,
}

impl TryFrom<u8> for PacketType {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == PacketType::Small as u8 => Ok(PacketType::Small),
            x if x == PacketType::Large as u8 => Ok(PacketType::Large),
            x if x == PacketType::Dynamic as u8 => Ok(PacketType::Dynamic),
            _ => Err(ParseError::UnknownPacketType),
        }
    }
}

enum State {
    Start,
    ReadDynamicInfo,
    ReadPayload(PacketType, usize),
    Error,
}

pub struct Parser {
    state: State,
}

pub enum ParseError {
    UnknownPacketType,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            state: State::Start,
        }
    }

    pub fn next(&mut self, byte: u8, buf: &mut Vec<u8>) -> Option<PacketType> {
        let mut result = None;
        self.state = match self.state {
            State::Start => match (byte).try_into() {
                Ok(packet_type) => match packet_type {
                    PacketType::Dynamic => State::ReadDynamicInfo,
                    PacketType::Small => {
                        State::ReadPayload(PacketType::Small, core::mem::size_of::<SmallPacket>())
                    }
                    PacketType::Large => {
                        State::ReadPayload(PacketType::Large, core::mem::size_of::<LargePacket>())
                    }
                },
                Err(_err) => State::Error,
            },
            State::ReadDynamicInfo => State::ReadPayload(PacketType::Dynamic, byte as usize),
            State::ReadPayload(t, n) => {
                buf.push(byte);
                if n == 1 {
                    result = Some(t);
                    State::Start
                } else {
                    State::ReadPayload(t, n - 1)
                }
            }
            State::Error => State::Error,
        };
        result
    }
}

pub fn random_packet_channel_busy() -> (JoinHandle<()>, UnboundedReceiver<Vec<u8>>) {
    let (tx, rx) = mpsc::unbounded_channel();

    let thread_handle = thread::spawn(move || {
        let mut rng = thread_rng();
        loop {
            let packet = random_packet(&mut rng);

            match tx.send(packet) {
                Ok(_) => continue,
                Err(_) => return,
            }
        }
    });

    (thread_handle, rx)
}

pub fn random_packet_channel_lazy(
    mut req: UnboundedReceiver<oneshot::Sender<Vec<u8>>>,
) -> JoinHandle<()> {
    let thread_handle = thread::spawn(move || {
        let mut rng = thread_rng();
        loop {
            match req.blocking_recv() {
                Some(tx) => {
                    tx.send(random_packet(&mut rng)).ok();
                }
                None => return,
            }
        }
    });

    thread_handle
}

pub struct Handler {
    name: String,
    parser: Parser,
    buf: Vec<u8>,
}

impl Handler {
    pub fn new(name: String) -> Self {
        Self {
            name,
            parser: Parser::new(),
            buf: vec![],
        }
    }
    pub fn handle_raw_packet(&mut self, raw_packet: Vec<u8>) {
        self.buf.clear();
        for byte in raw_packet {
            match self.parser.next(byte, &mut self.buf) {
                Some(packet_type) => {
                    let packet = Packet::new(self.buf.clone(), packet_type);
                    self.buf.clear();
                    println!("{} received {:?}", self.name, packet)
                }
                None => continue,
            }
        }
    }
}
pub fn random_packet<R: Rng>(mut rng: R) -> Vec<u8> {
    const PACKET_TYPES: [PacketType; 3] =
        [PacketType::Small, PacketType::Large, PacketType::Dynamic];

    let mut result = vec![];

    let packet_type = PACKET_TYPES.choose(&mut rng).unwrap();
    result.push(*packet_type as u8);
    match packet_type {
        PacketType::Small => {
            for _ in 0..core::mem::size_of::<SmallPacket>() {
                result.push(rng_gen(&mut rng))
            }
        }
        PacketType::Large => {
            for _ in 0..core::mem::size_of::<LargePacket>() {
                result.push(rng_gen(&mut rng))
            }
        }
        PacketType::Dynamic => {
            let size = rng.gen_range(1..20);

            result.push(size);
            for _ in 0..size {
                result.push(rng_gen(&mut rng))
            }
        }
    }
    result
}

fn rng_gen<R: Rng>(mut rng: R) -> u8 {
    sleep(Duration::from_millis(10));
    rng.gen()
}

pub async fn random_packet_async<R: Rng>(mut r: R) -> Vec<u8> {
    random_packet(&mut r)
}

pub fn raw_packets_stream() -> impl Stream<Item = Vec<u8>> {
    stream! {
        let mut rng = thread_rng();
        loop {
            yield random_packet(&mut rng);
        }
    }
}
