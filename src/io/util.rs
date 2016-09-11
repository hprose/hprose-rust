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
 * io/util.rs                                             *
 *                                                        *
 * io util for Rust.                                      *
 *                                                        *
 * LastModified: Sep 11, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

pub fn utf16_length(s: &str) -> i64 {
    let length = s.len();
    let bytes = s.as_bytes();
    let mut n = length as i64;
    let mut p = 0;
    while p < length {
        let a = bytes[p];
        match a >> 4 {
            0 ... 7 => p += 1,
            12 ... 13 => {
                p += 2;
                n -= 1
            },
            14 => {
                p += 3;
                n -= 2
            },
            15 => {
                if a & 8 == 8 {
                    return -1
                }
                p += 4;
                n -= 2
            }
            _ => return -1
        }
    }
    n
}
