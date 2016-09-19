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
 * LastModified: Sep 19, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::*;
use super::tags::*;
use super::reader::ParserError;

use std::{io, f64, str};

pub struct ByteReader<'a> {
    buf: &'a Bytes,
    off: usize
}

impl<'a> ByteReader<'a> {
    #[inline]
    pub fn new(buf: &'a Bytes) -> ByteReader<'a> {
        ByteReader {
            buf: buf,
            off: 0
        }
    }

    pub fn read_byte(&mut self) -> Result<u8, ParserError> {
        if self.off >= self.buf.len() {
            return Err(io_error_to_error(io::Error::new(io::ErrorKind::UnexpectedEof, "")))
        }
        let b = self.buf[self.off];
        self.off += 1;
        return Ok(b)
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
    pub fn read_i64(&mut self) -> Result<i64, ParserError> {
        self.read_i64_with_tag(TAG_SEMICOLON)
    }

    #[inline]
    pub fn read_u64(&mut self) -> Result<u64, ParserError> {
        self.read_i64_with_tag(TAG_SEMICOLON).map(|i| i as u64)
    }

    #[inline]
    pub fn read_len(&mut self) -> Result<usize, ParserError> {
        self.read_i64_with_tag(TAG_QUOTE).map(|i| i as usize)
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
                Err(io_error_to_error(io::Error::new(io::ErrorKind::UnexpectedEof, "")))
            }
        }
    }

    pub fn read_f32(&mut self) -> Result<f32, ParserError> {
        unimplemented!()
    }

    pub fn read_f64(&mut self) -> Result<f64, ParserError> {
        self.read_until(TAG_SEMICOLON)
            .and_then(|bytes| unsafe { str::from_utf8_unchecked(bytes) }
                .parse::<f64>()
                .map_err(|e| ParserError::ParseFloatError(e)))
    }

    pub fn read_u8_slice(&mut self, length: usize) -> Result<&[u8], ParserError> {
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

    pub fn read_u8_str(&mut self, length: usize) -> Result<String, ParserError> {
        self.read_u8_slice(length).map(|s| String::from(unsafe { str::from_utf8_unchecked(s).clone() }))
    }

    pub fn read_str(&mut self) -> Result<String, ParserError> {
        self.read_len()
            .and_then(|len| self.read_u8_str(len))
            .and_then(|s| self.read_byte().map(|_| s))
    }

    pub fn read_inf(&mut self) -> Result<f64, ParserError> {
        self.read_byte().map(|sign| if sign == TAG_POS { f64::INFINITY } else { f64::NEG_INFINITY })
    }
}

#[inline]
fn io_error_to_error(io: io::Error) -> ParserError {
    ParserError::IoError(io.kind(), io.to_string())
}
