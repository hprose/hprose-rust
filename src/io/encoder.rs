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
 * LastModified: Sep 19, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::Hprose;

pub trait Encoder {
    // Primitive types:
    fn write_nil(&mut self);
    fn write_bool(&mut self, v: bool);
    fn write_i64(&mut self, v: i64);
    fn write_u64(&mut self, v: u64);
    fn write_f32(&mut self, v: f32);
    fn write_f64(&mut self, v: f64);
    fn write_char(&mut self, v: char);
    fn write_str(&mut self, v: &str);
    fn write_bytes(&mut self, v: &[u8]);

    // Compound types:


    // Specialized types:
    fn write_option<F>(&mut self, f: F) where F: FnOnce(&mut Self);
    fn write_seq<F>(&mut self, len: usize, f: F) where F: FnOnce(&mut Self);

    // Reference:
    fn write_ref<T>(&mut self, p: *const T) -> bool;
    fn set_ref<T>(&mut self, p: *const T);
}

pub trait Encodable {
    fn encode<W: Encoder>(&self, w: &mut W);
}

impl Encodable for bool {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_bool(*self);
    }
}

impl Encodable for i8 {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_i64(*self as i64);
    }
}

impl Encodable for i16 {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_i64(*self as i64);
    }
}

impl Encodable for i32 {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_i64(*self as i64);
    }
}

impl Encodable for i64 {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_i64(*self);
    }
}

impl Encodable for isize {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_i64(*self as i64);
    }
}

impl Encodable for u8 {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_u64(*self as u64);
    }
}

impl Encodable for u16 {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_u64(*self as u64);
    }
}

impl Encodable for u32 {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_u64(*self as u64);
    }
}

impl Encodable for u64 {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_u64(*self);
    }
}

impl Encodable for usize {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_u64(*self as u64);
    }
}

impl Encodable for f32 {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_f32(*self);
    }
}

impl Encodable for f64 {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_f64(*self);
    }
}

impl Encodable for char {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_char(*self);
    }
}

impl Encodable for str {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_str(self);
    }
}

impl Encodable for String {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_str(self);
    }
}

use std::{mem, ptr};

impl<T: Encodable> Encodable for [T] {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.set_ref(ptr::null::<&[T]>());
        // todo: check i8
        if mem::size_of::<T>() == 1 {
            w.write_bytes(unsafe {
                mem::transmute::<&[T], &[u8]>(self)
            });
        } else {
            w.write_seq(self.len(), |w| {
                for e in self {
                    e.encode(w);
                }
            });
        }
    }
}

impl<T: Encodable> Encodable for Vec<T> {
    fn encode<W: Encoder>(&self, w: &mut W) {
        if w.write_ref(self) {
            return
        }
        w.set_ref(self);
        w.write_seq(self.len(), |w| {
            for e in self {
                e.encode(w);
            }
        });
    }
}

impl<T: Encodable> Encodable for Option<T> {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_option(|w| {
            match *self {
                None => w.write_nil(),
                Some(ref v) => v.encode(w)
            }
        })
    }
}

impl Encodable for Hprose {
    fn encode<W: Encoder>(&self, w: &mut W) {
        match *self {
            Hprose::String(ref s) => s.encode(w),
            _ => ()
        }
    }
}
