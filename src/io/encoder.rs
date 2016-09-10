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

impl Encoder for i64 {
    fn encode(&self, w: &mut Writer) {
        w.write_int(*self);
    }
}
