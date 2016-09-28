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
 * LastModified: Sep 28, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/


use super::*;

/// Serialize data
pub fn serialize<T: Encodable>(v: &T, simple: bool) -> Vec<u8> {
    Writer::new(simple).serialize(v).bytes()
}

/// Marshal data
#[inline]
pub fn marshal<T: Encodable>(v: &T) -> Vec<u8> {
    serialize(v, true)
}

/// Unserialize data
pub fn unserialize<T: Decodable>(buf: &Vec<u8>, simple: bool) -> DecodeResult<T> {
    Reader::new(buf, simple).read_value()
}

/// Unmarshal data
#[inline]
pub fn unmarshal<T: Decodable>(buf: &Vec<u8>) -> DecodeResult<T> {
    unserialize(buf, true)
}
