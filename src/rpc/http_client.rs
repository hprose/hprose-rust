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

use super::*;
use io::Hprose;
use io::Decodable;

use std::io::Read;

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
    fn send_and_receive(&self, uri: &str, data: Vec<u8>) -> Vec<u8> {
        let mut resp = self.client.post(uri).body(data.as_slice()).send().unwrap();
        let mut ret = Vec::new();
        resp.read_to_end(&mut ret);
        ret
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
    fn invoke<R: Decodable>(&self, name: String, args: Vec<Hprose>) -> R {
        let context = ClientContext::new(self);
        self.base_client.invoke::<R, HttpClient>(name, args, &context)
    }
}
