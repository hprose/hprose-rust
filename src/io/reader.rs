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
 * io/reader.rs                                           *
 *                                                        *
 * hprose reader for Rust.                                *
 *                                                        *
 * LastModified: Sep 13, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/


use super::tags;
use super::*;

use std::io;

#[derive(Clone, PartialEq, Debug)]
pub enum ParserError {
    /// msg, line, col
    //    SyntaxError(ErrorCode, usize, usize),
    IoError(io::ErrorKind, String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum DecoderError {
    ParserError(ParserError)
}

pub type DecodeResult<T> = Result<T, DecoderError>;

pub struct Reader {
    buf: Vec<u8>,
    off: usize
}

pub trait ByteReader {
    fn read_byte(&mut self) -> Result<u8, ParserError>;
}

impl Reader {
    #[inline]
    pub fn new(buf: Vec<u8>) -> Reader {
        Reader {
            buf: buf,
            off: 0
        }
    }

    #[inline]
    pub fn unserialize<T: Decodable>(&mut self) -> DecodeResult<T> {
        self.read()
    }

    #[inline]
    pub fn read<T: Decodable>(&mut self) -> DecodeResult<T> {
        Decodable::decode(self)
    }
}

#[inline]
fn io_error_to_error(io: io::Error) -> ParserError {
    ParserError::IoError(io.kind(), io.to_string())
}

impl ByteReader for Reader {
    fn read_byte(&mut self) -> Result<u8, ParserError> {
        if self.off >= self.buf.len() {
            return Err(io_error_to_error(io::Error::new(io::ErrorKind::UnexpectedEof, "")))
        }
        let b = self.buf[self.off];
        self.off += 1;
        return Ok(b)
    }
}

impl Decoder for Reader {
    type Error = DecoderError;

    fn read_nil(&mut self) -> DecodeResult<()> {
        unimplemented!()
    }

    fn read_bool(&mut self) -> DecodeResult<bool> {
        unimplemented!()
    }

    fn read_i64(&mut self) -> DecodeResult<i64> {
        unimplemented!()
    }

    fn read_u64(&mut self) -> DecodeResult<u64> {
        unimplemented!()
    }

    fn read_f32(&mut self) -> DecodeResult<f32> {
        unimplemented!()
    }

    fn read_f64(&mut self) -> DecodeResult<f64> {
        unimplemented!()
    }

    fn read_char(&mut self) -> DecodeResult<char> {
        unimplemented!()
    }

    fn read_str(&mut self) -> DecodeResult<String> {
        unimplemented!()
    }

    fn read_bytes(&mut self) -> DecodeResult<Vec<u8>> {
        unimplemented!()
    }

    fn read_option<T, F>(&mut self, f: F) -> DecodeResult<T> where F: FnMut(&mut Self, bool) -> DecodeResult<T> {
        unimplemented!()
    }

    fn read_seq<T, F>(&mut self, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self, usize) -> DecodeResult<T> {
        unimplemented!()
    }
}
