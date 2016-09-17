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
 * LastModified: Sep 17, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate test;

use super::tags::*;
use super::*;

use super::bool_decoder::bool_decode;
use super::i64_decoder::i64_decode;
use super::u64_decoder::u64_decode;

use std::fmt;
use std::io;
use std::f64;

#[derive(Clone, PartialEq, Debug)]
pub enum ParserError {
    BadUTF8Encode,
    ParseBoolError,
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
            DecoderError::CastError(srcType, dst_type) => write!(f, "can't convert {} to {}", srcType, dst_type),
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
    buf: &'a Bytes,
    off: usize
}

pub trait ByteReader {
    fn read_byte(&mut self) -> Result<u8, ParserError>;
    fn read_i64_with_tag(&mut self, tag: u8) -> Result<i64, ParserError>;
    fn read_i64(&mut self) -> Result<i64, ParserError>;
    fn read_u64(&mut self) -> Result<i64, ParserError>;
    fn read_len(&mut self) -> Result<usize, ParserError>;
    fn read_until(&mut self, tag: u8) -> Result<&[u8], ParserError>;
    fn read_f32(&mut self) -> Result<f32, ParserError>;
    fn read_f64(&mut self) -> Result<f64, ParserError>;
    fn read_u8_slice(&mut self, length: i64) -> Result<&[u8], ParserError>;
    fn read_inf(&mut self) -> Result<f64, ParserError>;
}

impl<'a> Reader<'a> {
    #[inline]
    pub fn new(buf: &'a Bytes) -> Reader<'a> {
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

    fn read_i64_with_tag(&mut self, tag: u8) -> Result<i64, ParserError> {
        let mut i: i64 = 0;
        self.read_byte().and_then(|b| {
            if b == tag {
                Ok(i)
            } else {
                let mut neg = false;
                let next = match b {
                    TAG_NEG => {
                        neg = true;
                        self.read_byte()
                    },
                    TAG_POS => self.read_byte(),
                    _ => Ok(b)
                };
                if neg {
                    next.and_then(|mut b| {
                        while b != tag {
                            i = i * 10 - (b as i64 - b'0' as i64);
                            b = match self.read_byte() {
                                Ok(b) => b,
                                Err(e) => return Err(e)
                            }
                        }
                        Ok(i)
                    })
                } else {
                    next.and_then(|mut b| {
                        while b != tag {
                            i = i * 10 + (b as i64 - b'0' as i64);
                            b = match self.read_byte() {
                                Ok(b) => b,
                                Err(e) => return Err(e)
                            }
                        }
                        Ok(i)
                    })
                }
            }
        })
    }

    fn read_i64(&mut self) -> Result<i64, ParserError> {
        unimplemented!()
    }

    fn read_u64(&mut self) -> Result<i64, ParserError> {
        unimplemented!()
    }

    fn read_len(&mut self) -> Result<usize, ParserError> {
        unimplemented!()
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

    fn read_f32(&mut self) -> Result<f32, ParserError> {
        unimplemented!()
    }

    fn read_f64(&mut self) -> Result<f64, ParserError> {
        unimplemented!()
    }

    fn read_u8_slice(&mut self, length: i64) -> Result<&[u8], ParserError> {
        if length == 0 {
            return Ok(&[])
        }
        let p = self.off;
        let mut i: i64 = 0;
        while i < length {
            let b = self.buf[self.off];
            match b >> 4 {
                0...7 => self.off += 1,
                12 | 13 => self.off += 2,
                14 => self.off += 3,
                15 => {
                    if b & 8 == 8 {
                        return Err(ParserError::BadUTF8Encode)
                    }
                    self.off += 4;
                    i += 1
                },
                _ => return Err(ParserError::BadUTF8Encode)
            }
            i += 1;
        }
        Ok(&self.buf[p..self.off])
    }

    fn read_inf(&mut self) -> Result<f64, ParserError> {
        self.read_byte().and_then(|sign| Ok(if sign == TAG_POS { f64::INFINITY } else { f64::NEG_INFINITY }))
    }
}

impl<'a> Decoder for Reader<'a> {
    type Error = DecoderError;

    fn read_nil(&mut self) -> DecodeResult<()> {
        unimplemented!()
    }

    fn read_bool(&mut self) -> DecodeResult<bool> {
        self.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| bool_decode(self, t))
    }

    fn read_i64(&mut self) -> DecodeResult<i64> {
        self.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| i64_decode(self, t))
    }

    fn read_u64(&mut self) -> DecodeResult<u64> {
        self.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| u64_decode(self, t))
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

    #[bench]
    fn benchmark_unserialize_i64(b: &mut Bencher) {
        let bytes = Writer::new(true).serialize(&12345).bytes();
        b.iter(|| {
            Reader::new(&bytes).unserialize::<i64>().unwrap();
        });
    }
}
