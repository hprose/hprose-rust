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
 * io/f32_decoder.rs                                      *
 *                                                        *
 * hprose f32 decoder for Rust.                           *
 *                                                        *
 * LastModified: Sep 19, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::*;
use super::tags::*;
use super::util::utf8_slice_to_str;
use super::byte_reader::ParserError;
use super::reader::cast_error;

use std::{result, str, f32};

type Result = result::Result<f32, DecoderError>;

pub fn f32_decode(r: &mut Reader, tag: u8) -> Result {
    match tag {
        b'0' | TAG_NULL | TAG_EMPTY | TAG_FALSE => Ok(0f32),
        b'1' | TAG_TRUE => Ok(1f32),
        b'2' => Ok(2f32),
        b'3' => Ok(3f32),
        b'4' => Ok(4f32),
        b'5' => Ok(5f32),
        b'6' => Ok(6f32),
        b'7' => Ok(7f32),
        b'8' => Ok(8f32),
        b'9' => Ok(9f32),
        TAG_NAN => Ok(f32::NAN),
        TAG_INFINITY => read_inf_as_f32(r),
        TAG_INTEGER | TAG_LONG => read_long_as_f32(r),
        TAG_DOUBLE => r.read_f32(),
        TAG_UTF8_CHAR => read_utf8_char_as_f32(r),
        TAG_STRING => read_string_as_f32(r),
        TAG_DATE => read_datetime_as_f32(r),
        TAG_TIME => read_time_as_f32(r),
        TAG_REF => read_ref_as_f32(r),
        _ => Err(cast_error(tag, "f32"))
    }
}

fn read_inf_as_f32(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_long_as_f32(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_f32(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_utf8_char_as_f32(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_string_as_f32(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_datetime_as_f32(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_time_as_f32(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_ref_as_f32(r: &mut Reader) -> Result {
    unimplemented!()
}
