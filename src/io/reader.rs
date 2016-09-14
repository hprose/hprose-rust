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
 * LastModified: Sep 14, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate test;

use super::tags;
use super::*;

use super::bool_decoder::bool_decoder;

use std::io;

#[derive(Clone, PartialEq, Debug)]
pub enum ParserError {
    /// msg, line, col
    //    SyntaxError(ErrorCode, usize, usize),
    IoError(io::ErrorKind, String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum DecoderError {
    ParserError(ParserError),
    CastError(u8, String)
}

pub type DecodeResult<T> = Result<T, DecoderError>;

pub struct Reader<'a> {
    buf: &'a Vec<u8>,
    off: usize
}

pub trait ByteReader {
    fn read_byte(&mut self) -> Result<u8, ParserError>;
    fn read_until(&mut self, tag: u8) -> Result<&[u8], ParserError>;
}

impl<'a> Reader<'a> {
    #[inline]
    pub fn new(buf: &'a Vec<u8>) -> Reader<'a> {
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

impl<'a> ByteReader for Reader<'a> {
    fn read_byte(&mut self) -> Result<u8, ParserError> {
        if self.off >= self.buf.len() {
            return Err(io_error_to_error(io::Error::new(io::ErrorKind::UnexpectedEof, "")))
        }
        let b = self.buf[self.off];
        self.off += 1;
        return Ok(b)
    }

    fn read_until(&mut self, tag: u8) -> Result<&[u8], ParserError> {
        let result = &self.buf[self.off..];
        match result.iter().position(|x| *x == tag) {
            Some(idx) => {
                self.off += idx + 1;
                Ok(&result[..idx])
            },
            None => {
                self.off = self.buf.len();
                Err(io_error_to_error(io::Error::new(io::ErrorKind::UnexpectedEof, "")))
            }
        }
    }
}

impl<'a> Decoder for Reader<'a> {
    type Error = DecoderError;

    fn read_nil(&mut self) -> DecodeResult<()> {
        unimplemented!()
    }

    fn read_bool(&mut self) -> DecodeResult<bool> {
        self.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| bool_decoder(self, t))
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

#[cfg(test)]
mod tests {
    use super::test::Bencher;
    use super::super::*;

    #[bench]
    fn benchmark_unserialize_bool(b: &mut Bencher) {
        let bytes = Writer::new(true).serialize(&true).bytes();
        b.iter(|| {
            Reader::new(&bytes).unserialize::<bool>().unwrap();
        });
    }
}
