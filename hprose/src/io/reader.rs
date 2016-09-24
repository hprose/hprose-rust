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
 * LastModified: Sep 24, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate test;

use super::tags::*;
use super::*;

use super::bool_decoder::bool_decode;
use super::i64_decoder::i64_decode;
use super::u64_decoder::u64_decode;
use super::f32_decoder::f32_decode;
use super::f64_decoder::f64_decode;
use super::string_decoder::string_decode;
use super::map_decoder::map_decode;
use super::reader_refer::ReaderRefer;

use std::fmt;
use std::f64;
use std::str;

/// A set of errors that can occur decoding byte slice
#[derive(Clone, PartialEq, Debug)]
pub enum DecoderError {
    ParserError(ParserError),
    CastError(&'static str, &'static str),
    UnexpectedTag(u8, Option<Bytes>),
    ReferenceError
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

/// Reader is a fine-grained operation struct for Hprose unserialization
pub struct Reader<'a> {
    pub byte_reader: ByteReader<'a>,
    refer: Option<ReaderRefer<'a>>
}

impl<'a> Reader<'a> {
    #[inline]
    pub fn new(buf: &'a [u8], simple: bool) -> Reader<'a> {
        Reader {
            byte_reader: ByteReader::new(buf),
            refer: if simple { None } else { Some(ReaderRefer::new()) }
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
        self.byte_reader.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| bool_decode(self, t))
    }

    fn read_i64(&mut self) -> DecodeResult<i64> {
        self.byte_reader.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| i64_decode(self, t))
    }

    fn read_u64(&mut self) -> DecodeResult<u64> {
        self.byte_reader.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| u64_decode(self, t))
    }

    fn read_f32(&mut self) -> DecodeResult<f32> {
        self.byte_reader.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| f32_decode(self, t))
    }

    fn read_f64(&mut self) -> DecodeResult<f64> {
        self.byte_reader.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| f64_decode(self, t))
    }

    fn read_char(&mut self) -> DecodeResult<char> {
        unimplemented!()
    }

    fn read_string_without_tag(&mut self) -> DecodeResult<String> {
        let start = self.byte_reader.off - 1;
        let result = self.byte_reader.read_string().map_err(|e| DecoderError::ParserError(e));
        let reference = &self.byte_reader.buf[start..self.byte_reader.off];
        self.refer.as_mut().map(|mut r| r.set(reference));
        result
    }

    fn read_string(&mut self) -> DecodeResult<String> {
        self.byte_reader.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| string_decode(self, t))
    }

    fn read_bytes(&mut self) -> DecodeResult<Bytes> {
        unimplemented!()
    }

    fn read_option<T, F>(&mut self, f: F) -> DecodeResult<T> where F: FnMut(&mut Reader<'a>, bool) -> DecodeResult<T> {
        unimplemented!()
    }

    fn read_seq<T, F>(&mut self, f: F) -> DecodeResult<T> where F: FnOnce(&mut Reader<'a>, usize) -> DecodeResult<T> {
        unimplemented!()
    }

    fn read_map<T, F>(&mut self, f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Reader<'a>, usize) -> DecodeResult<T>
    {
        self.byte_reader.read_byte().map_err(|e| DecoderError::ParserError(e)).and_then(|t| map_decode(self, t, |d, len| f(d, len)))
    }

    fn read_ref<T: Decodable>(&mut self) -> Result<T, DecoderError> {
        self.read_i64().and_then(|i| {
            match self.refer {
                Some(ref mut r) => Reader::new(r.read(i as usize), true).unserialize::<T>(),
                None => Err(DecoderError::ReferenceError)
            }
        })
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

    use std::collections::HashMap;

    #[test]
    fn test_unserialize_bool() {
        let true_value = String::from("true");
        let mut w = Writer::new(false);
        w.serialize(&true)
            .serialize(&false)
            .serialize(&())
            .serialize(&"")
            .serialize(&0)
            .serialize(&1)
            .serialize(&9)
            .serialize(&100)
            .serialize(&100000000000000)
            .serialize(&0.0)
            .serialize(&"t")
            .serialize(&"f")
            .serialize(&true_value)
            .serialize(&"false")
            .serialize(&true_value);
        let results = [true, false, false, false, false, true, true, true, true, false, true, false, true, false, true];
        let bytes = w.bytes();
        let mut r = Reader::new(&bytes, false);
        for result in &results {
            assert_eq!(r.unserialize::<bool>(), Ok(*result));
        }
    }

    #[bench]
    fn benchmark_unserialize_bool(b: &mut Bencher) {
        let bytes = Writer::new(true).serialize(&true).bytes();
        b.bytes = bytes.len() as u64;
        b.iter(|| {
            Reader::new(&bytes, true).unserialize::<bool>().unwrap();
        });
    }

    #[bench]
    fn benchmark_unserialize_i64(b: &mut Bencher) {
        let bytes = Writer::new(true).serialize(&12345).bytes();
        b.bytes = bytes.len() as u64;
        b.iter(|| {
            Reader::new(&bytes, true).unserialize::<i64>().unwrap();
        });
    }

    #[bench]
    fn benchmark_unserialize_str(b: &mut Bencher) {
        let bytes = Writer::new(true).serialize("你好，🇨🇳").bytes();
        b.bytes = bytes.len() as u64;
        b.iter(|| {
            Reader::new(&bytes, true).unserialize::<String>().unwrap();
        });
    }

    #[bench]
    fn benchmark_unserialize_map(b: &mut Bencher) {
        let mut map = HashMap::new();
        map.insert("name", "Tom");
        map.insert("国家", "🇨🇳");
        let bytes = Writer::new(true).serialize(&map).bytes();
        b.bytes = bytes.len() as u64;
        b.iter(|| {
            Reader::new(&bytes, true).unserialize::<HashMap<String, String>>().unwrap();
        });
    }
}
