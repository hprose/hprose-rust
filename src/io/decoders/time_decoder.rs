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
 * io/decoders/time_decoder.rs                          *
 *                                                        *
 * hprose time decoder for Rust.                          *
 *                                                        *
 * LastModified: Oct 8, 2016                              *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io::{Reader, Decoder, DecoderError, ParserError};
use io::tags::*;
use io::reader::cast_error;

use std::result;

use time::*;

type Result = result::Result<Tm, DecoderError>;

pub fn time_decode(r: &mut Reader, tag: u8) -> Result {
    match tag {
        b @ b'0' ... b'9' => Ok(at_utc(Timespec::new((b - b'0') as i64, 0))),
        TAG_INTEGER | TAG_LONG => read_long_as_time(r),
        TAG_DOUBLE => read_float_as_time(r),
        TAG_STRING => read_string_as_time(r),
        TAG_DATE => r.read_datetime_without_tag(),
        TAG_TIME => r.read_time_without_tag(),
        TAG_REF => r.read_ref(),
        _ => Err(cast_error(tag, "string"))
    }
}

fn read_long_as_time(r: &mut Reader) -> Result {
    r.byte_reader.read_i64()
        .map(|i| at_utc(Timespec::new(i, 0)))
        .map_err(|e| DecoderError::ParserError(e))
}

fn read_float_as_time(r: &mut Reader) -> Result {
    r.byte_reader.read_f64()
        .map(|f| at_utc(Timespec::new(f as i64, 0)))
        .map_err(|e| DecoderError::ParserError(e))
}

fn read_string_as_time(r: &mut Reader) -> Result {
    r.read_string_without_tag()
        .and_then(|s| strptime(&s, "%F %T.%f %z").map_err(|e| DecoderError::ParserError(ParserError::ParseTimeError(e))))
}
