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
 * io/tags.rs                                             *
 *                                                        *
 * hprose tags enum for Rust.                             *
 *                                                        *
 * LastModified: Sep 11, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

// Hprose Tags

// Serialize Type;
pub const TAG_INTEGER   :u8 = 'i' as u8;
pub const TAG_LONG      :u8 = 'l' as u8;
pub const TAG_DOUBLE    :u8 = 'd' as u8;
pub const TAG_NULL      :u8 = 'n' as u8;
pub const TAG_EMPTY     :u8 = 'e' as u8;
pub const TAG_TRUE      :u8 = 't' as u8;
pub const TAG_FALSE     :u8 = 'f' as u8;
pub const TAG_NAN       :u8 = 'N' as u8;
pub const TAG_INFINITY  :u8 = 'I' as u8;
pub const TAG_DATE      :u8 = 'D' as u8;
pub const TAG_TIME      :u8 = 'T' as u8;
pub const TAG_UTC       :u8 = 'Z' as u8;
pub const TAG_BYTES     :u8 = 'b' as u8;
pub const TAG_UTF8_CHAR :u8 = 'u' as u8;
pub const TAG_STRING    :u8 = 's' as u8;
pub const TAG_GUID      :u8 = 'g' as u8;
pub const TAG_LIST      :u8 = 'a' as u8;
pub const TAG_MAP       :u8 = 'm' as u8;
pub const TAG_CLASS     :u8 = 'c' as u8;
pub const TAG_OBJECT    :u8 = 'o' as u8;
pub const TAG_REF       :u8 = 'r' as u8;

// Serialize Marks;
pub const TAG_POS       :u8 = '+' as u8;
pub const TAG_NEG       :u8 = '-' as u8;
pub const TAG_SEMICOLON :u8 = ';' as u8;
pub const TAG_OPENBRACE :u8 = '{' as u8;
pub const TAG_CLOSEBRACE:u8 = '}' as u8;
pub const TAG_QUOTE     :u8 = '"' as u8;
pub const TAG_POINT     :u8 = '.' as u8;

// Protocol Tags;
pub const TAG_FUNCTIONS:u8 = 'F' as u8;
pub const TAG_CALL     :u8 = 'C' as u8;
pub const TAG_RESULT   :u8 = 'R' as u8;
pub const TAG_ARGUMENT :u8 = 'A' as u8;
pub const TAG_ERROR    :u8 = 'E' as u8;
pub const TAG_END      :u8 = 'z' as u8;
