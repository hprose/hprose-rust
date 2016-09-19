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
 * LastModified: Sep 19, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate test;

use super::tags::*;
use super::*;

use super::byte_reader::ByteReader;
use super::bool_decoder::bool_decode;
use super::i64_decoder::i64_decode;
use super::u64_decoder::u64_decode;

use std::fmt;
use std::io;
use std::f64;
use std::num;
use std::str;

#[derive(Clone, PartialEq, Debug)]
pub enum ParserError {
    BadUTF8Encode,
    ParseBoolError,
    ParseIntError(num::ParseIntError),
    ParseFloatError(num::ParseFloatError),
    IoError(io::ErrorKind, String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum DecoderError {
    ParserError(ParserError),
    CastError(&'static str, &'static str),
    UnexpectedTag(u8, Option<Bytes>)
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecoderError::CastError(src_type, dst_type) => write!(f, "can't convert {} to {}", src_type, dst_type),
            DecoderError::UnexpectedTag(tag, ref expect_tags_option) => {
                match *expect_tags_option {
                    // todo: format tag as 'c'(0xdd)
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
    pub reader: ByteReader<'a>
}

impl<'a> Reader<'a> {
    #[inline]
    pub fn new(buf: &'a Bytes) -> Reader<'a> {
        Reader {
            reader: ByteReader::new(buf)
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

impl<'a> Decoder for Reader<'a> {
    type Error = DecoderError;

    fn read_nil(&mut self) -> DecodeResult<()> {
        unimplemented!()
    }

    fn read_bool(&mut self) -> DecodeResult<bool> {
        self.reader.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| bool_decode(self, t))
    }

    fn read_i64(&mut self) -> DecodeResult<i64> {
        self.reader.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| i64_decode(self, t))
    }

    fn read_u64(&mut self) -> DecodeResult<u64> {
        self.reader.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| u64_decode(self, t))
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

    fn read_string_without_tag(&mut self) -> DecodeResult<String> {
        // todo: set reader ref
        self.reader.read_str().map_err(|e| DecoderError::ParserError(e))
    }

    fn read_string(&mut self) -> DecodeResult<String> {
        unimplemented!()
    }

    fn read_bytes(&mut self) -> DecodeResult<Bytes> {
        unimplemented!()
    }

    fn read_option<T, F>(&mut self, f: F) -> DecodeResult<T> where F: FnMut(&mut Self, bool) -> DecodeResult<T> {
        unimplemented!()
    }

    fn read_seq<T, F>(&mut self, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self, usize) -> DecodeResult<T> {
        unimplemented!()
    }
}

fn tag_to_str(tag: u8) -> Result<&'static str, DecoderError> {
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

pub fn cast_error(tag: u8, dst_type: &'static str) -> DecoderError {
    tag_to_str(tag)
        .map(|src_type| DecoderError::CastError(src_type, dst_type))
        .unwrap_or_else(|e| e)
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

    #[bench]
    fn benchmark_unserialize_i64(b: &mut Bencher) {
        let bytes = Writer::new(true).serialize(&12345).bytes();
        b.iter(|| {
            Reader::new(&bytes).unserialize::<i64>().unwrap();
        });
    }
}
