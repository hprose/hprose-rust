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
 * rpc/result_mode.rs                                     *
 *                                                        *
 * hprose ResultMode enum for Rust.                       *
 *                                                        *
 * LastModified: Sep 22, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

/// ResultMode is result mode
pub enum ResultMode {
    /// Normal is default mode
    Normal,
    /// Serialized means the result is serialized
    Serialized,
    /// Raw means the result is the raw bytes data
    Raw,
    /// RawWithEndTag means the result is the raw bytes data with the end tag
    RawWithEndTag
}
