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
 * io/decoders/f64_decoder.rs                             *
 *                                                        *
 * hprose f64 decoder for Rust.                           *
 *                                                        *
 * LastModified: Sep 25, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io::{Reader, Decoder, DecoderError, ParserError};
use io::tags::*;
use io::reader::cast_error;

use std::{result, str, f64};

type Result = result::Result<f64, DecoderError>;

pub fn f64_decode(r: &mut Reader, tag: u8) -> Result {
    match tag {
        b'0' | TAG_NULL | TAG_EMPTY | TAG_FALSE => Ok(0f64),
        b'1' | TAG_TRUE => Ok(1f64),
        b'2' => Ok(2f64),
        b'3' => Ok(3f64),
        b'4' => Ok(4f64),
        b'5' => Ok(5f64),
        b'6' => Ok(6f64),
        b'7' => Ok(7f64),
        b'8' => Ok(8f64),
        b'9' => Ok(9f64),
        TAG_NAN => Ok(f64::NAN),
        TAG_INFINITY => read_inf_as_f64(r),
        TAG_INTEGER | TAG_LONG => read_long_as_f64(r),
        TAG_DOUBLE => r.read_f64(),
        TAG_UTF8_CHAR => read_utf8_char_as_f64(r),
        TAG_STRING => read_string_as_f64(r),
        TAG_DATE => read_datetime_as_f64(r),
        TAG_TIME => read_time_as_f64(r),
        TAG_REF => r.read_ref(),
        _ => Err(cast_error(tag, "f64"))
    }
}

fn read_inf_as_f64(r: &mut Reader) -> Result {
    r.byte_reader.read_inf_64().map_err(|e| DecoderError::ParserError(e))
}

fn read_long_as_f64(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_utf8_char_as_f64(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_string_as_f64(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_datetime_as_f64(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_time_as_f64(r: &mut Reader) -> Result {
    unimplemented!()
}
