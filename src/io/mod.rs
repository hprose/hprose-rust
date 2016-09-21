/**********************************************************\
|                                                          |
|                          hprose                          |
|                                                          |
| Official WebSite: http://www.hprose.com/                 |
|                   http://www.hprose.org/                 |
|                                                          |
\**********************************************************/
/**********************************************************\
 *                                                        *
 * io/mod.rs                                              *
 *                                                        *
 * hprose io module for Rust.                             *
 *                                                        *
 * LastModified: Sep 21, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

pub use self::formatter::*;
pub use self::encoder::{Encoder, Encodable};
pub use self::decoder::{Decoder, Decodable};
pub use self::writer::{Writer, ByteWriter};
pub use self::reader::{Reader, DecodeResult, DecoderError};
pub use self::byte_reader::ByteReader;

use std::collections::HashMap;
use std::time::SystemTime;

pub type Bytes = Vec<u8>;
pub type List = Vec<Hprose>;
pub type Map = HashMap<String, Hprose>;

/// Represents a Hprose value
#[derive(Clone, PartialEq, Debug)]
pub enum Hprose {
    Nil,
    Boolean(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    DateTime(SystemTime),
    Bytes(self::Bytes),
    String(String),
    List(self::List),
    Map(self::Map)
}

/// Hprose Tags
pub mod tags;
mod util;

mod encoder;
mod decoder;
mod bool_decoder;
mod i64_decoder;
mod u64_decoder;
mod f32_decoder;
mod f64_decoder;
mod string_decoder;
mod map_decoder;

mod writer_refer;

mod formatter;
mod writer;
mod reader;
mod byte_reader;
