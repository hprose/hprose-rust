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
 * LastModified: Sep 30, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate test;

use super::*;
use super::tags::*;
use super::util::*;
use super::decoders::*;
use super::reader_refer::ReaderRefer;

use std::convert;
use std::{fmt, f64, str};

use time::{Tm, empty_tm};

/// A set of errors that can occur decoding byte slice
#[derive(Clone, PartialEq, Debug)]
pub enum DecoderError {
    ParserError(ParserError),
    CastError(&'static str, &'static str),
    UnexpectedTag(u8, Option<Bytes>),
    ReferenceError,
    ApplicationError(String),
}

impl convert::From<ParserError> for DecoderError {
    fn from(e: ParserError) -> DecoderError {
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
    pub fn read_byte(&mut self) -> DecodeResult<u8> {
        Ok(try!(self.byte_reader.read_byte()))
    }

    #[inline]
    pub fn unserialize<T: Decodable>(&mut self) -> DecodeResult<T> {
        self.read_value()
    }

    #[inline]
    pub fn read_value<T: Decodable>(&mut self) -> DecodeResult<T> {
        Decodable::decode(self)
    }

    pub fn read_string_without_tag(&mut self) -> DecodeResult<String> {
        let start = self.byte_reader.off - 1;
        let s = try!(self.byte_reader.read_string());
        let reference = &self.byte_reader.buf[start..self.byte_reader.off];
        self.refer.as_mut().map(|mut r| r.set(reference));
        Ok(s)
    }

    pub fn read_bytes_without_tag(&mut self) -> DecodeResult<Bytes> {
        let start = self.byte_reader.off - 1;
        let len = try!(self.byte_reader.read_len());
        let bytes = try!(self.byte_reader.next(len)).to_owned();
        try!(self.read_byte());
        let reference = &self.byte_reader.buf[start..self.byte_reader.off];
        self.refer.as_mut().map(|mut r| r.set(reference));
        Ok(bytes)
    }

    pub fn read_datetime_without_tag(&mut self) -> DecodeResult<Tm> {
        let start = self.byte_reader.off - 1;
        let mut tm = empty_tm();
        let mut tag: u8;
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

    pub fn read_time_without_tag(&mut self) -> DecodeResult<Tm> {
        let start = self.byte_reader.off - 1;
        let mut tm = empty_tm();
        let mut tag: u8;
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

    #[inline]
    pub fn read_count(&mut self) -> DecodeResult<usize> {
        let count = try!(self.byte_reader.read_i64_with_tag(TAG_OPENBRACE));
        Ok(count as usize)
    }

    pub fn reset(&mut self) {
        self.refer.as_mut().map(|r| r.reset());
    }
}

impl<'a> Decoder for Reader<'a> {
    type Error = DecoderError;

    fn read_nil(&mut self) -> DecodeResult<()> {
        let b = try!(self.read_byte());
        if b == TAG_NULL { Ok(()) } else { Err(DecoderError::UnexpectedTag(b, Some(vec!(TAG_NULL)))) }
    }

    fn read_bool(&mut self) -> DecodeResult<bool> {
        let b = try!(self.read_byte());
        bool_decode(self, b)
    }

    fn read_i64(&mut self) -> DecodeResult<i64> {
        let b = try!(self.read_byte());
        i64_decode(self, b)
    }

    fn read_u64(&mut self) -> DecodeResult<u64> {
        let b = try!(self.read_byte());
        u64_decode(self, b)
    }

    fn read_f32(&mut self) -> DecodeResult<f32> {
        let b = try!(self.read_byte());
        f32_decode(self, b)
    }

    fn read_f64(&mut self) -> DecodeResult<f64> {
        let b = try!(self.read_byte());
        f64_decode(self, b)
    }

    fn read_char(&mut self) -> DecodeResult<char> {
        let b = try!(self.read_byte());
        char_decode(self, b)
    }

    fn read_string(&mut self) -> DecodeResult<String> {
        let b = try!(self.read_byte());
        string_decode(self, b)
    }

    fn read_bytes(&mut self) -> DecodeResult<Bytes> {
        let b = try!(self.read_byte());
        bytes_decode(self, b)
    }

    fn read_datetime(&mut self) -> DecodeResult<Tm> {
        let b = try!(self.read_byte());
        time_decode(self, b)
    }

    fn read_option<T, F>(&mut self, mut f: F) -> DecodeResult<T> where F: FnMut(&mut Reader<'a>, bool) -> DecodeResult<T> {
        let b = try!(self.read_byte());
        if b == TAG_NULL {
            f(self, false)
        } else {
            self.byte_reader.unread_byte();
            f(self, true)
        }
    }

    fn read_seq<T, F>(&mut self, f: F) -> DecodeResult<T>
        where T: Decodable, F: FnOnce(&mut Reader<'a>, usize) -> DecodeResult<T>
    {
        let b = try!(self.read_byte());
        seq_decode(self, b, |d, len| f(d, len))
    }

    fn read_map<T, F>(&mut self, f: F) -> DecodeResult<T>
        where T: Decodable, F: FnOnce(&mut Reader<'a>, usize) -> DecodeResult<T>
    {
        let b = try!(self.read_byte());
        map_decode(self, b, |d, len| f(d, len))
    }

    fn read_ref<T: Decodable>(&mut self) -> Result<T, DecoderError> {
        let i = try!(self.byte_reader.read_i64());
        match self.refer {
            Some(ref mut r) => Reader::new(r.read(i as usize), true).unserialize::<T>(),
            None => Err(DecoderError::ReferenceError)
        }
    }

    fn error(&mut self, err: &str) -> DecoderError {
        DecoderError::ApplicationError(err.to_string())
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
    use std::{i32, i64, u32, u64, f32, f64};

    use time::*;

    use super::super::*;

    macro_rules! test {
        ($ty:ty, $($value:expr, $result:expr),+) => (
            let mut w = Writer::new(false);
            $(
                w.serialize(&$value);
            )+
            let bytes = w.bytes();
            let mut r = Reader::new(&bytes, false);
            $(
                assert_eq!(r.unserialize::<$ty>().unwrap(), $result);
            )+
        )
    }

    macro_rules! time {
        ($sec:expr, $nsec:expr) => (at_utc(Timespec::new($sec, $nsec)));
    }

    #[test]
    fn test_unserialize_bool() {
        let true_value = String::from("true");
        test! { bool,
            true,               true,
            false,              false,
            (),                 false,
            "",                 false,
            0,                  false,
            1,                  true,
            9,                  true,
            100,                true,
            100000000000000i64, true,
            0.0,                false,
            "t",                true,
            "f",                false,
            true_value,         true,
            "false",            false,
            true_value,         true
        }
    }

    #[test]
    fn test_unserialize_i64() {
        let int_value = String::from("1234567");
        test! { i64,
            true,                         1,
            false,                        0,
            (),                           0,
            "",                           0,
            0,                            0,
            1,                            1,
            9,                            9,
            100,                          100,
            -100,                         -100,
            i32::MIN,                     i32::MIN as i64,
            i64::MAX,                     i64::MAX,
            i64::MIN,                     i64::MIN,
            u64::MAX,                     u64::MAX as i64,
            0.0,                          0,
            "1",                          1,
            "9",                          9,
            int_value,                    1234567,
            time!(123, 456),              123000000456,
            time!(1234567890, 123456789), 1234567890123456789,
            int_value,                    1234567
        }
    }

    #[test]
    fn test_unserialize_f32() {
        let f32_value = String::from("3.14159");
        test! { f32,
            true,                         1f32,
            false,                        0f32,
            (),                           0f32,
            "",                           0f32,
            0,                            0f32,
            1,                            1f32,
            9,                            9f32,
            100,                          100f32,
            i64::MAX,                     i64::MAX as f32,
            f32::MAX,                     f32::MAX,
            0.0,                          0f32,
            "1",                          1f32,
            "9",                          9f32,
            f32_value,                    3.14159,
            time!(123, 456),              123.000000456f32,
            time!(1234567890, 123456789), 1234567890.123456789f32,
            f32_value,                    3.14159
        }
    }

    #[test]
    fn test_unserialize_f64() {
        let f64_value = String::from("3.14159");
        test! { f64,
            true,                         1f64,
            false,                        0f64,
            (),                           0f64,
            "",                           0f64,
            0,                            0f64,
            1,                            1f64,
            9,                            9f64,
            100,                          100f64,
            f32::MAX,                     3.4028235e38f64,
            f64::MAX,                     f64::MAX,
            0.0,                          0f64,
            "1",                          1f64,
            "9",                          9f64,
            f64_value,                    3.14159,
            time!(123, 456),              123.000000456f64,
            time!(1234567890, 123456789), 1234567890.123456789f64,
            f64_value,                    3.14159
        }
    }

    #[test]
    fn test_unserialize_string() {
        let str_value = "你好";
        let tm1 = strptime("1980-12-01", "%F").unwrap();
        let tm2 = strptime("2006-09-09 12:34:56.789456123Z", "%F %T.%f%z").unwrap();
        let mut tm3 = strptime("1970-01-01 12:34:56.789456123Z", "%F %T.%f%z").unwrap();
        tm3.tm_utcoff = now().tm_utcoff;
        test! { String,
            true,      "true",
            false,     "false",
            (),        "",
            "",        "",
            0,         "0",
            1,         "1",
            9,         "9",
            100,       "100",
            f32::MAX,  "3.4028235e38",
            f64::MAX,  "1.7976931348623157e308",
            0.0,       "0",
            "1",       "1",
            "9",       "9",
            str_value, "你好",
            tm1,       "1980-12-01 00:00:00.000000000 -0000",
            tm2,       "2006-09-09 12:34:56.789456123 -0000",
            tm3,       tm3.strftime("%F %T.%f %z").unwrap().to_string()
        }
    }
}

#[cfg(test)]
mod benchmarks {
    use std::{i32, i64, u64};
    use std::collections::HashMap;

    use test::Bencher;

    use io::*;

    macro_rules! b {
        ($b:expr, $ty:ty, $value:expr) => {
            let v: $ty = $value;
            let bytes = Writer::new(true).serialize(&v).bytes();
            $b.bytes = bytes.len() as u64;
            $b.iter(|| {
                Reader::new(&bytes, true).unserialize::<$ty>().unwrap();
            });
        }
    }

    #[bench]
    fn benchmark_unserialize_bool(b: &mut Bencher) {
        b!(b, bool, true);
    }

    #[bench]
    fn benchmark_unserialize_i64(b: &mut Bencher) {
        b!(b, i64, 12345);
    }

    #[bench]
    fn benchmark_unserialize_string(b: &mut Bencher) {
        b!(b, String, "你好，🇨🇳".to_string());
    }

    #[bench]
    fn benchmark_unserialize_int_array(b: &mut Bencher) {
        b!(b, [i32; 5], [1, 2, 3, 4, 5]);
    }

    #[bench]
    fn benchmark_unserialize_map(b: &mut Bencher) {
        let mut map = HashMap::new();
        map.insert("name".to_string(), "Tom".to_string());
        map.insert("国家".to_string(), "🇨🇳".to_string());
        b!(b, HashMap<String, String>, map);
    }
}
