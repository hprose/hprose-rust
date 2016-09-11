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
 * LastModified: Sep 11, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/


use super::encoder::Encoder;
use super::writer::Writer;

pub fn serialize<T: Encoder>(v: &T, simple: bool) -> Vec<u8> {
    Writer::new(simple).serialize(v).bytes()
}
