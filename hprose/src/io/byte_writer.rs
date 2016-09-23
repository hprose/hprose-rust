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
 * io/byte_writer.rs                                      *
 *                                                        *
 * byte writer for Rust.                                  *
 *                                                        *
 * LastModified: Sep 22, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::Bytes;

use std::ptr;
use std::string::FromUtf8Error;

pub struct ByteWriter {
    pub buf: Bytes
}

impl ByteWriter {
    /// Constructs a new `ByteWriter`.
    #[inline]
    pub fn new() -> ByteWriter {
        ByteWriter {
            buf: Vec::with_capacity(1024)
        }
    }

    #[inline]
    pub fn bytes(&mut self) -> Bytes {
        self.buf.clone()
    }

    #[inline]
    pub fn string(&mut self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.bytes())
    }

    /// Clears the buf, Removing all bytes.
    #[inline]
    pub fn clear(&mut self) {
        self.buf.clear();
    }

    /// Returns the number of bytes in the buf.
    #[inline]
    pub fn len(&mut self) -> usize {
        self.buf.len()
    }

    /// Writes a byte to the end of the buf.
    #[inline]
    pub fn write_byte(&mut self, b: u8) {
        self.buf.push(b);
    }

    /// Writes the contents in byte slice to the buf.
    pub fn write(&mut self, src: &[u8]) {
        let dst_len = self.len();
        let src_len = src.len();

        self.buf.reserve(src_len);

        unsafe {
            // We would have failed if `reserve` overflowed
            self.buf.set_len(dst_len + src_len);

            ptr::copy_nonoverlapping(
                src.as_ptr(),
                self.buf.as_mut_ptr().offset(dst_len as isize),
                src_len);
        }
    }
}
