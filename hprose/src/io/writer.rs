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
 * io/writer.rs                                           *
 *                                                        *
 * hprose writer for Rust.                                *
 *                                                        *
 * LastModified: Sep 30, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate dtoa;

use std::fmt::Display;
use std::{i32, i64, f32, f64, ptr};
use std::num::FpCategory as Fp;
use std::string::{String, FromUtf8Error};

use super::{Bytes, ByteWriter};
use super::tags::*;
use super::util::*;
use super::encoder::*;
use super::writer_refer::WriterRefer;

use num::{BigInt, BigUint, Integer, Complex};
use num::rational::Ratio;
use time::Tm;
use uuid::Uuid;

/// Writer is a fine-grained operation struct for Hprose serialization
pub struct Writer {
    byte_writer: ByteWriter,
    refer: Option<WriterRefer>
}

impl Writer {
    /// Constructs a new `Writer`.
    #[inline]
    pub fn new(simple: bool) -> Writer {
        Writer {
            byte_writer: ByteWriter::new(),
            refer: if simple { None } else { Some(WriterRefer::new()) }
        }
    }

    #[inline]
    pub fn bytes(&mut self) -> Bytes {
        self.byte_writer.bytes()
    }

    #[inline]
    pub fn string(&mut self) -> Result<String, FromUtf8Error> {
        self.byte_writer.string()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.byte_writer.clear();
    }

    #[inline]
    pub fn len(&mut self) -> usize {
        self.byte_writer.len()
    }

    #[inline]
    pub fn write_byte(&mut self, b: u8) {
        self.byte_writer.write_byte(b);
    }

    #[inline]
    pub fn write(&mut self, bytes: &[u8]) {
        self.byte_writer.write(bytes);
    }

    #[inline]
    pub fn serialize<T: Encodable + ?Sized>(&mut self, v: &T) -> &mut Writer {
        self.write_value(v);
        self
    }

    #[inline]
    pub fn write_value<T: Encodable + ?Sized>(&mut self, v: &T) {
        v.encode(self);
    }

    pub fn reset(&mut self) {
        self.refer.as_mut().map(|r| r.reset());
    }

    // private functions

    fn write_str_with_len(&mut self, s: &str, len: usize) {
        self.write_byte(TAG_STRING);
        self.write(get_uint_bytes(&mut [0; 20], len as u64));
        self.write_byte(TAG_QUOTE);
        self.write(s.as_bytes());
        self.write_byte(TAG_QUOTE);
    }

    #[inline]
    fn write_empty_bytes(&mut self) {
        self.write(&[TAG_BYTES, TAG_QUOTE, TAG_QUOTE]);
    }

    fn write_date(&mut self, buf: &mut [u8], year: i32, month: i32, day: i32) {
        self.write_byte(TAG_DATE);
        self.write(get_date_bytes(buf, year, month, day));
    }

    fn write_time(&mut self, buf: &mut [u8], hour: i32, min: i32, sec: i32, nsec: i32) {
        self.write_byte(TAG_TIME);
        self.write(get_time_bytes(buf, hour, min, sec));
        if nsec > 0 {
            self.write_byte(TAG_POINT);
            self.write(get_nsec_bytes(buf, nsec));
        }
    }

    fn write_list_header(&mut self, len: usize) {
        self.write_byte(TAG_LIST);
        self.write(get_uint_bytes(&mut [0; 20], len as u64));
        self.write_byte(TAG_OPENBRACE);
    }

    #[inline]
    fn write_list_footer(&mut self) {
        self.write_byte(TAG_CLOSEBRACE);
    }

    #[inline]
    fn write_empty_list(&mut self) {
        self.write(&[TAG_LIST, TAG_OPENBRACE, TAG_CLOSEBRACE]);
    }

    fn write_map_header(&mut self, len: usize) {
        self.write_byte(TAG_MAP);
        self.write(get_uint_bytes(&mut [0; 20], len as u64));
        self.write_byte(TAG_OPENBRACE);
    }

    #[inline]
    fn write_map_footer(&mut self) {
        self.write_byte(TAG_CLOSEBRACE);
    }

    #[inline]
    fn write_empty_map(&mut self) {
        self.write(&[TAG_MAP, TAG_OPENBRACE, TAG_CLOSEBRACE]);
    }
}

impl Encoder for Writer {
    #[inline]
    fn write_nil(&mut self) {
        self.write_byte(TAG_NULL);
    }

    #[inline]
    fn write_bool(&mut self, b: bool) {
        self.write_byte(if b { TAG_TRUE } else { TAG_FALSE });
    }

    fn write_i64(&mut self, i: i64) {
        if i >= 0 && i <= 9 {
            self.write_byte(b'0' + i as u8);
            return
        }
        if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
            self.write_byte(TAG_INTEGER);
        } else {
            self.write_byte(TAG_LONG);
        }
        self.write(get_int_bytes(&mut [0; 20], i));
        self.write_byte(TAG_SEMICOLON);
    }

    fn write_u64(&mut self, i: u64) {
        if i <= 9 {
            self.write_byte(b'0' + i as u8);
            return
        }
        if i <= i32::MAX as u64 {
            self.write_byte(TAG_INTEGER);
        } else {
            self.write_byte(TAG_LONG);
        }
        self.write(get_uint_bytes(&mut [0; 20], i));
        self.write_byte(TAG_SEMICOLON);
    }

    fn write_f32(&mut self, f: f32) {
        match f.classify() {
            Fp::Nan => self.write_byte(TAG_NAN),
            Fp::Infinite => {
                self.write_byte(TAG_INFINITY);
                self.write_byte(if f == f32::NEG_INFINITY { TAG_NEG } else { TAG_POS });
            },
            _ if f.fract() == 0f32 && f >= i64::MIN as f32 && f <= i64::MAX as f32 => {
                self.write_byte(TAG_DOUBLE);
                self.write(get_int_bytes(&mut [0; 20], f as i64));
                self.write_byte(TAG_SEMICOLON);
            },
            _ => {
                self.write_byte(TAG_DOUBLE);
                dtoa::write(&mut self.byte_writer.buf, f).unwrap();
                // self.write(f.to_string().as_bytes());
                self.write_byte(TAG_SEMICOLON);
            }
        };
    }

    fn write_f64(&mut self, f: f64) {
        match f.classify() {
            Fp::Nan => self.write_byte(TAG_NAN),
            Fp::Infinite => {
                self.write_byte(TAG_INFINITY);
                self.write_byte(if f == f64::NEG_INFINITY { TAG_NEG } else { TAG_POS });
            },
            _ if f.fract() == 0f64 && f >= i64::MIN as f64 && f <= i64::MAX as f64 => {
                self.write_byte(TAG_DOUBLE);
                self.write(get_int_bytes(&mut [0; 20], f as i64));
                self.write_byte(TAG_SEMICOLON);
            },
            _ => {
                self.write_byte(TAG_DOUBLE);
                dtoa::write(&mut self.byte_writer.buf, f).unwrap();
                // self.write(f.to_string().as_bytes());
                self.write_byte(TAG_SEMICOLON);
            }
        };
    }

    #[inline]
    fn write_char(&mut self, c: char) {
        self.write_str(&c.to_string());
    }

    fn write_str(&mut self, s: &str) {
        let len = utf16_len(s);
        match len {
            0 => self.write_byte(TAG_EMPTY),
            1 => {
                self.write_byte(TAG_UTF8_CHAR);
                self.write(s.as_bytes());
            },
            _ => {
                self.set_ref(ptr::null::<&str>());
                self.write_str_with_len(s, len)
            }
        }
    }

    fn write_string(&mut self, s: &String) {
        let length = utf16_len(s);
        match length {
            0 => self.write_byte(TAG_EMPTY),
            1 => {
                self.write_byte(TAG_UTF8_CHAR);
                self.write(s.as_bytes());
            },
            _ => {
                if self.write_ref(s) {
                    return
                }
                self.set_ref(s);
                self.write_str_with_len(s, length)
            }
        }
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        let len = bytes.len();
        if len == 0 {
            self.write_empty_bytes();
            return
        }
        self.write_byte(TAG_BYTES);
        self.write(get_uint_bytes(&mut [0; 20], len as u64));
        self.write_byte(TAG_QUOTE);
        self.write(bytes);
        self.write_byte(TAG_QUOTE);
    }

    fn write_bigint(&mut self, i: &BigInt) {
        self.write_byte(TAG_LONG);
        self.write(i.to_str_radix(10).as_bytes());
        self.write_byte(TAG_SEMICOLON);
    }

    fn write_biguint(&mut self, u: &BigUint) {
        self.write_byte(TAG_LONG);
        self.write(u.to_str_radix(10).as_bytes());
        self.write_byte(TAG_SEMICOLON);
    }

    fn write_ratio<T>(&mut self, r: &Ratio<T>) where T: Encodable + Clone + Integer + Display {
        if r.is_integer() {
            self.write_value(&r.to_integer());
        } else {
            let s = r.to_string();
            self.set_ref(ptr::null::<&Ratio<T>>());
            self.write_str_with_len(&s, s.len());
        }
    }

    fn write_complex32(&mut self, c: &Complex<f32>) {
        if c.im == 0.0 {
            self.write_f32(c.re);
            return
        }
        self.set_ref(ptr::null::<&Complex<f32>>());
        self.write_list_header(2);
        self.write_f32(c.re);
        self.write_f32(c.im);
        self.write_list_footer();
    }

    fn write_complex64(&mut self, c: &Complex<f64>) {
        if c.im == 0.0 {
            self.write_f64(c.re);
            return
        }
        self.set_ref(ptr::null::<&Complex<f64>>());
        self.write_list_header(2);
        self.write_f64(c.re);
        self.write_f64(c.im);
        self.write_list_footer();
    }

    fn write_datetime(&mut self, t: &Tm) {
        if self.write_ref(t) {
            return
        }
        self.set_ref(t);
        let mut buf: [u8; 9] = [0; 9];
        if t.tm_hour == 0 && t.tm_min == 0 && t.tm_sec == 0 && t.tm_nsec == 0 {
            self.write_date(&mut buf, 1900 + t.tm_year, t.tm_mon + 1, t.tm_mday);
        } else if t.tm_year == 70 && t.tm_mon == 0 && t.tm_mday == 1 {
            self.write_time(&mut buf, t.tm_hour, t.tm_min, t.tm_sec, t.tm_nsec);
        } else {
            self.write_date(&mut buf, 1900 + t.tm_year, t.tm_mon + 1, t.tm_mday);
            self.write_time(&mut buf, t.tm_hour, t.tm_min, t.tm_sec, t.tm_nsec);
        }
        self.write_byte(if t.tm_utcoff == 0 { TAG_UTC } else { TAG_SEMICOLON });
    }

    fn write_uuid(&mut self, u: &Uuid) {
        if self.write_ref(u) {
            return
        }
        self.set_ref(u);
        self.write_byte(TAG_GUID);
        self.write_byte(TAG_OPENBRACE);
        self.write(u.hyphenated().to_string().as_bytes());
        self.write_byte(TAG_CLOSEBRACE);
    }

    fn write_struct(&mut self, name: &str, len: usize) {
        self.write_byte(TAG_OBJECT);
        self.write_byte(TAG_OPENBRACE);
    }

    fn write_struct_field<T: Encodable>(&mut self, key: &str, value: T) {
        value.encode(self);
    }

    fn write_struct_end(&mut self) {
        self.write_byte(TAG_CLOSEBRACE);
    }

    fn write_option<F>(&mut self, f: F) where F: FnOnce(&mut Writer) {
        f(self);
    }

    fn write_seq<F>(&mut self, len: usize, f: F) where F: FnOnce(&mut Writer) {
        if len == 0 {
            self.write_empty_list();
            return
        }
        self.write_list_header(len);
        f(self);
        self.write_list_footer();
    }

    fn write_map<F>(&mut self, len: usize, f: F) where F: FnOnce(&mut Self) {
        if len == 0 {
            self.write_empty_map();
            return
        }
        self.write_map_header(len);
        f(self);
        self.write_map_footer();
    }

    #[inline]
    fn write_ref<T>(&mut self, p: *const T) -> bool {
        let w = &mut self.byte_writer;
        self.refer.as_mut().map_or(false, |r| r.write(w, p))
    }

    #[inline]
    fn set_ref<T>(&mut self, p: *const T) {
        self.refer.as_mut().map(|r| r.set(p));
    }
}


#[cfg(test)]
mod tests {
    extern crate rand;

    use std::*;
    use std::collections::HashMap;
    use std::marker::PhantomData;

    use self::rand::Rng;

    use super::*;
    use super::super::Hprose;

    use num::{BigInt, BigUint, Complex};
    use num::rational::Ratio;
    use time::strptime;

    macro_rules! t {
        ($value:expr, $result:expr) => {
            assert_eq!(Writer::new(true).serialize(&$value).string().unwrap(), $result);
        }
    }

    #[test]
    fn test_serialize_nil() {
        t!((), "n");
        t!(None::<()>, "n");
        t!(PhantomData::<()>, "n");
    }

    #[test]
    fn test_serialize_bool() {
        t!(true, "t");
        t!(false, "f");
    }

    #[test]
    fn test_serialize_digit() {
        for i in 0..10 {
            t!(i, i.to_string());
        }
    }

    #[test]
    fn test_serialize_int() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let i: i32 = rng.gen_range(10, i32::MAX);
            t!(i, format!("i{};", i));
        }
        for _ in 0..100 {
            let i: i64 = rng.gen_range(i32::MAX as i64 + 1, i64::MAX);
            t!(i, format!("l{};", i));
        }
    }

    #[test]
    fn test_serialize_i8() {
        for i in 0..10 {
            t!(i, i.to_string());
        }
        for i in 10..128 {
            t!(i, format!("i{};", i));
        }
        for i in -128..0 {
            t!(i, format!("i{};", i));
        }
    }

    #[test]
    fn test_serialize_i16() {
        t!(i16::MIN, format!("i{};", i16::MIN));
        t!(i16::MAX, format!("i{};", i16::MAX));
    }

    #[test]
    fn test_serialize_i32() {
        t!(i32::MIN, format!("i{};", i32::MIN));
        t!(i32::MAX, format!("i{};", i32::MAX));
    }

    #[test]
    fn test_serialize_i64() {
        t!(i32::MAX as i64, format!("i{};", i32::MAX));
        t!(i64::MIN, format!("l{};", i64::MIN));
        t!(i64::MAX, format!("l{};", i64::MAX));
    }

    #[test]
    fn test_serialize_uint() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let u: u32 = rng.gen_range(10, i32::MAX as u32);
            t!(u, format!("i{};", u));
        }
        for _ in 0..100 {
            let u: u64 = rng.gen_range(i32::MAX as u64 + 1, u64::MAX);
            t!(u, format!("l{};", u));
        }
    }

    #[test]
    fn test_serialize_u8() {
        for u in 0..10 {
            t!(u, u.to_string());
        }
        for u in 10..256 {
            t!(u, format!("i{};", u));
        }
    }

    #[test]
    fn test_serialize_u16() {
        t!(u16::MAX, format!("i{};", u16::MAX));
    }

    #[test]
    fn test_serialize_u32() {
        t!(u32::MAX, format!("l{};", u32::MAX));
    }

    #[test]
    fn test_serialize_u64() {
        t!(u32::MAX as u64, format!("l{};", u32::MAX));
        t!(u64::MAX, format!("l{};", u64::MAX));
    }

    #[test]
    fn test_serialize_f32() {
        t!(f32::NAN, "N");
        t!(f32::INFINITY, "I+");
        t!(f32::NEG_INFINITY, "I-");
        t!(f32::consts::PI, "d3.1415927;");
    }

    #[test]
    fn test_serialize_f64() {
        t!(f64::NAN, "N");
        t!(f64::INFINITY, "I+");
        t!(f64::NEG_INFINITY, "I-");
        t!(f64::consts::PI, "d3.141592653589793;");
    }

    #[test]
    fn test_serialize_str() {
        t!("", "e");
        t!("π", "uπ");
        t!("你", "u你");
        t!("你好", r#"s2"你好""#);
        t!("你好啊,hello!", r#"s10"你好啊,hello!""#);
        t!("🇨🇳", r#"s4"🇨🇳""#);
    }

    #[test]
    fn test_serialize_bytes() {
        t!(b"hello", r#"b5"hello""#);
        t!(b"", r#"b"""#);
    }

    #[test]
    fn test_serialize_bigint() {
        t!(BigInt::from(-123i64), "l-123;");
        t!(BigUint::from(123u64), "l123;");
    }

    #[test]
    fn test_serialize_ratio() {
        t!(Ratio::from_integer(123i32), "i123;");
        t!(Ratio::from_integer(123i64), "i123;");
        t!(Ratio::from_integer(123isize), "i123;");
        t!(Ratio::from_integer(BigInt::from(123i64)), "l123;");
        t!(Ratio::new(123i32, 2i32), r#"s5"123/2""#);
        t!(Ratio::new(123i64, 2i64), r#"s5"123/2""#);
        t!(Ratio::new(123isize, 2isize), r#"s5"123/2""#);
        t!(Ratio::new(BigInt::from(123i64), BigInt::from(2i64)), r#"s5"123/2""#);
    }

    #[test]
    fn test_serialize_complex() {
        t!(Complex::new(100f32, 0f32), "d100;");
        t!(Complex::new(100f64, 0f64), "d100;");
        t!(Complex::new(0f32, 100f32), "a2{d0;d100;}");
        t!(Complex::new(0f64, 100f64), "a2{d0;d100;}");
    }

    #[test]
    fn test_serialize_datetime() {
        t!(strptime("1980-12-01", "%F").unwrap(), "D19801201Z");
        t!(strptime("1970-01-01 12:34:56Z", "%F %T%z").unwrap(), "T123456Z");
        t!(strptime("1970-01-01 12:34:56.789000000Z", "%F %T.%f%z").unwrap(), "T123456.789Z");
        t!(strptime("1970-01-01 12:34:56.789456000Z", "%F %T.%f%z").unwrap(), "T123456.789456Z");
        t!(strptime("1970-01-01 12:34:56.789456123Z", "%F %T.%f%z").unwrap(), "T123456.789456123Z");
        t!(strptime("1980-12-01 12:34:56Z", "%F %T%z").unwrap(), "D19801201T123456Z");
        t!(strptime("1980-12-01 12:34:56.789000000Z", "%F %T.%f%z").unwrap(), "D19801201T123456.789Z");
        t!(strptime("1980-12-01 12:34:56.789456000Z", "%F %T.%f%z").unwrap(), "D19801201T123456.789456Z");
        t!(strptime("1980-12-01 12:34:56.789456123Z", "%F %T.%f%z").unwrap(), "D19801201T123456.789456123Z");
        t!(strptime("1980-12-01+08:00", "%F%z").unwrap(), "D19801201;");
        t!(strptime("1970-01-01 12:34:56+08:00", "%F %T%z").unwrap(), "T123456;");
        t!(strptime("1980-12-01 12:34:56+08:00", "%F %T%z").unwrap(), "D19801201T123456;");
        t!(strptime("1980-12-01 12:34:56.789456123+08:00", "%F %T.%f%z").unwrap(), "D19801201T123456.789456123;");
    }

    #[test]
    fn test_serialize_tuple() {
        t!((1, 3.14, true), "a3{1d3.14;t}");
    }

    #[test]
    fn test_serialize_array() {
        t!(&[1, 2, 3] as &[i32; 3], "a3{123}");
        t!(&[1.0, 2.0, 3.0], "a3{d1;d2;d3;}");
        t!(&[b'h', b'e', b'l', b'l', b'o'], r#"b5"hello""#);
        t!(&[] as &[u8; 0], r#"b"""#);
        t!(&[Hprose::I64(1), Hprose::F64(2.0), Hprose::Nil, Hprose::Boolean(true)], "a4{1d2;nt}");
        t!(&[true, false, true], "a3{tft}");
        t!(&[] as &[i32; 0], "a{}");
        t!(&[] as &[bool; 0], "a{}");
        t!(&[] as &[Hprose; 0], "a{}");
    }

    #[test]
    fn test_serialize_slice() {
        t!(&[1, 2, 3] as &[i32], "a3{123}");
        t!(&[1.0, 2.0, 3.0] as &[f64], "a3{d1;d2;d3;}");
        t!(&[b'h', b'e', b'l', b'l', b'o'] as &[u8], r#"b5"hello""#);
        t!(&[] as &[u8], r#"b"""#);
        t!(&[Hprose::I64(1), Hprose::F64(2.0), Hprose::Nil, Hprose::Boolean(true)] as &[Hprose], "a4{1d2;nt}");
        t!(&[true, false, true] as &[bool], "a3{tft}");
        t!(&[] as &[i32], "a{}");
        t!(&[] as &[bool], "a{}");
        t!(&[] as &[Hprose], "a{}");
    }

    #[test]
    fn test_serialize_vec() {
        t!(Vec::<Hprose>::new(), "a{}");
        t!(vec![Hprose::I64(1), Hprose::String(String::from("hello")), Hprose::Nil, Hprose::F64(3.14159)], r#"a4{1s5"hello"nd3.14159;}"#);
    }

    #[test]
    fn test_serialize_map() {
        let mut w = Writer::new(true);
        let mut map = HashMap::new();
        map.insert("name", Hprose::String(String::from("Tom")));
        map.insert("age", Hprose::I64(36));
        map.insert("male", Hprose::Boolean(true));
        let expected = [
            r#"m3{s4"name"s3"Tom"s3"age"i36;s4"male"t}"#,
            r#"m3{s3"age"i36;s4"male"ts4"name"s3"Tom"}"#,
            r#"m3{s3"age"i36;s4"name"s3"Tom"s4"male"t}"#,
            r#"m3{s4"name"s3"Tom"s4"male"ts3"age"i36;}"#,
            r#"m3{s4"male"ts3"age"i36;s4"name"s3"Tom"}"#,
            r#"m3{s4"male"ts4"name"s3"Tom"s3"age"i36;}"#
        ];
        let result = w.serialize(&map).string().unwrap();
        assert!(expected.contains(&result.as_str()), "expected one of {:?}, but {} found", expected, result)
    }
}

#[cfg(test)]
mod benchmarks {
    use std::{i32, i64, u64, f32, f64};
    use std::collections::HashMap;

    use test::Bencher;
    use time::strptime;

    use super::*;
    use super::super::Hprose;

    macro_rules! b {
        ($b:expr, $value:expr) => {
            let mut w = Writer::new(true);
            let v = $value;
            $b.bytes = w.serialize(&v).bytes().len() as u64;
            $b.iter(|| {
                w.serialize(&v);
            });
        }
    }

    #[bench]
    fn benchmark_serialize_nil(b: &mut Bencher) {
        b!(b, ());
    }

    #[bench]
    fn benchmark_serialize_bool(b: &mut Bencher) {
        b!(b, true);
    }

    #[bench]
    fn benchmark_serialize_int(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let mut i: i64 = 1;
        b.iter(|| {
            w.serialize(&i);
            i += 1;
        });
    }

    #[bench]
    fn benchmark_serialize_f32(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let mut i: f32 = 1.0;
        b.iter(|| {
            w.serialize(&i);
            i += 1.1;
        });
    }

    #[bench]
    fn benchmark_serialize_f64(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let mut i: f64 = 1.0;
        b.iter(|| {
            w.serialize(&i);
            i += 1.1;
        });
    }

    #[bench]
    fn benchmark_serialize_str(b: &mut Bencher) {
        b!(b, "你好,hello!");
    }

    #[bench]
    fn benchmark_serialize_bytes(b: &mut Bencher) {
        b!(b, "你好,hello!".as_bytes());
    }

    #[bench]
    fn benchmark_serialize_datetime(b: &mut Bencher) {
        b!(b, strptime("1980-01-01 12:34:56.789456123Z", "%F %T.%f%z").unwrap());
    }

    #[bench]
    fn benchmark_serialize_int_array(b: &mut Bencher) {
        b!(b, &[0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 1, 2, 3, 4, 0, 1, 2, 3, 4] as &[i32; 19]);
    }

    #[bench]
    fn benchmark_serialize_int_slice(b: &mut Bencher) {
        b!(b, &[0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 1, 2, 3, 4, 0, 1, 2, 3, 4] as &[i32]);
    }

    #[bench]
    fn benchmark_serialize_vec(b: &mut Bencher) {
        b!(b,  vec![Hprose::I64(1), Hprose::String(String::from("hello")), Hprose::Nil, Hprose::F64(3.14159)]);
    }

    #[bench]
    fn benchmark_serialize_empty_map(b: &mut Bencher) {
        let map: HashMap<i32, i32> = HashMap::new();
        b!(b, map);
    }

    #[bench]
    fn benchmark_serialize_string_key_map(b: &mut Bencher) {
        let mut map = HashMap::new();
        map.insert("name", Hprose::String(String::from("Tom")));
        map.insert("age", Hprose::I64(36));
        map.insert("male", Hprose::Boolean(true));
        b!(b, map);
    }
}
