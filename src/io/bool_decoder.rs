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
 * io/bool_decoder.rs                                     *
 *                                                        *
 * hprose bool decoder for Rust.                          *
 *                                                        *
 * LastModified: Sep 13, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::*;
use super::tags::*;

pub fn bool_decoder(r: &mut Reader, tag: u8) -> Result<bool, DecoderError> {
    match tag {
        b'0' | TAG_NULL | TAG_EMPTY | TAG_FALSE => Ok(read_bool_false(r)),
        b'1' ... b'9' | TAG_TRUE | TAG_NAN => Ok(read_bool_true(r)),
        _ => Err(DecoderError::CastError(tag, "bool".to_string()))
    }
}

fn read_bool_false(r: &mut Reader) -> bool {
    false
}

fn read_bool_true(r: &mut Reader) -> bool {
    true
}
