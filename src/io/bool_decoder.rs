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
 * LastModified: Sep 14, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::*;
use super::tags::*;
use super::reader::tagToStr;

pub fn bool_decoder(r: &mut Reader, tag: u8) -> Result<bool, DecoderError> {
    match tag {
        b'0' | TAG_NULL | TAG_EMPTY | TAG_FALSE => Ok(false),
        b'1' ... b'9' | TAG_TRUE | TAG_NAN => Ok(true),
        TAG_INTEGER | TAG_LONG | TAG_DOUBLE => read_number_as_bool(r),
        TAG_INFINITY => read_infinity_as_bool(r),
        TAG_UTF8_CHAR => read_utf8_char_as_bool(r),
        TAG_STRING => read_string_as_bool(r),
        TAG_REF => read_ref_as_bool(r),
        _ => tagToStr(tag).and_then(|srcType| Err(DecoderError::CastError(srcType, "bool")))
    }
}

fn read_number_as_bool(r: &mut Reader) -> Result<bool, DecoderError> {
    r
        .read_until(TAG_SEMICOLON)
        .map_err(|e| DecoderError::ParserError(e))
        .and_then(|bytes| Ok(if bytes.len() == 1 { bytes[0] != b'0' } else { true }))
}

fn read_infinity_as_bool(r: &mut Reader) -> Result<bool, DecoderError> {
    r.read_inf().map_err(|e| DecoderError::ParserError(e)).and_then(|inf| Ok(true))
}

fn read_utf8_char_as_bool(r: &mut Reader) -> Result<bool, DecoderError> {
    unimplemented!()
}

fn read_string_as_bool(r: &mut Reader) -> Result<bool, DecoderError> {
    unimplemented!()
}

fn read_ref_as_bool(r: &mut Reader) -> Result<bool, DecoderError> {
    unimplemented!()
}
