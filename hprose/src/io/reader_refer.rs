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
 * io/reader_refer.rs                                     *
 *                                                        *
 * hprose reader reference struct for Rust                *
 *                                                        *
 * LastModified: Sep 24, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

pub struct ReaderRefer<'a> {
    references: Vec<&'a [u8]>
}

impl<'a> ReaderRefer<'a> {
    #[inline]
    pub fn new() -> ReaderRefer<'a> {
        ReaderRefer {
            references: Vec::new()
        }
    }

    #[inline]
    pub fn set(&mut self, v: &'a [u8]) {
        self.references.push(v);
    }

    #[inline]
    pub fn read(&mut self, index: usize) -> &'a [u8] {
        self.references[index]
    }
}