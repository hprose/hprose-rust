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
 * LastModified: Sep 13, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

pub use self::formatter::*;
pub use self::encoder::{Encoder, Encodable};
pub use self::decoder::{Decoder, Decodable};
pub use self::writer::Writer;
pub use self::reader::{Reader, DecodeResult};

mod tags;
mod util;

mod encoder;
mod decoder;

mod writer_refer;

mod formatter;
mod writer;
mod reader;


