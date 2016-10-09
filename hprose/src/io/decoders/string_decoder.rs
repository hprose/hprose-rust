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
 * LastModified: Oct 9, 2016                              *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io::{Reader, Decoder, DecoderError, ParserError};
use io::tags::*;
use io::reader::cast_error;
use io::util::utf8_slice_to_str;

use std::result;

type Result = result::Result<String, DecoderError>;

pub fn string_decode(r: &mut Reader, tag: u8) -> Result {
    match tag {
        b'0' => Ok("0".to_owned()),
        b'1' => Ok("1".to_owned()),
        b'2' => Ok("2".to_owned()),
        b'3' => Ok("3".to_owned()),
        b'4' => Ok("4".to_owned()),
        b'5' => Ok("5".to_owned()),
        b'6' => Ok("6".to_owned()),
        b'7' => Ok("7".to_owned()),
        b'8' => Ok("8".to_owned()),
        b'9' => Ok("9".to_owned()),
        TAG_NULL | TAG_EMPTY => Ok(String::new()),
        TAG_FALSE => Ok("false".to_owned()),
        TAG_TRUE => Ok("true".to_owned()),
        TAG_NAN => Ok("NaN".to_owned()),
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
    r.read_byte()
        .map(|sign| if sign == TAG_POS { "+Inf".to_owned() } else { "-Inf".to_owned() })
}

fn read_number_as_string(r: &mut Reader) -> Result {
    r.byte_reader.read_until(TAG_SEMICOLON)
        .map(|bytes| utf8_slice_to_str(bytes).to_owned())
        .map_err(|e| DecoderError::ParserError(e))
}

fn read_utf8_char_as_string(r: &mut Reader) -> Result {
    r.byte_reader
        .read_utf8_slice(1)
        .map(|bytes| utf8_slice_to_str(bytes).to_owned())
        .map_err(|e| DecoderError::ParserError(e))
}

fn read_bytes_as_string(r: &mut Reader) -> Result {
    let start = r.byte_reader.off - 1;
    let len = {
        try!(r.byte_reader.read_len())
    };
    let bytes = try!(r.byte_reader.next(len)).to_owned();
    let s = try!(String::from_utf8(bytes).map_err(|_| ParserError::BadUTF8Encode));
    try!(r.read_byte());
    let reference = &r.byte_reader.buf[start..r.byte_reader.off];
    r.refer.as_mut().map(|mut r| r.set(reference));
    Ok(s)
}

fn read_guid_as_string(r: &mut Reader) -> Result {
    let start = r.byte_reader.off - 1;
    try!(r.read_byte());
    let bytes = try!(r.byte_reader.next(36)).to_owned();
    let s = try!(String::from_utf8(bytes).map_err(|_| ParserError::BadUTF8Encode));
    try!(r.read_byte());
    let reference = &r.byte_reader.buf[start..r.byte_reader.off];
    r.refer.as_mut().map(|mut r| r.set(reference));
    Ok(s)
}

fn read_datetime_as_string(r: &mut Reader) -> Result {
    r.read_datetime_without_tag().map(|ref tm| tm.strftime("%F %T.%f %z").unwrap().to_string())
}

fn read_time_as_string(r: &mut Reader) -> Result {
    r.read_time_without_tag().map(|ref tm| tm.strftime("%F %T.%f %z").unwrap().to_string())
}
