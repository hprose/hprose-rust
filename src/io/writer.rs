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
 * LastModified: Sep 11, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate test;

use std::collections::HashMap;
use std::i32;
use std::io::Write;
use std::string::String;

use super::tags::*;
use super::util::*;
use super::encoder::*;

pub struct Writer {
    buf: Vec<u8>,
    simple: bool,
    ref_map: HashMap<isize, i32>,
    ref_count: i32
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
        write!(self.buf, "{}", i).unwrap();
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
        write!(self.buf, "{}", i).unwrap();
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
        write!(self.buf, "{}", f).unwrap();
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
        write!(self.buf, "{}", f).unwrap();
        self.buf.push(TAG_SEMICOLON);
    }

    pub fn write_string(&mut self, s: &str) {
        // todo: add length -1 handler
        let length = utf16_length(s);
        match length {
            0 => self.buf.push(TAG_EMPTY),
            1 => {
                self.buf.push(TAG_UTF8_CHAR);
                write!(self.buf, "{}", s).unwrap()
            },
            _ => {
                self.set_writer_ref(0);
                self.write_string_inner(s, length)
            }
        }
    }

    pub fn write_list<T: Encoder>(&mut self, lst: &Vec<T>) {
        let ptr = lst as *const Vec<T> as isize;
        if self.writer_ref(ptr) {
            return
        }
        self.set_writer_ref(ptr);
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

    // private functions

    fn writer_ref(&mut self, p: isize) -> bool {
        if self.simple {
            return false
        }
        match self.ref_map.get(&p) {
            Some(n) => {
                self.buf.push(TAG_REF);
                write!(self.buf, "{}", n).unwrap();
                self.buf.push(TAG_SEMICOLON);
                true
            },
            None => false
        }
    }

    fn set_writer_ref(&mut self, p: isize) {
        if self.simple {
            return
        }
        if p > 0 {
            self.ref_map.insert(p, self.ref_count);
        }
        self.ref_count += 1
    }

    fn write_string_inner(&mut self, s: &str, length: i64) {
        self.buf.push(TAG_STRING);
        write!(self.buf, "{}", length).unwrap();
        self.buf.push(TAG_QUOTE);
        write!(self.buf, "{}", s).unwrap();
        self.buf.push(TAG_QUOTE);
    }

    fn write_list_header(&mut self, count: i64) {
        self.buf.push(TAG_LIST);
        write!(self.buf, "{}", count).unwrap();
        self.buf.push(TAG_OPENBRACE);
    }


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

    pub fn string(&mut self) -> String {
        String::from_utf8(self.buf.clone()).unwrap()
    }

    #[inline]
    pub fn new(simple: bool) -> Writer {
        Writer {
            buf: Vec::with_capacity(1024),
            simple: simple,
            ref_map: HashMap::new(),
            ref_count: 0
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
}
