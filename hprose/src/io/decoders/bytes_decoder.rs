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
 * io/decoders/bytes_decoder.rs                           *
 *                                                        *
 * hprose bytes decoder for Rust.                         *
 *                                                        *
 * LastModified: Sep 28, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io::{Reader, Decoder, Decodable, DecoderError, Bytes};
use io::tags::*;
use io::reader::cast_error;

use std::result;

type Result<T> = result::Result<T, DecoderError>;

pub fn bytes_decode(r: &mut Reader, tag: u8) -> Result<Bytes> {
    match tag {
        TAG_BYTES => read_bytes(r),
        TAG_LIST => read_list_as_bytes(r),
        TAG_REF => r.read_ref(),
        _ => Err(cast_error(tag, "bytes"))
    }
}

fn read_bytes(r: &mut Reader) -> Result<Bytes> {
    let start = r.byte_reader.off - 1;
    let len = try!(r.byte_reader.read_len());
    let bytes = try!(r.byte_reader.next(len)).to_owned();
    let reference = &r.byte_reader.buf[start..r.byte_reader.off];
    r.refer.as_mut().map(|r| r.set(reference));
    Ok(bytes)
}

fn read_list_as_bytes(r: &mut Reader) -> Result<Bytes> {
    let start = r.byte_reader.off - 1;
    let len = try!(r.byte_reader.read_len());
    let mut bytes = Vec::with_capacity(len);
    for _ in 0..len {
        bytes.push(try!(Decodable::decode(r)));
    }
    let reference = &r.byte_reader.buf[start..r.byte_reader.off];
    r.refer.as_mut().map(|r| r.set(reference));
    Ok(bytes)
}
