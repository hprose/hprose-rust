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

use std::i32;
use std::io::Write;
use std::string::String;

use super::tags::*;
use super::encoder::*;

pub struct Writer {
    buf: Vec<u8>
}

impl Writer {
    pub fn serialize<T: Encoder>(&mut self, v: T) -> &mut Writer {
        self.write_value(v);
        self
    }

    pub fn write_value<T: Encoder>(&mut self, v: T) {
        v.encode(self);
    }

    pub fn write_bool(&mut self, b: bool) {
        self.buf.push(if b { TagTrue } else { TagFalse });
    }

    pub fn write_int(&mut self, i: i64) {
        if i >= 0 && i <= 9 {
            self.buf.push('0' as u8 + i as u8);
            return
        }
        if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
            self.buf.push(TagInteger);
        } else {
            self.buf.push(TagLong);
        }
        write!(self.buf, "{}", i);
        self.buf.push(TagSemicolon);
    }

    pub fn write_uint(&mut self, i: u64) {
        if i <= 9 {
            self.buf.push('0' as u8 + i as u8);
            return
        }
        if i <= i32::MAX as u64 {
            self.buf.push(TagInteger);
        } else {
            self.buf.push(TagLong);
        }
        write!(self.buf, "{}", i);
        self.buf.push(TagSemicolon);
    }

    pub fn write_float32(&mut self, f: f32) {
        if f.is_nan() {
            self.buf.push(TagNaN);
            return
        }
        if f.is_infinite() {
            self.buf.push(TagInfinity);
            self.buf.push(if f.is_sign_negative() { TagNeg } else { TagPos });
            return
        }
        self.buf.push(TagDouble);
        write!(self.buf, "{}", f);
        self.buf.push(TagSemicolon);
    }

    pub fn write_float64(&mut self, f: f64) {
        if f.is_nan() {
            self.buf.push(TagNaN);
            return
        }
        if f.is_infinite() {
            self.buf.push(TagInfinity);
            self.buf.push(if f.is_sign_negative() { TagNeg } else { TagPos });
            return
        }
        self.buf.push(TagDouble);
        write!(self.buf, "{}", f);
        self.buf.push(TagSemicolon);
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn string(&mut self) -> String {
        String::from_utf8(self.buf.clone()).unwrap()
    }

    #[inline]
    pub fn new() -> Writer {
        Writer {
            buf: Vec::with_capacity(1024)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::super::tags::*;
    use super::*;
    use super::test::Bencher;

    use std::{f32, f64};

    #[test]
    fn test_serialize_bool() {
        let mut w = Writer::new();
        w.serialize(true);
        assert_eq!(w.string(), "t");
        w.clear();
        w.serialize(false);
        assert_eq!(w.string(), "f");
    }

    #[bench]
    fn benchmark_serialize_bool(b: &mut Bencher) {
        let mut w = Writer::new();
        b.iter(|| {
            w.serialize(true);
        });
    }

    #[test]
    fn test_serialize_int() {
        let mut w = Writer::new();
        w.serialize(8);
        assert_eq!(w.string(), "8");
        w.clear();
        w.serialize(88);
        assert_eq!(w.string(), "i88;");
    }

    #[bench]
    fn benchmark_serialize_int(b: &mut Bencher) {
        let mut w = Writer::new();
        let mut i: i64 = 1;
        b.iter(|| {
            w.serialize(i);
            i += 1;
        });
    }

    #[test]
    fn test_serialize_float32() {
        let testCases = [
            (f32::NAN, "N"),
            (f32::INFINITY, "I+"),
            (f32::NEG_INFINITY, "I-"),
            (f32::consts::PI, "d3.1415927;")
        ];
        let mut w = Writer::new();
        for testCase in &testCases {
            w.serialize(testCase.0);
            assert_eq!(w.string(), testCase.1);
            w.clear();
        }
    }

    #[bench]
    fn benchmark_serialize_float32(b: &mut Bencher) {
        let mut w = Writer::new();
        let mut i: f32 = 1.0;
        b.iter(|| {
            w.serialize(i);
            i += 1.0;
        });
    }

    #[test]
    fn test_serialize_float64() {
        let testCases = [
            (f64::NAN, "N"),
            (f64::INFINITY, "I+"),
            (f64::NEG_INFINITY, "I-"),
            (f64::consts::PI, "d3.141592653589793;")
        ];
        let mut w = Writer::new();
        for testCase in &testCases {
            w.serialize(testCase.0);
            assert_eq!(w.string(), testCase.1);
            w.clear();
        }
    }

    #[bench]
    fn benchmark_serialize_float64(b: &mut Bencher) {
        let mut w = Writer::new();
        let mut i: f64 = 1.0;
        b.iter(|| {
            w.serialize(i);
            i += 1.0;
        });
    }
}
