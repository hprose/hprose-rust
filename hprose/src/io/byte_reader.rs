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
 * LastModified: Sep 27, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::tags::*;
use super::util::utf8_slice_to_str;

use self::ParserError::*;

use std::{io, num, f32, f64};

#[derive(Clone, PartialEq, Debug)]
pub enum ParserError {
    BadUTF8Encode,
    ParseBoolError,
    ParseIntError(num::ParseIntError),
    ParseFloatError(num::ParseFloatError),
    IoError(io::ErrorKind),
}

pub type ParserResult<T> = Result<T, ParserError>;

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

    pub fn next(&mut self, count: usize) -> ParserResult<&[u8]> {
        let p = self.off + count;
        if p <= self.buf.len() {
            let b = &self.buf[self.off..p];
            self.off = p;
            Ok(b)
        } else {
            Err(IoError(io::ErrorKind::UnexpectedEof))
        }
    }

    pub fn read_byte(&mut self) -> ParserResult<u8> {
        if self.off < self.buf.len() {
            let b = self.buf[self.off];
            self.off += 1;
            Ok(b)
        } else {
            Err(IoError(io::ErrorKind::UnexpectedEof))
        }
    }

    #[inline]
    pub fn unread_byte(&mut self) {
        if self.off > 0 {
            self.off -= 1;
        }
    }

    pub fn read_i64_with_tag(&mut self, tag: u8) -> ParserResult<i64> {
        let mut i: i64 = 0;
        let mut b = try!(self.read_byte());
        if b == tag {
            return Ok(i)
        }
        let mut neg = false;
        if b == TAG_NEG {
            neg = true;
            b = try!(self.read_byte());
        } else if b == TAG_POS {
            b = try!(self.read_byte());
        }
        if neg {
            while b != tag {
                i = i.wrapping_mul(10).wrapping_sub((b - b'0') as i64);
                b = try!(self.read_byte());
            }
            Ok(i)
        } else {
            while b != tag {
                i = i.wrapping_mul(10).wrapping_add((b - b'0') as i64);
                b = try!(self.read_byte());
            }
            Ok(i)
        }
    }

    #[inline]
    pub fn read_u64_with_tag(&mut self, tag: u8) -> ParserResult<u64> {
        self.read_i64_with_tag(tag).map(|i| i as u64)
    }

    pub fn read_long_as_f64(&mut self) -> ParserResult<f64> {
        let mut f = 0f64;
        let mut b = try!(self.read_byte());
        if b == TAG_SEMICOLON {
            return Ok(f)
        }
        let mut neg = false;
        if b == TAG_NEG {
            neg = true;
            b = try!(self.read_byte());
        } else if b == TAG_POS {
            b = try!(self.read_byte());
        }
        if neg {
            while b != TAG_SEMICOLON {
                f = f * 10f64 - (b - b'0') as f64;
                b = try!(self.read_byte());
            }
            Ok(f)
        } else {
            while b != TAG_SEMICOLON {
                f = f * 10f64 + (b - b'0') as f64;
                b = try!(self.read_byte());
            }
            Ok(f)
        }
    }

    #[inline]
    pub fn read_i64(&mut self) -> ParserResult<i64> {
        self.read_i64_with_tag(TAG_SEMICOLON)
    }

    #[inline]
    pub fn read_u64(&mut self) -> ParserResult<u64> {
        self.read_u64_with_tag(TAG_SEMICOLON)
    }

    #[inline]
    pub fn read_len(&mut self) -> ParserResult<usize> {
        self.read_i64_with_tag(TAG_QUOTE).map(|i| i as usize)
    }

    pub fn read_until(&mut self, tag: u8) -> ParserResult<&[u8]> {
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

    pub fn read_f32(&mut self) -> ParserResult<f32> {
        self.read_until(TAG_SEMICOLON)
            .and_then(|v| utf8_slice_to_str(v).parse::<f32>().map_err(|e| ParseFloatError(e)))
    }

    pub fn read_f64(&mut self) -> ParserResult<f64> {
        self.read_until(TAG_SEMICOLON)
            .and_then(|v| utf8_slice_to_str(v).parse::<f64>().map_err(|e| ParseFloatError(e)))
    }

    pub fn read_utf8_slice(&mut self, length: usize) -> ParserResult<&[u8]> {
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

    pub fn read_utf8_string(&mut self, length: usize) -> ParserResult<String> {
        self.read_utf8_slice(length).map(|s| unsafe { String::from_utf8_unchecked(s.to_owned()) })
    }

    pub fn read_string(&mut self) -> ParserResult<String> {
        let len = try!(self.read_len());
        let s = self.read_utf8_string(len);
        try!(self.read_byte());
        s
    }

    pub fn read_inf_32(&mut self) -> ParserResult<f32> {
        self.read_byte().map(|sign| if sign == TAG_POS { f32::INFINITY } else { f32::NEG_INFINITY })
    }

    pub fn read_inf_64(&mut self) -> ParserResult<f64> {
        self.read_byte().map(|sign| if sign == TAG_POS { f64::INFINITY } else { f64::NEG_INFINITY })
    }
}
