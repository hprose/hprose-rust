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
 * io/map_decoder.rs                                      *
 *                                                        *
 * hprose map decoder for Rust.                           *
 *                                                        *
 * LastModified: Sep 21, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::*;
use super::tags::*;
use super::byte_reader::ParserError;
use super::reader::cast_error;

use std::{result, str};
use std::collections::HashMap;

type Result<T> = result::Result<T, DecoderError>;

pub fn map_decode<'a, T, F>(r: &mut Reader<'a>, tag: u8, f: F) -> Result<T>
    where F: FnOnce(&mut Reader<'a>, usize) -> DecodeResult<T>
{
    match tag {
        //        TAG_NULL | TAG_EMPTY => Ok(),
        TAG_LIST => read_list_as_map(r),
        TAG_MAP => read_map(r, f),
        TAG_CLASS => read_struct_meta(r),
        TAG_OBJECT => read_struct_as_map(r),
        TAG_REF => read_ref_as_map(r),
        _ => Err(cast_error(tag, "map"))
    }
}

fn read_list_as_map<T>(r: &mut Reader) -> Result<T> {
    unimplemented!()
}

fn read_map<'a, T, F>(r: &mut Reader<'a>, f: F) -> Result<T>
    where F: FnOnce(&mut Reader<'a>, usize) -> DecodeResult<T>
{
    r.reader.read_count()
        .map_err(|e| DecoderError::ParserError(e))
        .and_then(|len| f(r, len))
}

fn read_struct_meta<T>(r: &mut Reader) -> Result<T> {
    unimplemented!()
}

fn read_struct_as_map<T>(r: &mut Reader) -> Result<T> {
    unimplemented!()
}

fn read_ref_as_map<T>(r: &mut Reader) -> Result<T> {
    unimplemented!()
}
