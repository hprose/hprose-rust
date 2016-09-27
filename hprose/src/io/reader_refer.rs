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
 * LastModified: Sep 27, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

pub struct ReaderRefer<'a> {
    refs: Vec<&'a [u8]>
}

impl<'a> ReaderRefer<'a> {
    #[inline]
    pub fn new() -> ReaderRefer<'a> {
        ReaderRefer {
            refs: Vec::new()
        }
    }

    #[inline]
    pub fn set(&mut self, v: &'a [u8]) {
        self.refs.push(v);
    }

    #[inline]
    pub fn read(&mut self, index: usize) -> &'a [u8] {
        self.refs[index]
    }

    #[inline]
    pub fn reset(&mut self) {
        self.refs.clear();
    }
}
