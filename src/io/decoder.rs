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
 * LastModified: Sep 13, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

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
    fn read_str(&mut self) -> Result<String, Self::Error>;
    fn read_bytes(&mut self) -> Result<Vec<u8>, Self::Error>;

    // Compound types:


    // Specialized types:
    fn read_option<T, F>(&mut self, f: F) -> Result<T, Self::Error>
        where F: FnMut(&mut Self, bool) -> Result<T, Self::Error>;

    fn read_seq<T, F>(&mut self, f: F) -> Result<T, Self::Error>
        where F: FnOnce(&mut Self, usize) -> Result<T, Self::Error>;

    // Reference:
}

pub trait Decodable: Sized {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error>;
}

impl Decodable for bool {
    fn decode<D: Decoder>(d: &mut D) -> Result<bool, D::Error> {
        d.read_bool()
    }
}

impl Decodable for i64 {
    fn decode<D: Decoder>(d: &mut D) -> Result<i64, D::Error> {
        d.read_i64()
    }
}

impl Decodable for u64 {
    fn decode<D: Decoder>(d: &mut D) -> Result<u64, D::Error> {
        d.read_u64()
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
        d.read_str()
    }
}

impl<T:Decodable> Decodable for Option<T> {
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
