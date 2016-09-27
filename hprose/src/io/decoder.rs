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
 * io/decoder.rs                                          *
 *                                                        *
 * hprose decoder for Rust.                               *
 *                                                        *
 * LastModified: Sep 27, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use std::rc::Rc;
use std::sync::Arc;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::cell::{Cell, RefCell};
use std::hash::{Hash, BuildHasher};
use std::collections::HashMap;

use time::Tm;

pub trait Decoder {
    type Error;

    // Primitive types:
    fn read_nil(&mut self) -> Result<(), Self::Error>;
    fn read_bool(&mut self) -> Result<bool, Self::Error>;
    fn read_i64(&mut self) -> Result<i64, Self::Error>;
    fn read_u64(&mut self) -> Result<u64, Self::Error>;
    fn read_f32(&mut self) -> Result<f32, Self::Error>;
    fn read_f64(&mut self) -> Result<f64, Self::Error>;
    fn read_char(&mut self) -> Result<char, Self::Error>;
    fn read_string(&mut self) -> Result<String, Self::Error>;
    fn read_bytes(&mut self) -> Result<Vec<u8>, Self::Error>;

    // Extern crate types:
    fn read_datetime_without_tag(&mut self) -> Result<Tm, Self::Error>;
    fn read_time_without_tag(&mut self) -> Result<Tm, Self::Error>;

    // Compound types:


    // Specialized types:
    fn read_option<T, F>(&mut self, f: F) -> Result<T, Self::Error>
        where F: FnMut(&mut Self, bool) -> Result<T, Self::Error>;

    fn read_seq<T, F>(&mut self, f: F) -> Result<T, Self::Error>
        where F: FnOnce(&mut Self, usize) -> Result<T, Self::Error>;

    fn read_map<T, F>(&mut self, f: F) -> Result<T, Self::Error>
        where T: Decodable, F: FnOnce(&mut Self, usize) -> Result<T, Self::Error>;

    // Reference:
    fn read_ref<T: Decodable>(&mut self) -> Result<T, Self::Error>;
}

pub trait Decodable: Sized {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error>;
}

impl Decodable for () {
    fn decode<D: Decoder>(d: &mut D) -> Result<(), D::Error> {
        d.read_nil()
    }
}

impl Decodable for bool {
    fn decode<D: Decoder>(d: &mut D) -> Result<bool, D::Error> {
        d.read_bool()
    }
}

impl Decodable for i8 {
    fn decode<D: Decoder>(d: &mut D) -> Result<i8, D::Error> {
        d.read_i64().map(|i| i as i8)
    }
}

impl Decodable for i16 {
    fn decode<D: Decoder>(d: &mut D) -> Result<i16, D::Error> {
        d.read_i64().map(|i| i as i16)
    }
}

impl Decodable for i32 {
    fn decode<D: Decoder>(d: &mut D) -> Result<i32, D::Error> {
        d.read_i64().map(|i| i as i32)
    }
}

impl Decodable for i64 {
    fn decode<D: Decoder>(d: &mut D) -> Result<i64, D::Error> {
        d.read_i64()
    }
}

impl Decodable for isize {
    fn decode<D: Decoder>(d: &mut D) -> Result<isize, D::Error> {
        d.read_i64().map(|i| i as isize)
    }
}

impl Decodable for u8 {
    fn decode<D: Decoder>(d: &mut D) -> Result<u8, D::Error> {
        d.read_u64().map(|u| u as u8)
    }
}

impl Decodable for u16 {
    fn decode<D: Decoder>(d: &mut D) -> Result<u16, D::Error> {
        d.read_u64().map(|u| u as u16)
    }
}

impl Decodable for u32 {
    fn decode<D: Decoder>(d: &mut D) -> Result<u32, D::Error> {
        d.read_u64().map(|u| u as u32)
    }
}

impl Decodable for u64 {
    fn decode<D: Decoder>(d: &mut D) -> Result<u64, D::Error> {
        d.read_u64()
    }
}

impl Decodable for usize {
    fn decode<D: Decoder>(d: &mut D) -> Result<usize, D::Error> {
        d.read_u64().map(|u| u as usize)
    }
}

impl Decodable for f32 {
    fn decode<D: Decoder>(d: &mut D) -> Result<f32, D::Error> {
        d.read_f32()
    }
}

impl Decodable for f64 {
    fn decode<D: Decoder>(d: &mut D) -> Result<f64, D::Error> {
        d.read_f64()
    }
}

impl Decodable for char {
    fn decode<D: Decoder>(d: &mut D) -> Result<char, D::Error> {
        d.read_char()
    }
}

impl Decodable for String {
    fn decode<D: Decoder>(d: &mut D) -> Result<String, D::Error> {
        d.read_string()
    }
}

impl<T: Decodable> Decodable for Box<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Box<T>, D::Error> {
        Ok(Box::new(try!(Decodable::decode(d))))
    }
}

impl<T: Decodable> Decodable for Rc<T> {
    #[inline]
    fn decode<D: Decoder>(d: &mut D) -> Result<Rc<T>, D::Error> {
        Ok(Rc::new(try!(Decodable::decode(d))))
    }
}

impl<T: Decodable + Send + Sync> Decodable for Arc<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Arc<T>, D::Error> {
        Ok(Arc::new(try!(Decodable::decode(d))))
    }
}

impl<'a, T: ?Sized> Decodable for Cow<'a, T>
where T: ToOwned, T::Owned: Decodable
{
    #[inline]
    fn decode<D: Decoder>(d: &mut D) -> Result<Cow<'static, T>, D::Error> {
        Ok(Cow::Owned(try!(Decodable::decode(d))))
    }
}

impl<T: Decodable + Copy> Decodable for Cell<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Cell<T>, D::Error> {
        Ok(Cell::new(try!(Decodable::decode(d))))
    }
}

impl<T: Decodable> Decodable for RefCell<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<RefCell<T>, D::Error> {
        Ok(RefCell::new(try!(Decodable::decode(d))))
    }
}

impl<K, V, S> Decodable for HashMap<K, V, S>
where K: Decodable + Hash + Eq,
      V: Decodable,
      S: BuildHasher + Default
{
    fn decode<D: Decoder>(d: &mut D) -> Result<HashMap<K, V, S>, D::Error> {
        d.read_map(|d, len| {
            let state = Default::default();
            let mut map = HashMap::with_capacity_and_hasher(len, state);
            for _ in 0..len {
                let key = Decodable::decode(d)?;
                let val = Decodable::decode(d)?;
                map.insert(key, val);
            }
            Ok(map)
        })
    }
}

impl<T: Decodable> Decodable for Option<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Option<T>, D::Error> {
        d.read_option(|d, b| {
            if b {
                Ok(Some(Decodable::decode(d)?))
            } else {
                Ok(None)
            }
        })
    }
}

impl<T> Decodable for PhantomData<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<PhantomData<T>, D::Error> {
        d.read_nil().and(Ok(PhantomData))
    }
}
