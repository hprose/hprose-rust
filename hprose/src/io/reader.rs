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
 * LastModified: Sep 26, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate test;

use super::*;
use super::tags::*;
use super::util::*;
use super::decoders::*;
use super::reader_refer::ReaderRefer;

use std::convert::From;
use std::{fmt, f64, str};

use time::{Tm, empty_tm};

/// A set of errors that can occur decoding byte slice
#[derive(Clone, PartialEq, Debug)]
pub enum DecoderError {
    ParserError(ParserError),
    CastError(&'static str, &'static str),
    UnexpectedTag(u8, Option<Bytes>),
    ReferenceError
}

impl From<ParserError> for DecoderError {
    fn from(e: ParserError) -> Self {
        DecoderError::ParserError(e)
    }
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
    pub refer: Option<ReaderRefer<'a>>
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
        let b = try!(self.byte_reader.read_byte());
        bool_decode(self, b)
    }

    fn read_i64(&mut self) -> DecodeResult<i64> {
        let b = try!(self.byte_reader.read_byte());
        i64_decode(self, b)
    }

    fn read_u64(&mut self) -> DecodeResult<u64> {
        let b = try!(self.byte_reader.read_byte());
        u64_decode(self, b)
    }

    fn read_f32(&mut self) -> DecodeResult<f32> {
        let b = try!(self.byte_reader.read_byte());
        f32_decode(self, b)
    }

    fn read_f64(&mut self) -> DecodeResult<f64> {
        let b = try!(self.byte_reader.read_byte());
        f64_decode(self, b)
    }

    fn read_char(&mut self) -> DecodeResult<char> {
        unimplemented!()
    }

    fn read_string_without_tag(&mut self) -> DecodeResult<String> {
        let start = self.byte_reader.off - 1;
        let s = try!(self.byte_reader.read_string());
        let reference = &self.byte_reader.buf[start..self.byte_reader.off];
        self.refer.as_mut().map(|mut r| r.set(reference));
        Ok(s)
    }

    fn read_string(&mut self) -> DecodeResult<String> {
        let b = try!(self.byte_reader.read_byte());
        string_decode(self, b)
    }

    fn read_bytes(&mut self) -> DecodeResult<Bytes> {
        unimplemented!()
    }

    fn read_datetime_without_tag(&mut self) -> DecodeResult<Tm> {
        let start = self.byte_reader.off - 1;
        let mut tm = empty_tm();
        let mut tag = 0;
        {
            let bytes = try!(self.byte_reader.next(9));
            tm.tm_year = bytes_to_diget4(&bytes[..4]) - 1900;
            tm.tm_mon = bytes_to_diget2(&bytes[4..6]) - 1;
            tm.tm_mday = bytes_to_diget2(&bytes[6..8]);
            tag = bytes[8];
        }
        if tag == TAG_TIME {
            {
                let bytes = try!(self.byte_reader.next(7));
                tm.tm_hour = bytes_to_diget2(&bytes[..2]);
                tm.tm_min = bytes_to_diget2(&bytes[2..4]);
                tm.tm_sec = bytes_to_diget2(&bytes[4..6]);
                tag = bytes[6];
            }
            if tag == TAG_POINT {
                {
                    let bytes = try!(self.byte_reader.next(4));
                    tm.tm_nsec = bytes_to_diget3(&bytes[..3]);
                    tag = bytes[3];
                }
                if (tag >= b'0') && (tag <= b'9') {
                    {
                        let bytes = try!(self.byte_reader.next(3));
                        tm.tm_nsec = tm.tm_nsec * 1000 + (tag - b'0') as i32 * 100 + bytes_to_diget2(&bytes[..2]);
                        tag = bytes[2];
                    }
                    if (tag >= b'0') && (tag <= b'9') {
                        let bytes = try!(self.byte_reader.next(3));
                        tm.tm_nsec = tm.tm_nsec * 1000 + (tag - b'0') as i32 * 100 + bytes_to_diget2(&bytes[..2]);
                        tag = bytes[2];
                    }
                }
            }
        };
        if tag != TAG_UTC {
            tm.tm_utcoff = get_utcoff();
        }
        let reference = &self.byte_reader.buf[start..self.byte_reader.off];
        self.refer.as_mut().map(|mut r| r.set(reference));
        Ok(tm)
    }

    fn read_time_without_tag(&mut self) -> Result<Tm, Self::Error> {
        let start = self.byte_reader.off - 1;
        let mut tm = empty_tm();
        let mut tag = 0;
        tm.tm_year = 70;
        tm.tm_mday = 1;
        {
            let bytes = try!(self.byte_reader.next(7));
            tm.tm_hour = bytes_to_diget2(&bytes[..2]);
            tm.tm_min = bytes_to_diget2(&bytes[2..4]);
            tm.tm_sec = bytes_to_diget2(&bytes[4..6]);
            tag = bytes[6];
        }
        if tag == TAG_POINT {
            {
                let bytes = try!(self.byte_reader.next(4));
                tm.tm_nsec = bytes_to_diget3(&bytes[..3]);
                tag = bytes[3];
            }
            if (tag >= b'0') && (tag <= b'9') {
                {
                    let bytes = try!(self.byte_reader.next(3));
                    tm.tm_nsec = tm.tm_nsec * 1000 + (tag - b'0') as i32 * 100 + bytes_to_diget2(&bytes[..2]);
                    tag = bytes[2];
                }
                if (tag >= b'0') && (tag <= b'9') {
                    let bytes = try!(self.byte_reader.next(3));
                    tm.tm_nsec = tm.tm_nsec * 1000 + (tag - b'0') as i32 * 100 + bytes_to_diget2(&bytes[..2]);
                    tag = bytes[2];
                }
            }
        }
        if tag != TAG_UTC {
            tm.tm_utcoff = get_utcoff();
        }
        let reference = &self.byte_reader.buf[start..self.byte_reader.off];
        self.refer.as_mut().map(|mut r| r.set(reference));
        Ok(tm)
    }

    fn read_option<T, F>(&mut self, f: F) -> DecodeResult<T> where F: FnMut(&mut Reader<'a>, bool) -> DecodeResult<T> {
        unimplemented!()
    }

    fn read_seq<T, F>(&mut self, f: F) -> DecodeResult<T> where F: FnOnce(&mut Reader<'a>, usize) -> DecodeResult<T> {
        unimplemented!()
    }

    fn read_map<T, F>(&mut self, f: F) -> DecodeResult<T>
        where T: Decodable, F: FnOnce(&mut Reader<'a>, usize) -> DecodeResult<T>
    {
        let b = try!(self.byte_reader.read_byte());
        map_decode(self, b, |d, len| f(d, len))
    }

    fn read_ref<T: Decodable>(&mut self) -> Result<T, DecoderError> {
        let i = try!(self.byte_reader.read_i64());
        match self.refer {
            Some(ref mut r) => Reader::new(r.read(i as usize), true).unserialize::<T>(),
            None => Err(DecoderError::ReferenceError)
        }
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

    use std::{i32, i64, u32, u64, f32, f64};
    use std::collections::HashMap;
    use std::mem::transmute;

    use time::{Timespec, at_utc};

    macro_rules! test {
        ($ty:ty, $writer:expr, $($value:expr, $result:expr),+) => (
            $(
                $writer.serialize(&$value);
            )+
            let bytes = $writer.bytes();
            let mut r = Reader::new(&bytes, false);
            $(
                assert_eq!(r.unserialize::<$ty>(), Ok($result));
            )+
        )
    }

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
            .serialize(&100000000000000i64)
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

    #[test]
    fn test_unserialize_i64() {
        let int_value = String::from("1234567");
        let mut w = Writer::new(false);
        test! { i64, w,
            true, 1,
            false, 0,
            (), 0,
            "", 0,
            0, 0,
            1, 1,
            9, 9,
            100, 100,
            -100, -100,
            i32::MIN, i32::MIN as i64,
            i64::MAX, i64::MAX,
            i64::MIN, i64::MIN,
            u64::MAX, u64::MAX as i64,
            0.0, 0,
            "1", 1,
            "9", 9,
            int_value, 1234567,
            at_utc(Timespec::new(123, 456)), 123000000456,
            at_utc(Timespec::new(1234567890, 123456789)), 1234567890123456789,
            int_value, 1234567
        }
    }

    #[bench]
    fn benchmark_unserialize_i64(b: &mut Bencher) {
        let bytes = Writer::new(true).serialize(&12345).bytes();
        b.bytes = bytes.len() as u64;
        b.iter(|| {
            Reader::new(&bytes, true).unserialize::<i64>().unwrap();
        });
    }

    #[test]
    fn test_unserialize_f32() {
        let f32_value = String::from("3.14159");
        let mut w = Writer::new(false);
        test! { f32, w,
            true, 1f32,
            false, 0f32,
            (), 0f32,
            "", 0f32,
            0, 0f32,
            1, 1f32,
            9, 9f32,
            100, 100f32,
            i64::MAX, i64::MAX as f32,
            f32::MAX, f32::MAX,
            0.0, 0f32,
            "1", 1f32,
            "9", 9f32,
            f32_value, 3.14159,
            at_utc(Timespec::new(123, 456)), 123.000000456f32,
            at_utc(Timespec::new(1234567890, 123456789)), 1234567890.123456789f32,
            f32_value, 3.14159
        }
    }

    #[test]
    fn test_unserialize_f64() {
        let f64_value = String::from("3.14159");
        let mut w = Writer::new(false);
        test! { f64, w,
            true, 1f64,
            false, 0f64,
            (), 0f64,
            "", 0f64,
            0, 0f64,
            1, 1f64,
            9, 9f64,
            100, 100f64,
            f32::MAX, f32::MAX as f64,
            f64::MAX, f64::MAX,
            0.0, 0f64,
            "1", 1f64,
            "9", 9f64,
            f64_value, 3.14159,
            at_utc(Timespec::new(123, 456)), 123.000000456f64,
            at_utc(Timespec::new(1234567890, 123456789)), 1234567890.123456789f64,
            f64_value, 3.14159
        }
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
