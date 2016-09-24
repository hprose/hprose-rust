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
 * LastModified: Sep 24, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate test;
extern crate dtoa;

use std::{i32, f32, f64, ptr};
use std::num::FpCategory as Fp;
use std::string::{String, FromUtf8Error};

use super::{Bytes, ByteWriter};
use super::tags::*;
use super::util::*;
use super::encoder::*;
use super::writer_refer::WriterRefer;

/// Writer is a fine-grained operation struct for Hprose serialization
pub struct Writer {
    pub byte_writer: ByteWriter,
    refer: Option<WriterRefer>
}

impl Writer {
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
    pub fn serialize<T: Encodable + ?Sized>(&mut self, v: &T) -> &mut Writer {
        self.write_value(v);
        self
    }

    #[inline]
    pub fn write_value<T: Encodable + ?Sized>(&mut self, v: &T) {
        v.encode(self);
    }

    // private functions

    fn write_string(&mut self, s: &str, length: i64) {
        self.byte_writer.write_byte(TAG_STRING);
        let mut buf: [u8; 20] = [0; 20];
        self.byte_writer.write(get_int_bytes(&mut buf, length));
        self.byte_writer.write_byte(TAG_QUOTE);
        self.byte_writer.write(s.as_bytes());
        self.byte_writer.write_byte(TAG_QUOTE);
    }

    fn write_list_header(&mut self, len: usize) {
        self.byte_writer.write_byte(TAG_LIST);
        let mut buf: [u8; 20] = [0; 20];
        self.byte_writer.write(get_uint_bytes(&mut buf, len as u64));
        self.byte_writer.write_byte(TAG_OPENBRACE);
    }

    #[inline]
    fn write_list_footer(&mut self) {
        self.byte_writer.write_byte(TAG_CLOSEBRACE);
    }

    #[inline]
    fn write_empty_list(&mut self) {
        self.byte_writer.write(&[TAG_LIST, TAG_OPENBRACE, TAG_CLOSEBRACE]);
    }

    fn write_map_header(&mut self, len: usize) {
        self.byte_writer.write_byte(TAG_MAP);
        let mut buf: [u8; 20] = [0; 20];
        self.byte_writer.write(get_uint_bytes(&mut buf, len as u64));
        self.byte_writer.write_byte(TAG_OPENBRACE);
    }

    #[inline]
    fn write_map_footer(&mut self) {
        self.byte_writer.write_byte(TAG_CLOSEBRACE);
    }

    #[inline]
    fn write_empty_map(&mut self) {
        self.byte_writer.write(&[TAG_MAP, TAG_OPENBRACE, TAG_CLOSEBRACE]);
    }
}

impl Encoder for Writer {
    #[inline]
    fn write_nil(&mut self) {
        self.byte_writer.write_byte(TAG_NULL);
    }

    #[inline]
    fn write_bool(&mut self, b: bool) {
        self.byte_writer.write_byte(if b { TAG_TRUE } else { TAG_FALSE });
    }

    fn write_i64(&mut self, i: i64) {
        if i >= 0 && i <= 9 {
            self.byte_writer.write_byte(b'0' + i as u8);
            return
        }
        if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
            self.byte_writer.write_byte(TAG_INTEGER);
        } else {
            self.byte_writer.write_byte(TAG_LONG);
        }
        let mut buf: [u8; 20] = [0; 20];
        self.byte_writer.write(get_int_bytes(&mut buf, i));
        self.byte_writer.write_byte(TAG_SEMICOLON);
    }

    fn write_u64(&mut self, i: u64) {
        if i <= 9 {
            self.byte_writer.write_byte(b'0' + i as u8);
            return
        }
        if i <= i32::MAX as u64 {
            self.byte_writer.write_byte(TAG_INTEGER);
        } else {
            self.byte_writer.write_byte(TAG_LONG);
        }
        let mut buf: [u8; 20] = [0; 20];
        self.byte_writer.write(get_uint_bytes(&mut buf, i));
        self.byte_writer.write_byte(TAG_SEMICOLON);
    }

    fn write_f32(&mut self, f: f32) {
        match f.classify() {
            Fp::Nan => self.byte_writer.write_byte(TAG_NAN),
            Fp::Infinite => {
                self.byte_writer.write_byte(TAG_INFINITY);
                self.byte_writer.write_byte(if f == f32::NEG_INFINITY { TAG_NEG } else { TAG_POS });
            },
            _ if f.fract() != 0f32 => {
                self.byte_writer.write_byte(TAG_DOUBLE);
                dtoa::write(&mut self.byte_writer.buf, f).unwrap();
                // self.byte_writer.write_from_slice(f.to_string().as_bytes());
                self.byte_writer.write_byte(TAG_SEMICOLON);
            }
            _ => {
                self.byte_writer.write_byte(TAG_DOUBLE);
                let mut buf: [u8; 20] = [0; 20];
                self.byte_writer.write(get_int_bytes(&mut buf, f as i64));
                self.byte_writer.write_byte(TAG_SEMICOLON);
            }
        };
    }

    fn write_f64(&mut self, f: f64) {
        match f.classify() {
            Fp::Nan => self.byte_writer.write_byte(TAG_NAN),
            Fp::Infinite => {
                self.byte_writer.write_byte(TAG_INFINITY);
                self.byte_writer.write_byte(if f == f64::NEG_INFINITY { TAG_NEG } else { TAG_POS });
            },
            _ if f.fract() != 0f64 => {
                self.byte_writer.write_byte(TAG_DOUBLE);
                dtoa::write(&mut self.byte_writer.buf, f).unwrap();
                // self.byte_writer.write_from_slice(f.to_string().as_bytes());
                self.byte_writer.write_byte(TAG_SEMICOLON);
            }
            _ => {
                self.byte_writer.write_byte(TAG_DOUBLE);
                let mut buf: [u8; 20] = [0; 20];
                self.byte_writer.write(get_int_bytes(&mut buf, f as i64));
                self.byte_writer.write_byte(TAG_SEMICOLON);
            }
        };
    }

    #[inline]
    fn write_char(&mut self, c: char) {
        let s = c.to_string();
        self.write_str(&s);
    }

    fn write_str(&mut self, s: &str) {
        let length = utf16_length(s);
        match length {
            0 => self.byte_writer.write_byte(TAG_EMPTY),
            - 1 => self.write_bytes(s.as_bytes()),
            1 => {
                self.byte_writer.write_byte(TAG_UTF8_CHAR);
                self.byte_writer.write(s.as_bytes());
            },
            _ => {
                self.set_ref(ptr::null::<&str>());
                self.write_string(s, length)
            }
        }
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        let count = bytes.len();
        if count == 0 {
            self.byte_writer.write(&[TAG_BYTES, TAG_QUOTE, TAG_QUOTE]);
            return
        }
        self.byte_writer.write_byte(TAG_BYTES);
        let mut buf: [u8; 20] = [0; 20];
        self.byte_writer.write(get_int_bytes(&mut buf, count as i64));
        self.byte_writer.write_byte(TAG_QUOTE);
        self.byte_writer.write(bytes);
        self.byte_writer.write_byte(TAG_QUOTE);
    }

    fn write_struct(&mut self, name: &str, len: usize) {
        self.byte_writer.write_byte(TAG_OBJECT);
        self.byte_writer.write_byte(TAG_OPENBRACE);
    }

    fn write_struct_field<T: Encodable>(&mut self, key: &str, value: T) {
        value.encode(self);
    }

    fn write_struct_end(&mut self) {
        self.byte_writer.write_byte(TAG_CLOSEBRACE);
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
        let buf = &mut self.byte_writer.buf;
        self.refer.as_mut().map_or(false, |r| r.write(buf, p))
    }

    #[inline]
    fn set_ref<T>(&mut self, p: *const T) {
        self.refer.as_mut().map(|r| r.set(p));
    }
}


#[cfg(test)]
mod tests {
    extern crate rand;

    use self::rand::Rng;

    use super::*;
    use super::super::{Hprose, Encodable};
    use super::test::Bencher;

    use std::{i32, i64, u64, f32, f64};
    use std::collections::{HashSet, HashMap};

    #[test]
    fn test_serialize_nil() {
        let mut w = Writer::new(true);
        w.serialize(&());
        assert_eq!(w.string().unwrap(), "n");
    }

    #[bench]
    fn benchmark_serialize_nil(b: &mut Bencher) {
        let mut w = Writer::new(true);
        b.bytes = 1;
        b.iter(|| {
            w.serialize(&());
        });
    }

    #[test]
    fn test_serialize_bool() {
        let mut w = Writer::new(true);
        w.serialize(&true);
        assert_eq!(w.string().unwrap(), "t");
        w.clear();
        w.serialize(&false);
        assert_eq!(w.string().unwrap(), "f");
    }

    #[bench]
    fn benchmark_serialize_bool(b: &mut Bencher) {
        let mut w = Writer::new(true);
        b.bytes = 1;
        b.iter(|| {
            w.serialize(&true);
        });
    }

    #[test]
    fn test_serialize_digit() {
        let mut w = Writer::new(true);
        for i in 0..10 {
            w.clear();
            w.serialize(&i);
            assert_eq!(w.string().unwrap(), i.to_string());
        }
    }

    #[test]
    fn test_serialize_int() {
        let mut w = Writer::new(true);
        let mut rng = rand::thread_rng();
        for i in 0..100 {
            w.clear();
            let x: i32 = rng.gen_range(10, i32::MAX);
            w.serialize(&x);
            assert_eq!(w.string().unwrap(), format!("i{};", x));
        }
        for i in 0..100 {
            w.clear();
            let x: i64 = rng.gen_range(i32::MAX as i64 + 1, i64::MAX);
            w.serialize(&x);
            assert_eq!(w.string().unwrap(), format!("l{};", x));
        }
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

    #[test]
    fn test_serialize_i8() {
        let mut w = Writer::new(true);
        for i in 0..10 {
            w.clear();
            w.serialize(&(i as i8));
            assert_eq!(w.string().unwrap(), i.to_string());
        }
        for i in 10..128 {
            w.clear();
            w.serialize(&(i as i8));
            assert_eq!(w.string().unwrap(), format!("i{};", i));
        }
        for i in -128..0 {
            w.clear();
            w.serialize(&(i as i8));
            assert_eq!(w.string().unwrap(), format!("i{};", i));
        }
    }

    #[test]
    fn test_serialize_uint() {
        let mut w = Writer::new(true);
        let mut rng = rand::thread_rng();
        for u in 0..100 {
            w.clear();
            let x: u32 = rng.gen_range(10, i32::MAX as u32);
            w.serialize(&x);
            assert_eq!(w.string().unwrap(), format!("i{};", x));
        }
        for u in 0..100 {
            w.clear();
            let x: u64 = rng.gen_range(i32::MAX as u64 + 1, u64::MAX);
            w.serialize(&x);
            assert_eq!(w.string().unwrap(), format!("l{};", x));
        }
    }

    #[test]
    fn test_serialize_u8() {
        let mut w = Writer::new(true);
        for u in 0..10 {
            w.clear();
            w.serialize(&(u as u8));
            assert_eq!(w.string().unwrap(), u.to_string());
        }
        for u in 10..256 {
            w.clear();
            w.serialize(&(u as u8));
            assert_eq!(w.string().unwrap(), format!("i{};", u));
        }
    }

    #[test]
    fn test_serialize_f32() {
        let test_cases = [
            (f32::NAN, "N"),
            (f32::INFINITY, "I+"),
            (f32::NEG_INFINITY, "I-"),
            (f32::consts::PI, "d3.1415927;")
        ];
        let mut w = Writer::new(true);
        for test_case in &test_cases {
            w.serialize(&test_case.0);
            assert_eq!(w.string().unwrap(), test_case.1);
            w.clear();
        }
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

    #[test]
    fn test_serialize_f64() {
        let test_cases = [
            (f64::NAN, "N"),
            (f64::INFINITY, "I+"),
            (f64::NEG_INFINITY, "I-"),
            (f64::consts::PI, "d3.141592653589793;")
        ];
        let mut w = Writer::new(true);
        for test_case in &test_cases {
            w.serialize(&test_case.0);
            assert_eq!(w.string().unwrap(), test_case.1);
            w.clear();
        }
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

    #[test]
    fn test_serialize_str() {
        let test_cases = [
            ("", "e"),
            ("π", "uπ"),
            ("你", "u你"),
            ("你好", r#"s2"你好""#),
            ("你好啊,hello!", r#"s10"你好啊,hello!""#),
            ("🇨🇳", r#"s4"🇨🇳""#)
        ];
        let mut w = Writer::new(true);
        for test_case in &test_cases {
            w.serialize(test_case.0);
            assert_eq!(w.string().unwrap(), test_case.1);
            w.clear();
        }
    }

    #[bench]
    fn benchmark_serialize_str(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let s = "你好,hello!";
        b.bytes = s.len() as u64;
        b.iter(|| {
            w.serialize(s);
        });
    }

    #[test]
    fn test_serialize_bytes() {
        let test_cases = [
            ("hello".as_bytes(), r#"b5"hello""#),
            ("".as_bytes(), r#"b"""#)
        ];
        let mut w = Writer::new(true);
        for test_case in &test_cases {
            w.serialize(test_case.0);
            assert_eq!(w.string().unwrap(), test_case.1);
            w.clear();
        }
    }

    #[bench]
    fn benchmark_serialize_bytes(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let bytes = "你好,hello!".as_bytes();
        b.bytes = bytes.len() as u64;
        b.iter(|| {
            w.serialize(bytes);
        });
    }

    #[test]
    fn test_serialize_array() {
        let mut w = Writer::new(true);
        test::<[i32; 3]>(&mut w, [1, 2, 3], "a3{123}");
        test(&mut w, [1.0, 2.0, 3.0], "a3{d1;d2;d3;}");
        test(&mut w, [b'h', b'e', b'l', b'l', b'o'], r#"b5"hello""#);
        test::<[u8; 0]>(&mut w, [], r#"b"""#);
        test(&mut w, [Hprose::I64(1), Hprose::F64(2.0), Hprose::Nil, Hprose::Boolean(true)], "a4{1d2;nt}");
        test(&mut w, [true, false, true], "a3{tft}");
        test::<[i32; 0]>(&mut w, [], "a{}");
        test::<[bool; 0]>(&mut w, [], "a{}");
        test::<[Hprose; 0]>(&mut w, [], "a{}");
    }

    #[bench]
    fn benchmark_serialize_int_array(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let array = [0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 1, 2, 3, 4, 0, 1, 2, 3, 4];
        b.iter(|| {
            w.serialize(&array);
        });
    }

    #[test]
    fn test_serialize_slice() {
        let mut w = Writer::new(true);
        test::<&[i32]>(&mut w, &[1, 2, 3], "a3{123}");
        test::<&[f64]>(&mut w, &[1.0, 2.0, 3.0], "a3{d1;d2;d3;}");
        test::<&[u8]>(&mut w, &[b'h', b'e', b'l', b'l', b'o'], r#"b5"hello""#);
        test::<&[u8]>(&mut w, &[], r#"b"""#);
        test::<&[Hprose]>(&mut w, &[Hprose::I64(1), Hprose::F64(2.0), Hprose::Nil, Hprose::Boolean(true)], "a4{1d2;nt}");
        test::<&[bool]>(&mut w, &[true, false, true], "a3{tft}");
        test::<&[i32]>(&mut w, &[], "a{}");
        test::<&[bool]>(&mut w, &[], "a{}");
        test::<&[Hprose]>(&mut w, &[], "a{}");
    }

    #[bench]
    fn benchmark_serialize_int_slice(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let array = [0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 1, 2, 3, 4, 0, 1, 2, 3, 4];
        let slice = &array[..];
        b.iter(|| {
            w.serialize(slice);
        });
    }

    #[test]
    fn test_serialize_vec() {
        let mut w = Writer::new(true);
        let mut v: Vec<Hprose> = Vec::new();
        assert_eq!(w.serialize(&v).string().unwrap(), "a{}");
        w.clear();
        let mut v = vec![Hprose::I64(1), Hprose::String(String::from("hello")), Hprose::Nil, Hprose::F64(3.14159)];
        assert_eq!(w.serialize(&v).string().unwrap(), r#"a4{1s5"hello"nd3.14159;}"#);
    }

    #[bench]
    fn benchmark_serialize_vec(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let mut v = vec![Hprose::I64(1), Hprose::String(String::from("hello")), Hprose::Nil, Hprose::F64(3.14159)];
        b.iter(|| {
            w.serialize(&v);
        });
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

    #[bench]
    fn benchmark_serialize_empty_map(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let mut map: HashMap<i32, i32> = HashMap::new();
        b.iter(|| {
            w.serialize(&map);
        });
    }

    #[bench]
    fn benchmark_serialize_string_key_map(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let mut map = HashMap::new();
        map.insert("name", Hprose::String(String::from("Tom")));
        map.insert("age", Hprose::I64(36));
        map.insert("male", Hprose::Boolean(true));
        b.iter(|| {
            w.serialize(&map);
        });
    }

    fn test<T: Encodable>(w: &mut Writer, v: T, expected: &str) {
        w.clear();
        assert_eq!(w.serialize(&v).string().unwrap(), expected);
    }
}
