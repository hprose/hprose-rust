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
 * LastModified: Oct 8, 2016                              *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::Bytes;

use std::ptr;
use std::string::FromUtf8Error;

pub struct ByteWriter {
    pub vec: Bytes
}

impl ByteWriter {
    /// Constructs a new `ByteWriter`.
    #[inline]
    pub fn new() -> ByteWriter {
        ByteWriter {
            vec: Vec::new()
        }
    }

    /// Converts a `ByteWriter` into a byte vector.
    ///
    /// This consumes the `ByteWriter`, so we do not need to copy its contents.
    #[inline]
    pub fn into_bytes(self) -> Bytes {
        self.vec
    }

    /// Returns a byte slice of this `ByteWriter`'s contents.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.vec
    }

    /// Returns the `String` of this writer.
    #[inline]
    pub fn as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.vec.clone())
    }

    /// Clears the vector, Removing all bytes.
    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear();
    }

    /// Returns the number of bytes in the vector.
    #[inline]
    pub fn len(&mut self) -> usize {
        self.vec.len()
    }

    /// Writes a byte to the end of the vector.
    /// # Panics
    ///
    /// Panics if the number of elements in the vector overflows a `usize`.
    ///
    #[inline]
    pub fn write_byte(&mut self, b: u8) {
        self.vec.push(b);
    }

    /// Writes all the bytes to the end of the vector.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the vector overflows a `usize`.
    ///
    pub fn write(&mut self, bytes: &[u8]) {
        let slen = self.vec.len();
        let blen = bytes.len();

        self.vec.reserve(blen);

        unsafe {
            ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                self.vec.get_unchecked_mut(slen),
                blen);

            self.vec.set_len(slen + blen);
        }
    }
}
