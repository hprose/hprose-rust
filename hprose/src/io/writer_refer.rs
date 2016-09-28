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
 * io/writer_refer.rs                                     *
 *                                                        *
 * hprose writer reference struct for Rust                *
 *                                                        *
 * LastModified: Sep 28, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use std::collections::HashMap;

use super::ByteWriter;
use super::tags::*;
use super::util::get_uint_bytes;

pub struct WriterRefer {
    references: HashMap<isize, u32>,
    lastref: u32
}

impl WriterRefer {
    #[inline]
    pub fn new() -> WriterRefer {
        WriterRefer {
            references: HashMap::new(),
            lastref: 0
        }
    }

    pub fn set<T>(&mut self, p: *const T) {
        let i = p as isize;
        if i > 0 {
            self.references.insert(p as isize, self.lastref);
        }
        self.lastref += 1;
    }

    pub fn write<T>(&mut self, w: &mut ByteWriter, p: *const T) -> bool {
        let i = p as isize;
        self.references.get(&i).map_or(false, |n| {
            w.write_byte(TAG_REF);
            w.write(get_uint_bytes(&mut [0; 20], *n as u64));
            w.write_byte(TAG_SEMICOLON);
            true
        })
    }
}
