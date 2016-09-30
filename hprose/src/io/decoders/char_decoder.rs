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
 * io/decoders/char_decoder.rs                          *
 *                                                        *
 * hprose char decoder for Rust.                          *
 *                                                        *
 * LastModified: Sep 30, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io::{Reader, Decoder, DecoderError, ParserError};
use io::tags::*;
use io::reader::cast_error;
use io::util::utf8_slice_to_str;

use std::result;

type Result = result::Result<char, DecoderError>;

pub fn char_decode(r: &mut Reader, tag: u8) -> Result {
    match tag {
        b'0' => Ok('0'),
        b'1' => Ok('1'),
        b'2' => Ok('2'),
        b'3' => Ok('3'),
        b'4' => Ok('4'),
        b'5' => Ok('5'),
        b'6' => Ok('6'),
        b'7' => Ok('7'),
        b'8' => Ok('8'),
        b'9' => Ok('9'),
        TAG_UTF8_CHAR => read_utf8_char_as_char(r),
        TAG_STRING => read_string_as_char(r),
        TAG_REF => r.read_ref(),
        _ => Err(cast_error(tag, "char"))
    }
}

fn read_utf8_char_as_char(r: &mut Reader) -> Result {
    r.byte_reader
        .read_utf8_slice(1)
        .map(|bytes| utf8_slice_to_str(bytes).chars().next().unwrap())
        .map_err(|e| DecoderError::ParserError(e))
}

fn read_string_as_char(r: &mut Reader) -> Result {
    r.read_string_without_tag().map(|s| s.chars().next().unwrap())
}
