//! HTTP/1 protocol implementation.

use bytes::{Bytes, BytesMut};

mod chunked;
mod client;
mod codec;
mod decoder;
mod dispatcher;
mod encoder;
mod expect;
mod payload;
mod service;
mod upgrade;
mod utils;

pub use self::client::{ClientCodec, ClientPayloadCodec};
pub use self::codec::Codec;
pub use self::dispatcher::Dispatcher;
pub use self::expect::ExpectHandler;
pub use self::payload::Payload;
pub use self::service::{H1Service, H1ServiceHandler};
pub use self::upgrade::UpgradeHandler;
pub use self::utils::SendResponse;

#[derive(Debug)]
/// Codec message
pub enum Message<T> {
    /// Http message
    Item(T),
    /// Payload chunk
    Chunk(Option<Bytes>),
}

impl<T> From<T> for Message<T> {
    fn from(item: T) -> Self {
        Message::Item(item)
    }
}

/// Incoming request type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    None,
    Payload,
    Stream,
}

const LW: usize = 2 * 1024;
const HW: usize = 32 * 1024;

pub(crate) fn reserve_readbuf(src: &mut BytesMut) {
    let cap = src.capacity();
    if cap < LW {
        src.reserve(HW - cap);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::Request;

    impl Message<Request> {
        pub fn message(self) -> Request {
            match self {
                Message::Item(req) => req,
                _ => panic!("error"),
            }
        }

        pub fn chunk(self) -> Bytes {
            match self {
                Message::Chunk(Some(data)) => data,
                _ => panic!("error"),
            }
        }

        pub fn eof(self) -> bool {
            match self {
                Message::Chunk(None) => true,
                Message::Chunk(Some(_)) => false,
                _ => panic!("error"),
            }
        }
    }
}
