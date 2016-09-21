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
 * LastModified: Sep 21, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/


use super::*;

/// Serialize data
pub fn serialize<T: Encodable>(v: &T, simple: bool) -> Vec<u8> {
    Writer::new(simple).serialize(v).bytes()
}

/// Unserialize data
pub fn unserialize<T: Decodable>(buf: & Vec<u8>) -> DecodeResult<T> {
    Reader::new(buf).read()
}
