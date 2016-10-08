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
 * rpc/client.rs                                          *
 *                                                        *
 * hprose client for Rust.                                *
 *                                                        *
 * LastModified: Oct 8, 2016                              *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use std::time::Duration;

use io;
use io::{Encodable, Decodable};

use super::ResultMode;

/// InvokeSettings is the invoke settings of hprose client
pub struct InvokeSettings {
    pub by_ref: bool,
    pub simple: bool,
    pub idempotent: bool,
    pub failswitch: bool,
    pub oneway: bool,
    pub retry: i32,
    pub mode: ResultMode,
    pub timeout: Option<Duration>
}

#[derive(Clone, PartialEq, Debug)]
pub enum InvokeError {
    TransError(String),
    DecoderError(io::DecoderError),
    RemoteError(String),
    WrongResponse(Vec<u8>)
}

pub type InvokeResult<T> = Result<T, InvokeError>;

/// Client is hprose client
pub trait Client {
    fn invoke<R: Decodable, A: Encodable>(&self, name: &str, args: &mut Vec<A>, settings: Option<InvokeSettings>) -> InvokeResult<R>;
}

/// Transporter is the hprose client transporter
pub trait Transporter {
    fn send_and_receive(&self, uri: &str, data: &[u8]) -> Result<Vec<u8>, InvokeError>;
}

/// ClientContext is the hprose client context
pub struct ClientContext<'a, T: 'a + Client> {
    settings: &'a Option<InvokeSettings>,
    client: &'a T
}

impl<'a, T: 'a + Client> ClientContext<'a, T> {
    #[inline]
    pub fn new(client: &'a T, settings: &'a Option<InvokeSettings>) -> ClientContext<'a, T> {
        ClientContext {
            client: client,
            settings: settings
        }
    }
}
