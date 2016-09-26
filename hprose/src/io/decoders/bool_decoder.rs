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
 * io/decoders/bool_decoder.rs                            *
 *                                                        *
 * hprose bool decoder for Rust.                          *
 *                                                        *
 * LastModified: Sep 25, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io::{Reader, Decoder, DecoderError, ParserError};
use io::tags::*;
use io::reader::cast_error;
use io::util::utf8_slice_to_str;

use std::result;

type Result = result::Result<bool, DecoderError>;

pub fn bool_decode(r: &mut Reader, tag: u8) -> Result {
    match tag {
        b'0' | TAG_NULL | TAG_EMPTY | TAG_FALSE => Ok(false),
        b'1' ... b'9' | TAG_TRUE | TAG_NAN => Ok(true),
        TAG_INTEGER | TAG_LONG | TAG_DOUBLE => read_number_as_bool(r),
        TAG_INFINITY => read_inf_as_bool(r),
        TAG_UTF8_CHAR => read_utf8_char_as_bool(r),
        TAG_STRING => read_string_as_bool(r),
        TAG_REF => r.read_ref(),
        _ => Err(cast_error(tag, "bool"))
    }
}

fn read_number_as_bool(r: &mut Reader) -> Result {
    r.byte_reader
        .read_until(TAG_SEMICOLON)
        .map(|bytes| if bytes.len() == 1 { bytes[0] != b'0' } else { true })
        .map_err(|e| DecoderError::ParserError(e))
}

fn read_inf_as_bool(r: &mut Reader) -> Result {
    r.byte_reader
        .read_inf_64()
        .map(|_| true)
        .map_err(|e| DecoderError::ParserError(e))
}

fn read_utf8_char_as_bool(r: &mut Reader) -> Result {
    r.byte_reader
        .read_utf8_slice(1)
        .and_then(|s| parse_bool(utf8_slice_to_str(s)))
        .map_err(|e| DecoderError::ParserError(e))
}

fn read_string_as_bool(r: &mut Reader) -> Result {
    r
        .read_string_without_tag()
        .and_then(|s| parse_bool(&s).map_err(|e| DecoderError::ParserError(e)))
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
