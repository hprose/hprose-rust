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
 * io/decoders/map_decoder.rs                             *
 *                                                        *
 * hprose map decoder for Rust.                           *
 *                                                        *
 * LastModified: Sep 27, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io::{Reader, Decoder, Decodable,  DecoderError};
use io::tags::*;
use io::reader::cast_error;

use std::{result, str};

type Result<T> = result::Result<T, DecoderError>;

pub fn map_decode<'a, T, F>(r: &mut Reader<'a>, tag: u8, f: F) -> Result<T>
    where T: Decodable, F: FnOnce(&mut Reader<'a>, usize) -> Result<T>
{
    match tag {
        TAG_LIST => read_list_as_map(r),
        TAG_MAP => read_map(r, f),
        TAG_CLASS => read_struct_meta(r),
        TAG_OBJECT => read_struct_as_map(r),
        TAG_REF => r.read_ref(),
        _ => Err(cast_error(tag, "map"))
    }
}

fn read_list_as_map<T>(r: &mut Reader) -> Result<T> {
    unimplemented!()
}

fn read_map<'a, T, F>(r: &mut Reader<'a>, f: F) -> Result<T>
    where F: FnOnce(&mut Reader<'a>, usize) -> Result<T>
{
    let start = r.byte_reader.off - 1;
    let len = try!(r.read_count());
    let map = try!(f(r, len));
    let reference = &r.byte_reader.buf[start..r.byte_reader.off];
    r.refer.as_mut().map(|mut r| r.set(reference));
    Ok(map)
}

fn read_struct_meta<T>(r: &mut Reader) -> Result<T> {
    unimplemented!()
}

fn read_struct_as_map<T>(r: &mut Reader) -> Result<T> {
    unimplemented!()
}
