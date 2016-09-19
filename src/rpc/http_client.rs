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
 * LastModified: Sep 19, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate hyper;

use super::*;
use io::Hprose;

pub struct HttpTransporter {}

impl HttpTransporter {
    #[inline]
    pub fn new() -> HttpTransporter {
        HttpTransporter {}
    }
}

impl Transporter for HttpTransporter {
    fn send_and_receive(uri: String, data: Vec<u8>) -> Vec<u8> {
        unimplemented!()
    }
}

pub struct HttpClient {
    base_client: BaseClient<HttpTransporter>,
    client: hyper::client::Client
}

impl HttpClient {
    pub fn new(url: String) -> HttpClient {
        HttpClient {
            base_client: BaseClient::new(HttpTransporter::new(), url),
            client: hyper::client::Client::new()
        }
    }
}

impl Client for HttpClient {
    fn invoke(&self, name: String, args: Vec<Hprose>) {}
}
