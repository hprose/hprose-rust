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

use io;
use io::Hprose;
use io::{Writer, ByteWriter, Encoder, Encodable, Reader, ByteReader, Decoder, Decodable, DecodeResult};
use io::tags::*;

#[derive(Clone, PartialEq, Debug)]
pub enum InvokeError {
    TransError(String),
    DecoderError(io::DecoderError),
    RemoteError(String),
    WrongResponse(Vec<u8>)
}

pub type InvokeResult<T> = Result<T, InvokeError>;

use self::InvokeError::*;

pub trait Client {
    fn invoke<R: Decodable, A: Encodable>(&self, name: &str, args: Vec<A>) -> InvokeResult<R>;
}

pub trait Transporter {
    fn send_and_receive(&self, uri: &str, data: &[u8]) -> Result<Vec<u8>, InvokeError>;
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

    pub fn invoke<R: Decodable, A: Encodable, C: Client>(&self, name: &str, args: Vec<A>, context: &ClientContext<C>) -> InvokeResult<R> {
        let odata = self.do_output(name, &args, context);
        self.trans.send_and_receive(&self.url, &odata).and_then(|idata| self.do_input(idata, &args, context))
    }

    pub fn do_output<A: Encodable, C: Client>(&self, name: &str, args: &Vec<A>, context: &ClientContext<C>) -> Vec<u8> {
        let mut w = Writer::new(true);
        w.write_byte(TAG_CALL);
        w.write_str(name);
        w.write_seq(args.len(), |w| {
            for e in args {
                e.encode(w);
            }
        });
        w.write_byte(TAG_END);
        w.bytes()
    }

    pub fn do_input<R: Decodable, A: Encodable, C: Client>(&self, data: Vec<u8>, args: &Vec<A>, context: &ClientContext<C>) -> InvokeResult<R> {
        let mut r = Reader::new(&data);
        r.reader.read_byte()
            .map_err(|e| DecoderError(io::DecoderError::ParserError(e)))
            .and_then(|tag| match tag {
                TAG_RESULT => r.unserialize::<R>().map_err(|e| InvokeError::DecoderError(e)),
                //                TAG_ARGUMENT => (),
                TAG_ERROR => r.read_string().map_err(|e| InvokeError::DecoderError(e)).and_then(|s| Err(InvokeError::RemoteError(s))),
                _ => Err(InvokeError::WrongResponse(data.clone())),
            })
    }
}
