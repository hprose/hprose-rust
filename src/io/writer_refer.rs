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
 * LastModified: Sep 12, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use std::collections::HashMap;
use std::io::Write;

use super::tags::*;

pub struct WriterRefer {
    references: HashMap<isize, i32>,
    lastref: i32
}

impl WriterRefer {
    pub fn set<T>(&mut self, p: *const T) {
        let i = p as isize;
        if i > 0 {
            self.references.insert(p as isize, self.lastref);
        }
        self.lastref += 1;
    }

    pub fn write<T>(&mut self, v: &mut Vec<u8>, p: *const T) -> bool {
        let i = p as isize;
        self.references.get(&i).map_or(false, |n| {
            v.push(TAG_REF);
            write!(v, "{}", n).unwrap();
            v.push(TAG_SEMICOLON);
            true
        })
    }

    #[inline]
    pub fn new() -> WriterRefer {
        WriterRefer {
            references: HashMap::new(),
            lastref: 0
        }
    }
}
