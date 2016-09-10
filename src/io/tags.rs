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
 * LastModified: Sep 9, 2016                              *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

// Hprose Tags

// Serialize Type;
pub const TagInteger  :u8 = 'i' as u8;
pub const TagLong     :u8 = 'l' as u8;
pub const TagDouble   :u8 = 'd' as u8;
pub const TagNull     :u8 = 'n' as u8;
pub const TagEmpty    :u8 = 'e' as u8;
pub const TagTrue     :u8 = 't' as u8;
pub const TagFalse    :u8 = 'f' as u8;
pub const TagNaN      :u8 = 'N' as u8;
pub const TagInfinity :u8 = 'I' as u8;
pub const TagDate     :u8 = 'D' as u8;
pub const TagTime     :u8 = 'T' as u8;
pub const TagUTC      :u8 = 'Z' as u8;
pub const TagBytes    :u8 = 'b' as u8;
pub const TagUTF8Char :u8 = 'u' as u8;
pub const TagString   :u8 = 's' as u8;
pub const TagGUID     :u8 = 'g' as u8;
pub const TagList     :u8 = 'a' as u8;
pub const TagMap      :u8 = 'm' as u8;
pub const TagClass    :u8 = 'c' as u8;
pub const TagObject   :u8 = 'o' as u8;
pub const TagRef      :u8 = 'r' as u8;

// Serialize Marks;
pub const TagPos        :u8 = '+' as u8;
pub const TagNeg        :u8 = '-' as u8;
pub const TagSemicolon  :u8 = ';' as u8;
pub const TagOpenbrace  :u8 = '{' as u8;
pub const TagClosebrace :u8 = '}' as u8;
pub const TagQuote      :u8 = '"' as u8;
pub const TagPoint      :u8 = '.' as u8;

// Protocol Tags;
pub const TagFunctions :u8 = 'F' as u8;
pub const TagCall      :u8 = 'C' as u8;
pub const TagResult    :u8 = 'R' as u8;
pub const TagArgument  :u8 = 'A' as u8;
pub const TagError     :u8 = 'E' as u8;
pub const TagEnd       :u8 = 'z' as u8;
