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
 * LastModified: Sep 30, 2016                             *
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
            buf: Vec::new()
        }
    }

    /// Returns the `Bytes` of this writer.
    #[inline]
    pub fn bytes(&mut self) -> Bytes {
        self.buf.clone()
    }

    /// Returns the `String` of this writer.
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
    /// # Panics
    ///
    /// Panics if the number of elements in the buf overflows a `usize`.
    ///
    #[inline]
    pub fn write_byte(&mut self, b: u8) {
        self.buf.push(b);
    }

    /// Writes all the bytes to the end of the buf.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the buf overflows a `usize`.
    ///
    pub fn write(&mut self, bytes: &[u8]) {
        let slen = self.buf.len();
        let blen = bytes.len();

        self.buf.reserve(blen);

        unsafe {
            ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                self.buf.get_unchecked_mut(slen),
                blen);

            self.buf.set_len(slen + blen);
        }
    }
}
