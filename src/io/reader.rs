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
use super::decoder::{Decodable, Decoder};

#[derive(Clone, PartialEq, Debug)]
pub enum DecoderError {}

pub type DecodeResult<T> = Result<T, DecoderError>;

pub struct Reader {
    buf: Vec<u8>
}

impl Reader {
    #[inline]
    pub fn new(buf: Vec<u8>) -> Reader {
        Reader {
            buf: buf
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

impl Decoder for Reader {
    type Error = DecoderError;

    fn read_nil(&mut self) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn read_bool(&mut self) -> Result<bool, Self::Error> {
        unimplemented!()
    }

    fn read_i64(&mut self) -> Result<i64, Self::Error> {
        unimplemented!()
    }

    fn read_u64(&mut self) -> Result<u64, Self::Error> {
        unimplemented!()
    }

    fn read_f32(&mut self) -> Result<f32, Self::Error> {
        unimplemented!()
    }

    fn read_f64(&mut self) -> Result<f64, Self::Error> {
        unimplemented!()
    }

    fn read_char(&mut self) -> Result<char, Self::Error> {
        unimplemented!()
    }

    fn read_str(&mut self) -> Result<String, Self::Error> {
        unimplemented!()
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>, Self::Error> {
        unimplemented!()
    }

    fn read_option<T, F>(&mut self, f: F) -> Result<T, Self::Error> where F: FnMut(&mut Self, bool) -> Result<T, Self::Error> {
        unimplemented!()
    }

    fn read_seq<T, F>(&mut self, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self, usize) -> Result<T, Self::Error> {
        unimplemented!()
    }
}
