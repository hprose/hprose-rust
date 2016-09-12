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
 * LastModified: Sep 12, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate test;
extern crate dtoa;

use std::i32;
use std::io::Write;
use std::ptr;
use std::string::String;

use super::tags::*;
use super::util::*;
use super::encoder::*;
use super::writer_refer::WriterRefer;

pub struct Writer {
    buf: Vec<u8>,
    refer: Option<WriterRefer>
}

impl Writer {
    pub fn serialize<T: Encoder + ?Sized>(&mut self, v: &T) -> &mut Writer {
        self.write_value(v);
        self
    }

    #[inline]
    pub fn write_value<T: Encoder + ?Sized>(&mut self, v: &T) {
        v.encode(self);
    }

    pub fn write_bool(&mut self, b: bool) {
        self.buf.push(if b { TAG_TRUE } else { TAG_FALSE });
    }

    pub fn write_int(&mut self, i: i64) {
        if i >= 0 && i <= 9 {
            self.buf.push('0' as u8 + i as u8);
            return
        }
        if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
            self.buf.push(TAG_INTEGER);
        } else {
            self.buf.push(TAG_LONG);
        }
        let mut buf: [u8; 20] = [0; 20];
        self.buf.extend_from_slice(get_int_bytes(&mut buf, i));
        self.buf.push(TAG_SEMICOLON);
    }

    pub fn write_uint(&mut self, i: u64) {
        if i <= 9 {
            self.buf.push('0' as u8 + i as u8);
            return
        }
        if i <= i32::MAX as u64 {
            self.buf.push(TAG_INTEGER);
        } else {
            self.buf.push(TAG_LONG);
        }
        let mut buf: [u8; 20] = [0; 20];
        self.buf.extend_from_slice(get_uint_bytes(&mut buf, i));
        self.buf.push(TAG_SEMICOLON);
    }

    pub fn write_float32(&mut self, f: f32) {
        if f.is_nan() {
            self.buf.push(TAG_NAN);
            return
        }
        if f.is_infinite() {
            self.buf.push(TAG_INFINITY);
            self.buf.push(if f.is_sign_negative() { TAG_NEG } else { TAG_POS });
            return
        }
        self.buf.push(TAG_DOUBLE);
        dtoa::write(&mut self.buf, f).unwrap();
        self.buf.push(TAG_SEMICOLON);
    }

    pub fn write_float64(&mut self, f: f64) {
        if f.is_nan() {
            self.buf.push(TAG_NAN);
            return
        }
        if f.is_infinite() {
            self.buf.push(TAG_INFINITY);
            self.buf.push(if f.is_sign_negative() { TAG_NEG } else { TAG_POS });
            return
        }
        self.buf.push(TAG_DOUBLE);
        dtoa::write(&mut self.buf, f).unwrap();
        self.buf.push(TAG_SEMICOLON);
    }

    pub fn write_string(&mut self, s: &str) {
        // todo: add length -1 handler
        let length = utf16_length(s);
        match length {
            0 => self.buf.push(TAG_EMPTY),
            1 => {
                self.buf.push(TAG_UTF8_CHAR);
                self.buf.extend_from_slice(s.as_bytes());
            },
            _ => {
                self.set_ref(ptr::null::<&str>());
                self.write_string_inner(s, length)
            }
        }
    }

    pub fn write_list<T: Encoder>(&mut self, lst: &Vec<T>) {
        if self.write_ref(lst) {
            return
        }
        self.set_ref(lst);
        let count = lst.len();
        if count == 0 {
            self.write_empty_list();
            return
        }
        self.write_list_header(count as i64);
        for v in lst {
            self.serialize(v);
        }
        self.write_list_footer();
    }

    pub fn write_slice<T: Encoder>(&mut self, slice: &[T]) {
        self.set_ref(ptr::null::<&[T]>());
        let count = slice.len();
        if count == 0 {
            self.write_empty_list();
            return
        }
        self.write_list_header(count as i64);
        for v in slice {
            self.serialize(v);
        }
        self.write_list_footer();
    }

    // private functions

    #[inline]
    fn write_ref<T>(&mut self, p: *const T) -> bool {
        let buf = &mut self.buf;
        self.refer.as_mut().map_or(true, |r| r.write(buf, p))
    }

    #[inline]
    fn set_ref<T>(&mut self, p: *const T) {
        self.refer.as_mut().map(|r| r.set(p));
    }

    fn write_string_inner(&mut self, s: &str, length: i64) {
        self.buf.push(TAG_STRING);
        let mut buf: [u8; 20] = [0; 20];
        self.buf.extend_from_slice(get_int_bytes(&mut buf, length));
        self.buf.push(TAG_QUOTE);
        self.buf.extend_from_slice(s.as_bytes());
        self.buf.push(TAG_QUOTE);
    }

    fn write_list_header(&mut self, count: i64) {
        self.buf.push(TAG_LIST);
        let mut buf: [u8; 20] = [0; 20];
        self.buf.extend_from_slice(get_int_bytes(&mut buf, count));
        self.buf.push(TAG_OPENBRACE);
    }

    #[inline]
    fn write_list_footer(&mut self) {
        self.buf.push(TAG_CLOSEBRACE);
    }

    fn write_empty_list(&mut self) {
        self.buf.push(TAG_LIST);
        self.buf.push(TAG_OPENBRACE);
        self.buf.push(TAG_CLOSEBRACE);
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn bytes(&mut self) -> Vec<u8> {
        self.buf.clone()
    }

    pub fn string(&mut self) -> String {
        String::from_utf8(self.buf.clone()).unwrap()
    }

    #[inline]
    pub fn new(simple: bool) -> Writer {
        Writer {
            buf: Vec::with_capacity(1024),
            refer: if simple { None } else { Some(WriterRefer::new()) }
        }
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
    fn test_serialize_float32() {
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
    fn benchmark_serialize_float32(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let mut i: f32 = 1.0;
        b.iter(|| {
            w.serialize(&i);
            i += 1.0;
        });
    }

    #[test]
    fn test_serialize_float64() {
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
    fn benchmark_serialize_float64(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let mut i: f64 = 1.0;
        b.iter(|| {
            w.serialize(&i);
            i += 1.0;
        });
    }

    #[test]
    fn test_serialize_string() {
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
    fn benchmark_serialize_string(b: &mut Bencher) {
        let mut w = Writer::new(true);
        let s = "你好,hello!";
        b.iter(|| {
            w.serialize(s);
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
