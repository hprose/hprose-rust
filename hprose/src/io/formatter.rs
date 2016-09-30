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
 * io/formatter.rs                                        *
 *                                                        *
 * io Formatter for Rust.                                 *
 *                                                        *
 * LastModified: Sep 30, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/


use super::*;

/// Serialize data
pub fn serialize<T: Encodable>(v: &T, simple: bool) -> Bytes {
    Writer::new(simple).serialize(v).bytes()
}

/// Marshal data
#[inline]
pub fn marshal<T: Encodable>(v: &T) -> Bytes {
    serialize(v, true)
}

/// Unserialize data
pub fn unserialize<T: Decodable>(buf: &Bytes, simple: bool) -> DecodeResult<T> {
    Reader::new(buf, simple).read_value()
}

/// Unmarshal data
#[inline]
pub fn unmarshal<T: Decodable>(buf: &Bytes) -> DecodeResult<T> {
    unserialize(buf, true)
}
