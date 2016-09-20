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
 * rpc/http_client.rs                                     *
 *                                                        *
 * hprose http client for Rust.                           *
 *                                                        *
 * LastModified: Sep 20, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate hyper;

use self::hyper::client::Client as HyperClient;
use self::hyper::Error as HyperError;

use super::*;
use io::{Hprose, Decodable, Encodable};

use std::io::Read;
use std::error::Error;

pub struct HttpTransporter {
    client: HyperClient
}

impl HttpTransporter {
    #[inline]
    pub fn new() -> HttpTransporter {
        HttpTransporter {
            client: HyperClient::new()
        }
    }
}

impl Transporter for HttpTransporter {
    fn send_and_receive(&self, uri: &str, data: &[u8]) -> Result<Vec<u8>, InvokeError> {
        self.client.post(uri).body(data).send().map(|mut resp| {
            let mut ret = Vec::new();
            resp.read_to_end(&mut ret);
            ret
        }).map_err(|e| InvokeError::TransError(String::from(e.description())))
    }
}

pub struct HttpClient {
    base_client: BaseClient<HttpTransporter>,
}

impl HttpClient {
    pub fn new(url: String) -> HttpClient {
        HttpClient {
            base_client: BaseClient::new(HttpTransporter::new(), url)
        }
    }
}

impl Client for HttpClient {
    fn invoke<R: Decodable, A: Encodable>(&self, name: &str, args: Vec<A>) -> InvokeResult<R> {
        let context = ClientContext::new(self);
        self.base_client.invoke::<R, A, HttpClient>(name, args, &context)
    }
}
