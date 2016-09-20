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
 * LastModified: Sep 20, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io::Hprose;
use io::{Writer, ByteWriter, Encoder, Encodable, Reader, ByteReader, Decoder, Decodable};
use io::tags::*;

pub trait Client {
    fn invoke<R: Decodable>(&self, name: String, args: Vec<Hprose>) -> R;
}

pub trait Transporter {
    fn send_and_receive(&self, uri: &str, data: Vec<u8>) -> Vec<u8>;
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

    pub fn invoke<R: Decodable, C: Client>(&self, name: String, args: Vec<Hprose>, context: &ClientContext<C>) -> R {
        let odata = self.do_output(name, &args, context);
        let idata = self.trans.send_and_receive(&self.url, odata);
        self.do_input(idata, &args, context)
    }

    pub fn do_output<C: Client>(&self, name: String, args: &Vec<Hprose>, context: &ClientContext<C>) -> Vec<u8> {
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

    pub fn do_input<R: Decodable, C: Client>(&self, data: Vec<u8>, args: &Vec<Hprose>, context: &ClientContext<C>) -> R {
        let mut r = Reader::new(&data);
        r.reader.read_byte().unwrap();
        r.unserialize::<R>().unwrap()
    }
}
