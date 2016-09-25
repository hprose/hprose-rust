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
 * io/decoders/u64_decoder.rs                             *
 *                                                        *
 * hprose u64 decoder for Rust.                           *
 *                                                        *
 * LastModified: Sep 25, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io::{Reader, Decoder, DecoderError, ParserError};
use io::tags::*;
use io::reader::cast_error;
use io::util::utf8_slice_to_str;

use std::{result, str};

type Result = result::Result<u64, DecoderError>;

pub fn u64_decode(r: &mut Reader, tag: u8) -> Result {
    match tag {
        b'0' | TAG_NULL | TAG_EMPTY | TAG_FALSE => Ok(0),
        b'1' | TAG_TRUE => Ok(1),
        b'2' => Ok(2),
        b'3' => Ok(3),
        b'4' => Ok(4),
        b'5' => Ok(5),
        b'6' => Ok(6),
        b'7' => Ok(7),
        b'8' => Ok(8),
        b'9' => Ok(9),
        TAG_INTEGER | TAG_LONG => read_u64(r),
        TAG_DOUBLE => read_f64_as_u64(r),
        TAG_UTF8_CHAR => read_utf8_char_as_u64(r),
        TAG_STRING => read_string_as_u64(r),
        TAG_DATE => read_datetime_as_u64(r),
        TAG_TIME => read_time_as_u64(r),
        TAG_REF => r.read_ref(),
        _ => Err(cast_error(tag, "u64"))
    }
}

fn read_u64(r: &mut Reader) -> Result {
    r.byte_reader.read_u64_with_tag(TAG_SEMICOLON).map_err(|e| DecoderError::ParserError(e))
}

fn read_f64_as_u64(r: &mut Reader) -> Result {
    r.byte_reader.read_f64().map(|f| f as u64).map_err(|e| DecoderError::ParserError(e))
}

fn read_utf8_char_as_u64(r: &mut Reader) -> Result {
    r.byte_reader
        .read_utf8_slice(1)
        .and_then(|s| utf8_slice_to_str(s).parse::<u64>().map_err(|e| ParserError::ParseIntError(e)))
        .map_err(|e| DecoderError::ParserError(e))
}

fn read_string_as_u64(r: &mut Reader) -> Result {
    r
        .read_string_without_tag()
        .and_then(|s| s.parse::<u64>().map_err(|e| ParserError::ParseIntError(e)).map_err(|e| DecoderError::ParserError(e)))
}

fn read_datetime_as_u64(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_time_as_u64(r: &mut Reader) -> Result {
    unimplemented!()
}
