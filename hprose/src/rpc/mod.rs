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
 * rpc/mod.rs                                             *
 *                                                        *
 * hprose rpc module for Rust.                            *
 *                                                        *
 * LastModified: Oct 9, 2016                              *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

mod client;
mod filter;
mod base_client;
mod http_client;
mod result_mode;

pub use self::client::{InvokeSettings, InvokeResult, InvokeError, Client, Transporter, ClientContext};
pub use self::filter::{Filter, FilterManager};
pub use self::base_client::BaseClient;
pub use self::http_client::{HttpClient, HttpTransporter};
pub use self::result_mode::ResultMode;
