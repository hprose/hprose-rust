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
 * LastModified: Sep 19, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::*;
use super::tags::*;
use super::reader::ParserError;
use super::reader::cast_error;

use std::result;
use std::str;

type Result = result::Result<bool, DecoderError>;

pub fn bool_decode(r: &mut Reader, tag: u8) -> Result {
    match tag {
        b'0' | TAG_NULL | TAG_EMPTY | TAG_FALSE => Ok(false),
        b'1' ... b'9' | TAG_TRUE | TAG_NAN => Ok(true),
        TAG_INTEGER | TAG_LONG | TAG_DOUBLE => read_number_as_bool(r),
        TAG_INFINITY => read_inf_as_bool(r),
        TAG_UTF8_CHAR => read_utf8_char_as_bool(r),
        TAG_STRING => read_string_as_bool(r),
        TAG_REF => read_ref_as_bool(r),
        _ => Err(cast_error(tag, "bool"))
    }
}

fn read_number_as_bool(r: &mut Reader) -> Result {
    r.reader
        .read_until(TAG_SEMICOLON)
        .map(|bytes| if bytes.len() == 1 { bytes[0] != b'0' } else { true })
        .map_err(|e| DecoderError::ParserError(e))
}

fn read_inf_as_bool(r: &mut Reader) -> Result {
    r.reader
        .read_inf()
        .map(|_| true)
        .map_err(|e| DecoderError::ParserError(e))
}

fn read_utf8_char_as_bool(r: &mut Reader) -> Result {
    r.reader
        .read_u8_slice(1)
        .and_then(|s| parse_bool(unsafe { str::from_utf8_unchecked(s) }))
        .map_err(|e| DecoderError::ParserError(e))
}

fn read_string_as_bool(r: &mut Reader) -> Result {
    r
        .read_string_without_tag()
        .and_then(|s| parse_bool(&s).map_err(|e| DecoderError::ParserError(e)))
}

fn read_ref_as_bool(r: &mut Reader) -> Result {
    unimplemented!()
}

// parse_bool returns the boolean value represented by the string.
// It accepts 1, t, T, TRUE, true, True, 0, f, F, FALSE, false, False.
// Any other value returns an error.
fn parse_bool(s: &str) -> result::Result<bool, ParserError> {
    match s {
        "1" | "t" | "T" | "true" | "TRUE" | "True" => Ok(true),
        "0" | "f" | "F" | "false" | "FALSE" | "False" => Ok(false),
        _ => Err(ParserError::ParseBoolError)
    }
}
