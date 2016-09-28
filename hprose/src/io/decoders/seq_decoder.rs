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
 * io/decoders/seq_decoder.rs                             *
 *                                                        *
 * hprose seq decoder for Rust.                           *
 *                                                        *
 * LastModified: Sep 28, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io::{Reader, Decoder, Decodable, DecoderError};
use io::tags::*;
use io::reader::cast_error;

use std::result;

type Result<T> = result::Result<T, DecoderError>;

pub fn seq_decode<'a, T, F>(r: &mut Reader<'a>, tag: u8, f: F) -> Result<T>
    where T: Decodable, F: FnOnce(&mut Reader<'a>, usize) -> Result<T>
{
    match tag {
        TAG_LIST => read_list_as_seq(r, f),
        TAG_REF => r.read_ref(),
        _ => Err(cast_error(tag, "seq"))
    }
}

fn read_list_as_seq<'a, T, F>(r: &mut Reader<'a>, f: F) -> Result<T>
    where F: FnOnce(&mut Reader<'a>, usize) -> Result<T>
{
    let start = r.byte_reader.off - 1;
    let len = try!(r.read_count());
    let seq = try!(f(r, len));
    let reference = &r.byte_reader.buf[start..r.byte_reader.off];
    r.refer.as_mut().map(|r| r.set(reference));
    Ok(seq)
}
