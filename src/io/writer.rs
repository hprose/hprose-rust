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
 * LastModified: Sep 19, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate test;
extern crate dtoa;

use std::{i32, f32, f64};
use std::num::FpCategory as Fp;
use std::ptr;
use std::string::String;

use super::Bytes;
use super::tags::*;
use super::util::*;
use super::encoder::*;
use super::writer_refer::WriterRefer;

pub struct Writer {
    buf: Bytes,
    refer: Option<WriterRefer>
}

pub trait ByteWriter {
    fn bytes(&mut self) -> Bytes;
    fn string(&mut self) -> String;
    fn clear(&mut self);
    fn len(&mut self) -> usize;
    fn write_byte(&mut self, tag: u8);
    fn write_from_slice(&mut self, other: &[u8]);
}

impl Writer {
    #[inline]
    pub fn serialize<T: Encodable + ?Sized>(&mut self, v: &T) -> &mut Writer {
        self.write_value(v);
        self
    }

    #[inline]
    pub fn write_value<T: Encodable + ?Sized>(&mut self, v: &T) {
        v.encode(self);
    }

    #[inline]
    pub fn new(simple: bool) -> Writer {
        Writer {
            buf: Vec::with_capacity(1024),
            refer: if simple { None } else { Some(WriterRefer::new()) }
        }
    }

    // private functions

    fn write_string(&mut self, s: &str, length: i64) {
        self.write_byte(TAG_STRING);
        let mut buf: [u8; 20] = [0; 20];
        self.write_from_slice(get_int_bytes(&mut buf, length));
        self.write_byte(TAG_QUOTE);
        self.write_from_slice(s.as_bytes());
        self.write_byte(TAG_QUOTE);
    }

    fn write_list_header(&mut self, len: usize) {
        self.write_byte(TAG_LIST);
        let mut buf: [u8; 20] = [0; 20];
        self.write_from_slice(get_uint_bytes(&mut buf, len as u64));
        self.write_byte(TAG_OPENBRACE);
    }

    #[inline]
    fn write_list_footer(&mut self) {
        self.write_byte(TAG_CLOSEBRACE);
    }

    #[inline]
    fn write_empty_list(&mut self) {
        self.write_from_slice(&[TAG_LIST, TAG_OPENBRACE, TAG_CLOSEBRACE]);
    }
}

impl ByteWriter for Writer {
    fn bytes(&mut self) -> Bytes {
        self.buf.clone()
    }

    fn string(&mut self) -> String {
        String::from_utf8(self.buf.clone()).unwrap()
    }

    #[inline]
    fn clear(&mut self) {
        self.buf.clear();
    }

    #[inline]
    fn len(&mut self) -> usize {
        self.buf.len()
    }

    #[inline]
    fn write_byte(&mut self, tag: u8) {
        self.buf.push(tag);
    }

    fn write_from_slice(&mut self, src: &[u8]) {
        let dst_len = self.len();
        let src_len = src.len();

        self.buf.reserve(src_len);

        unsafe {
            // We would have failed if `reserve` overflowed
            self.buf.set_len(dst_len + src_len);

            ptr::copy_nonoverlapping(
                src.as_ptr(),
                self.buf.as_mut_ptr().offset(dst_len as isize),
                src_len);
        }
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
        let mut buf: [u8; 20] = [0; 20];
        self.write_from_slice(get_int_bytes(&mut buf, i));
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
        let mut buf: [u8; 20] = [0; 20];
        self.write_from_slice(get_uint_bytes(&mut buf, i));
        self.write_byte(TAG_SEMICOLON);
    }

    fn write_f32(&mut self, f: f32) {
        match f.classify() {
            Fp::Nan => self.write_byte(TAG_NAN),
            Fp::Infinite => {
                self.write_byte(TAG_INFINITY);
                self.write_byte(if f == f32::NEG_INFINITY { TAG_NEG } else { TAG_POS });
            },
            _ if f.fract() != 0f32 => {
                self.write_byte(TAG_DOUBLE);
                dtoa::write(&mut self.buf, f).unwrap();
                // self.write_from_slice(f.to_string().as_bytes());
                self.write_byte(TAG_SEMICOLON);
            }
            _ => {
                self.write_byte(TAG_DOUBLE);
                let mut buf: [u8; 20] = [0; 20];
                self.write_from_slice(get_int_bytes(&mut buf, f as i64));
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
            _ if f.fract() != 0f64 => {
                self.write_byte(TAG_DOUBLE);
                dtoa::write(&mut self.buf, f).unwrap();
                // self.write_from_slice(f.to_string().as_bytes());
                self.write_byte(TAG_SEMICOLON);
            }
            _ => {
                self.write_byte(TAG_DOUBLE);
                let mut buf: [u8; 20] = [0; 20];
                self.write_from_slice(get_int_bytes(&mut buf, f as i64));
                self.write_byte(TAG_SEMICOLON);
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
            0 => self.write_byte(TAG_EMPTY),
            - 1 => self.write_bytes(s.as_bytes()),
            1 => {
                self.write_byte(TAG_UTF8_CHAR);
                self.write_from_slice(s.as_bytes());
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
            self.write_from_slice(&[TAG_BYTES, TAG_QUOTE, TAG_QUOTE]);
            return
        }
        self.write_byte(TAG_BYTES);
        let mut buf: [u8; 20] = [0; 20];
        self.write_from_slice(get_int_bytes(&mut buf, count as i64));
        self.write_byte(TAG_QUOTE);
        self.write_from_slice(bytes);
        self.write_byte(TAG_QUOTE);
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

    #[inline]
    fn write_ref<T>(&mut self, p: *const T) -> bool {
        let buf = &mut self.buf;
        self.refer.as_mut().map_or(true, |r| r.write(buf, p))
    }

    #[inline]
    fn set_ref<T>(&mut self, p: *const T) {
        self.refer.as_mut().map(|r| r.set(p));
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::test::Bencher;

    use std::{f32, f64};

    #[test]
    fn test_serialize_bool() {
        let mut w = Writer::new(true);
        w.serialize(&true);
        assert_eq!(w.string(), "t");
        w.clear();
        w.serialize(&false);
        assert_eq!(w.string(), "f");
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
    fn test_serialize_int() {
        let mut w = Writer::new(true);
        w.serialize(&8);
        assert_eq!(w.string(), "8");
        w.clear();
        w.serialize(&88);
        assert_eq!(w.string(), "i88;");
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
            assert_eq!(w.string(), test_case.1);
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
            assert_eq!(w.string(), test_case.1);
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
            ("你好", "s2\"你好\""),
            ("你好啊,hello!", "s10\"你好啊,hello!\""),
            ("🇨🇳", "s4\"🇨🇳\"")
        ];
        let mut w = Writer::new(true);
        for test_case in &test_cases {
            w.serialize(test_case.0);
            assert_eq!(w.string(), test_case.1);
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
            ("hello".as_bytes(), "b5\"hello\""),
            ("".as_bytes(), "b\"\"")
        ];
        let mut w = Writer::new(true);
        for test_case in &test_cases {
            w.serialize(test_case.0);
            assert_eq!(w.string(), test_case.1);
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

    #[bench]
    fn benchmark_serialize_int_slice(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let array = [0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 1, 2, 3, 4, 0, 1, 2, 3, 4];
        let slice = &array[..];
        b.iter(|| {
            w.serialize(slice);
        });
    }
}
