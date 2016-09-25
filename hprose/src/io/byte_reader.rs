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
 * io/byte_reader.rs                                      *
 *                                                        *
 * byte reader for Rust.                                  *
 *                                                        *
 * LastModified: Sep 25, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::tags::*;
use super::util::utf8_slice_to_str;

use self::ParserError::*;

use std::{io, num, f64};

#[derive(Clone, PartialEq, Debug)]
pub enum ParserError {
    BadUTF8Encode,
    ParseBoolError,
    ParseIntError(num::ParseIntError),
    ParseFloatError(num::ParseFloatError),
    IoError(io::ErrorKind),
}

pub struct ByteReader<'a> {
    pub buf: &'a [u8],
    pub off: usize
}

impl<'a> ByteReader<'a> {
    #[inline]
    pub fn new(buf: &'a [u8]) -> ByteReader<'a> {
        ByteReader {
            buf: buf,
            off: 0
        }
    }

    pub fn next(&mut self, count: usize) -> Result<&[u8], ParserError> {
        let p = self.off + count;
        if p <= self.buf.len() {
            let b = &self.buf[self.off..p];
            self.off = p;
            Ok(b)
        } else {
            Err(IoError(io::ErrorKind::UnexpectedEof))
        }
    }

    pub fn read_byte(&mut self) -> Result<u8, ParserError> {
        if self.off < self.buf.len() {
            let b = self.buf[self.off];
            self.off += 1;
            Ok(b)
        } else {
            Err(IoError(io::ErrorKind::UnexpectedEof))
        }
    }

    pub fn read_i64_with_tag(&mut self, tag: u8) -> Result<i64, ParserError> {
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

    #[inline]
    pub fn read_u64_with_tag(&mut self, tag: u8) -> Result<u64, ParserError> {
        self.read_i64_with_tag(tag).map(|i| i as u64)
    }

    #[inline]
    pub fn read_i64(&mut self) -> Result<i64, ParserError> {
        self.read_i64_with_tag(TAG_SEMICOLON)
    }

    #[inline]
    pub fn read_u64(&mut self) -> Result<u64, ParserError> {
        self.read_u64_with_tag(TAG_SEMICOLON)
    }

    #[inline]
    fn read_length(&mut self) -> Result<usize, ParserError> {
        self.read_i64_with_tag(TAG_QUOTE).map(|i| i as usize)
    }

    #[inline]
    pub fn read_count(&mut self) -> Result<usize, ParserError> {
        self.read_i64_with_tag(TAG_OPENBRACE).map(|i| i as usize)
    }

    pub fn read_until(&mut self, tag: u8) -> Result<&[u8], ParserError> {
        let result = &self.buf[self.off..];
        match result.iter().position(|x| *x == tag) {
            Some(idx) => {
                self.off += idx + 1;
                Ok(&result[..idx])
            },
            None => {
                self.off = self.buf.len();
                Ok(result)
            }
        }
    }

    pub fn read_f32(&mut self) -> Result<f32, ParserError> {
        self.read_until(TAG_SEMICOLON)
            .and_then(|v| utf8_slice_to_str(v).parse::<f32>().map_err(|e| ParseFloatError(e)))
    }

    pub fn read_f64(&mut self) -> Result<f64, ParserError> {
        self.read_until(TAG_SEMICOLON)
            .and_then(|v| utf8_slice_to_str(v).parse::<f64>().map_err(|e| ParseFloatError(e)))
    }

    pub fn read_utf8_slice(&mut self, length: usize) -> Result<&[u8], ParserError> {
        if length == 0 {
            return Ok(&[])
        }
        let p = self.off;
        let mut i: usize = 0;
        while i < length {
            let b = self.buf[self.off];
            match b >> 4 {
                0...7 => self.off += 1,
                12 | 13 => self.off += 2,
                14 => self.off += 3,
                15 => {
                    if b & 8 == 8 {
                        return Err(BadUTF8Encode)
                    }
                    self.off += 4;
                    i += 1
                },
                _ => return Err(BadUTF8Encode)
            }
            i += 1;
        }
        Ok(&self.buf[p..self.off])
    }

    pub fn read_utf8_string(&mut self, length: usize) -> Result<String, ParserError> {
        self.read_utf8_slice(length).map(|s| unsafe { String::from_utf8_unchecked(s.to_owned()) })
    }

    pub fn read_string(&mut self) -> Result<String, ParserError> {
        self.read_length()
            .and_then(|len| self.read_utf8_string(len))
            .and_then(|s| self.read_byte().map(|_| s))
    }

    pub fn read_inf(&mut self) -> Result<f64, ParserError> {
        self.read_byte().map(|sign| if sign == TAG_POS { f64::INFINITY } else { f64::NEG_INFINITY })
    }
}
