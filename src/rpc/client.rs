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
 * LastModified: Sep 19, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io::Hprose;
use io::{Writer, ByteWriter, Encoder, Encodable};
use io::tags::*;

pub trait Client {
    fn invoke(&self, name: String, args: Vec<Hprose>);
}

pub trait Transporter {
    fn send_and_receive(uri: String, data: Vec<u8>) -> Vec<u8>;
}

pub struct ClientContext<'a, T: 'a + Client> {
    client: &'a T
}

impl<'a, T: 'a + Client> ClientContext<'a, T> {
    #[inline]
    pub fn new(client: &'a T) -> ClientContext<'a, T> {
        ClientContext {
            client: client
        }
    }
}

pub struct BaseClient<T: Transporter> {
    trans: T,
    url: String
}

impl<T: Transporter> BaseClient<T> {
    #[inline]
    pub fn new(trans: T, url: String) -> BaseClient<T> {
        BaseClient {
            trans: trans,
            url: url
        }
    }

    pub fn do_output<C: Client>(&self, name: String, args: Vec<Hprose>, context: &ClientContext<C>) -> Vec<u8> {
        let mut w = Writer::new(true);
        w.write_byte(TAG_CALL);
        w.write_str(&name);
        w.write_seq(args.len(), |w| {
            for e in args {
                e.encode(w);
            }
        });
        w.write_byte(TAG_END);
        w.bytes()
    }
}
