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
 * LastModified: Sep 22, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::Hprose;

use std::rc::Rc;
use std::sync::Arc;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::cell::{Cell, RefCell};
use std::hash::{Hash, BuildHasher};
use std::collections::{LinkedList, VecDeque, BTreeMap, BTreeSet, HashMap, HashSet};

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
    fn write_map<F>(&mut self, len: usize, f: F) where F: FnOnce(&mut Self);

    // Reference:
    fn write_ref<T>(&mut self, p: *const T) -> bool;
    fn set_ref<T>(&mut self, p: *const T);
}

pub trait Encodable {
    fn encode<W: Encoder>(&self, w: &mut W);
}

impl Encodable for () {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_nil()
    }
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

impl<'a, T: ?Sized + Encodable> Encodable for &'a T {
    fn encode<W: Encoder>(&self, w: &mut W) {
        (**self).encode(w)
    }
}

impl<T: ?Sized + Encodable> Encodable for Box<T> {
    fn encode<W: Encoder>(&self, w: &mut W) {
        (**self).encode(w)
    }
}

impl<T: Encodable> Encodable for Rc<T> {
    #[inline]
    fn encode<W: Encoder>(&self, w: &mut W) {
        (**self).encode(w)
    }
}

impl<T: Encodable> Encodable for Arc<T> {
    #[inline]
    fn encode<W: Encoder>(&self, w: &mut W) {
        (**self).encode(w)
    }
}

impl<'a, T: Encodable + ToOwned + ?Sized> Encodable for Cow<'a, T> {
    #[inline]
    fn encode<W: Encoder>(&self, w: &mut W) {
        (**self).encode(w)
    }
}

impl<T: Encodable + Copy> Encodable for Cell<T> {
    fn encode<W: Encoder>(&self, w: &mut W) {
        self.get().encode(w)
    }
}

// Should use `try_borrow`, returning a
// `encoder.error("attempting to Encode borrowed RefCell")`
// from `encode` when `try_borrow` returns `None`.
impl<T: Encodable> Encodable for RefCell<T> {
    fn encode<W: Encoder>(&self, w: &mut W) {
        self.borrow().encode(w)
    }
}

macro_rules! array {
    () => ();
    ($($size:expr), +) => (
        $(impl<T: Encodable> Encodable for [T;($size)] {
            fn encode<W: Encoder>(&self, w: &mut W) {
                w.set_ref(ptr::null::<&[T]>());
                w.write_seq(self.len(), |w| {
                    for e in self {
                        e.encode(w);
                    }
                });
            }
        })+
    )
}

array! { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32 }

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

impl<T: Encodable> Encodable for LinkedList<T> {
    fn encode<W: Encoder>(&self, w: &mut W) {
        if w.write_ref(self) {
            return
        }
        w.set_ref(self);
        w.write_seq(self.len(), |w| {
            for (i, e) in self.iter().enumerate() {
                e.encode(w);
            }
        })
    }
}

impl<T: Encodable> Encodable for VecDeque<T> {
    fn encode<W: Encoder>(&self, w: &mut W) {
        if w.write_ref(self) {
            return
        }
        w.set_ref(self);
        w.write_seq(self.len(), |w| {
            for (i, e) in self.iter().enumerate() {
                e.encode(w);
            }
        })
    }
}

impl<T: Encodable + Ord> Encodable for BTreeSet<T> {
    fn encode<W: Encoder>(&self, w: &mut W) {
        if w.write_ref(self) {
            return
        }
        w.set_ref(self);
        w.write_seq(self.len(), |w| {
            for e in self.iter() {
                e.encode(w);
            }
        })
    }
}

impl<T> Encodable for HashSet<T> where T: Encodable + Hash + Eq {
    fn encode<W: Encoder>(&self, w: &mut W) {
        if w.write_ref(self) {
            return
        }
        w.set_ref(self);
        w.write_seq(self.len(), |w| {
            for e in self.iter() {
                e.encode(w);
            }
        })
    }
}

impl<K: Encodable + Ord, V: Encodable> Encodable for BTreeMap<K, V> {
    fn encode<W: Encoder>(&self, w: &mut W) {
        if w.write_ref(self) {
            return
        }
        w.set_ref(self);
        w.write_map(self.len(), |e| {
            for (key, val) in self.iter() {
                key.encode(e);
                val.encode(e);
            }
        })
    }
}

impl<K, V, S> Encodable for HashMap<K, V, S>
where K: Encodable + Hash + Eq,
      V: Encodable,
      S: BuildHasher
{
    fn encode<W: Encoder>(&self, w: &mut W) {
        if w.write_ref(self) {
            return
        }
        w.set_ref(self);
        w.write_map(self.len(), |e| {
            for (key, val) in self {
                key.encode(e);
                val.encode(e);
            }
        })
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

impl<T> Encodable for PhantomData<T> {
    fn encode<W: Encoder>(&self, w: &mut W) {
        w.write_nil()
    }
}

impl Encodable for Hprose {
    fn encode<W: Encoder>(&self, w: &mut W) {
        match *self {
            Hprose::Nil => w.write_nil(),
            Hprose::Boolean(b) => w.write_bool(b),
            Hprose::I64(i) => w.write_i64(i),
            Hprose::F32(f) => w.write_f32(f),
            Hprose::F64(f) => w.write_f64(f),
            Hprose::String(ref s) => w.write_str(s),
            _ => ()
        }
    }
}
