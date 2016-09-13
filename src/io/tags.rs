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
 * LastModified: Sep 13, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

// Hprose Tags

// Serialize Type;
pub const TAG_INTEGER   :u8 = b'i';
pub const TAG_LONG      :u8 = b'l';
pub const TAG_DOUBLE    :u8 = b'd';
pub const TAG_NULL      :u8 = b'n';
pub const TAG_EMPTY     :u8 = b'e';
pub const TAG_TRUE      :u8 = b't';
pub const TAG_FALSE     :u8 = b'f';
pub const TAG_NAN       :u8 = b'N';
pub const TAG_INFINITY  :u8 = b'I';
pub const TAG_DATE      :u8 = b'D';
pub const TAG_TIME      :u8 = b'T';
pub const TAG_UTC       :u8 = b'Z';
pub const TAG_BYTES     :u8 = b'b';
pub const TAG_UTF8_CHAR :u8 = b'u';
pub const TAG_STRING    :u8 = b's';
pub const TAG_GUID      :u8 = b'g';
pub const TAG_LIST      :u8 = b'a';
pub const TAG_MAP       :u8 = b'm';
pub const TAG_CLASS     :u8 = b'c';
pub const TAG_OBJECT    :u8 = b'o';
pub const TAG_REF       :u8 = b'r';

// Serialize Marks;
pub const TAG_POS       :u8 = b'+';
pub const TAG_NEG       :u8 = b'-';
pub const TAG_SEMICOLON :u8 = b';';
pub const TAG_OPENBRACE :u8 = b'{';
pub const TAG_CLOSEBRACE:u8 = b'}';
pub const TAG_QUOTE     :u8 = b'"';
pub const TAG_POINT     :u8 = b'.';

// Protocol Tags;
pub const TAG_FUNCTIONS :u8 = b'F';
pub const TAG_CALL      :u8 = b'C';
pub const TAG_RESULT    :u8 = b'R';
pub const TAG_ARGUMENT  :u8 = b'A';
pub const TAG_ERROR     :u8 = b'E';
pub const TAG_END       :u8 = b'z';
