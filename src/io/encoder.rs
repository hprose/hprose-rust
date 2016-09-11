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
 * io/encoder.rs                                          *
 *                                                        *
 * hprose encoder for Rust.                               *
 *                                                        *
 * LastModified: Sep 11, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::writer::Writer;

pub trait Encoder {
    fn encode(&self, w: &mut Writer);
}

impl Encoder for bool {
    fn encode(&self, w: &mut Writer) {
        w.write_bool(*self);
    }
}

impl Encoder for i8 {
    fn encode(&self, w: &mut Writer) {
        w.write_int(*self as i64);
    }
}

impl Encoder for i16 {
    fn encode(&self, w: &mut Writer) {
        w.write_int(*self as i64);
    }
}

impl Encoder for i32 {
    fn encode(&self, w: &mut Writer) {
        w.write_int(*self as i64);
    }
}

impl Encoder for i64 {
    fn encode(&self, w: &mut Writer) {
        w.write_int(*self);
    }
}

impl Encoder for isize {
    fn encode(&self, w: &mut Writer) {
        w.write_int(*self as i64);
    }
}

impl Encoder for u8 {
    fn encode(&self, w: &mut Writer) {
        w.write_uint(*self as u64);
    }
}

impl Encoder for u16 {
    fn encode(&self, w: &mut Writer) {
        w.write_uint(*self as u64);
    }
}

impl Encoder for u32 {
    fn encode(&self, w: &mut Writer) {
        w.write_uint(*self as u64);
    }
}

impl Encoder for u64 {
    fn encode(&self, w: &mut Writer) {
        w.write_uint(*self);
    }
}

impl Encoder for usize {
    fn encode(&self, w: &mut Writer) {
        w.write_uint(*self as u64);
    }
}

impl Encoder for f32 {
    fn encode(&self, w: &mut Writer) {
        w.write_float32(*self);
    }
}

impl Encoder for f64 {
    fn encode(&self, w: &mut Writer) {
        w.write_float64(*self);
    }
}

impl Encoder for str {
    fn encode(&self, w: &mut Writer) {
        w.write_string(self);
    }
}

impl Encoder for String {
    fn encode(&self, w: &mut Writer) {
        w.write_string(self);
    }
}

impl<T: Encoder> Encoder for Vec<T> {
    fn encode(&self, w: &mut Writer) {
        w.write_list(self);
    }
}
