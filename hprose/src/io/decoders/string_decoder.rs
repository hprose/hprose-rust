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
 * io/decoders/string_decoder.rs                          *
 *                                                        *
 * hprose string decoder for Rust.                        *
 *                                                        *
 * LastModified: Sep 25, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io::{Reader, Decoder, DecoderError, ParserError};
use io::tags::*;
use io::reader::cast_error;

use std::{result, str};

type Result = result::Result<String, DecoderError>;

pub fn string_decode(r: &mut Reader, tag: u8) -> Result {
    match tag {
        b'0' => Ok(String::from("0")),
        b'1' => Ok(String::from("1")),
        b'2' => Ok(String::from("2")),
        b'3' => Ok(String::from("3")),
        b'4' => Ok(String::from("4")),
        b'5' => Ok(String::from("5")),
        b'6' => Ok(String::from("6")),
        b'7' => Ok(String::from("7")),
        b'8' => Ok(String::from("8")),
        b'9' => Ok(String::from("9")),
        TAG_NULL | TAG_EMPTY => Ok(String::new()),
        TAG_FALSE => Ok(String::from("false")),
        TAG_TRUE => Ok(String::from("true")),
        TAG_NAN => Ok(String::from("NaN")),
        TAG_INFINITY => read_inf_as_string(r),
        TAG_INTEGER | TAG_LONG | TAG_DOUBLE => read_number_as_string(r),
        TAG_UTF8_CHAR => read_utf8_char_as_string(r),
        TAG_STRING => r.read_string_without_tag(),
        TAG_BYTES => read_bytes_as_string(r),
        TAG_GUID => read_guid_as_string(r),
        TAG_DATE => read_datetime_as_string(r),
        TAG_TIME => read_time_as_string(r),
        TAG_REF => r.read_ref(),
        _ => Err(cast_error(tag, "string"))
    }
}

fn read_inf_as_string(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_number_as_string(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_utf8_char_as_string(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_bytes_as_string(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_guid_as_string(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_datetime_as_string(r: &mut Reader) -> Result {
    unimplemented!()
}

fn read_time_as_string(r: &mut Reader) -> Result {
    unimplemented!()
}
