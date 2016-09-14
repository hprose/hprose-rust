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
 * io/u64_decoder.rs                                      *
 *                                                        *
 * hprose u64 decoder for Rust.                           *
 *                                                        *
 * LastModified: Sep 14, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::*;
use super::tags::*;
use super::reader::tagToStr;

use std::result;

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
        TAG_REF => read_ref_as_u64(r),
        _ => tagToStr(tag).and_then(|srcType| Err(DecoderError::CastError(srcType, "u64")))
    }
}

fn read_u64(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_f64_as_u64(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_utf8_char_as_u64(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_string_as_u64(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_datetime_as_u64(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_time_as_u64(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_ref_as_u64(r: &mut Reader) -> Result {
    unimplemented!()
}