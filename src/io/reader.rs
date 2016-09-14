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

use super::tags::*;
use super::*;

use super::bool_decoder::bool_decoder;

use std::fmt;
use std::io;

#[derive(Clone, PartialEq, Debug)]
pub enum ParserError {
    IoError(io::ErrorKind, String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum DecoderError {
    ParserError(ParserError),
    CastError(&'static str, &'static str),
    UnexpectedTag(u8, Option<Vec<u8>>)
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecoderError::CastError(srcType, dst_type) => write!(f, "can't convert {} to {}", srcType, dst_type),
            DecoderError::UnexpectedTag(tag, ref expect_tags_option) => {
                match *expect_tags_option {
                    Some(ref expect_tags) => write!(f, "Tag '{:?}' expected, but '{}' found in stream", expect_tags, tag),
                    None => write!(f, "Unexpected serialize tag '{}' in stream", tag)
                }
            },
            _ => fmt::Debug::fmt(self, f)
        }
    }
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

pub fn tagToStr(tag: u8) -> Result<&'static str, DecoderError> {
    match tag {
        b'0'...b'9' | TAG_INTEGER => Ok("i32"),
        TAG_LONG => Ok("big int"),
        TAG_DOUBLE => Ok("f64"),
        TAG_NULL => Ok("nil"),
        TAG_EMPTY => Ok("empty string"),
        TAG_TRUE => Ok("true"),
        TAG_FALSE => Ok("false"),
        TAG_NAN => Ok("NaN"),
        TAG_INFINITY => Ok("Infinity"),
        TAG_DATE | TAG_TIME => Ok("time"),
        TAG_BYTES => Ok("bytes"),
        TAG_UTF8_CHAR | TAG_STRING => Ok("string"),
        TAG_GUID => Ok("GUID"),
        TAG_LIST => Ok("slice"),
        TAG_MAP => Ok("map"),
        TAG_CLASS | TAG_OBJECT => Ok("struct"),
        TAG_REF => Ok("reference"),
        _ => Err(DecoderError::UnexpectedTag(tag, None))
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
