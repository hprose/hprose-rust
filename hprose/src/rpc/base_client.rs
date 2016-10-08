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
 * rpc/base_client.rs                                     *
 *                                                        *
 * hprose rpc base client for Rust.                       *
 *                                                        *
 * LastModified: Oct 8, 2016                              *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use io;
use io::{Writer, Encoder, Encodable, Reader, Decoder, Decodable};
use io::tags::*;

use super::*;

/// BaseClient is the hprose base client
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

    pub fn invoke<R: Decodable, A: Encodable, C: Client>(&self, name: &str, args: &mut Vec<A>, options: &InvokeOptions, context: &ClientContext<C>) -> InvokeResult<R> {
        let odata = self.do_output(name, args, options, context);
        self.trans.send_and_receive(&self.url, &odata).and_then(|idata| self.do_input(idata, args, context))
    }

    pub fn do_output<A: Encodable, C: Client>(&self, name: &str, args: &mut Vec<A>, options: &InvokeOptions, context: &ClientContext<C>) -> Vec<u8> {
        let mut w = Writer::new(true);
        w.write_byte(TAG_CALL);
        w.write_str(name);
        let by_ref = options.by_ref;
        let len = args.len();
        if len > 0 || by_ref {
            w.write_seq(args.len(), |w| {
                for e in args {
                    e.encode(w);
                }
            });
            if by_ref {
                w.write_bool(true);
            }
        }
        w.write_byte(TAG_END);
        w.into_bytes()
    }

    pub fn do_input<R: Decodable, A: Encodable, C: Client>(&self, data: Vec<u8>, args: &mut Vec<A>, context: &ClientContext<C>) -> InvokeResult<R> {
        let mut r = Reader::new(&data, false);
        r.byte_reader.read_byte()
            .map_err(|e| InvokeError::DecoderError(io::DecoderError::ParserError(e)))
            .and_then(|tag| match tag {
                TAG_RESULT => r.unserialize::<R>().map_err(|e| InvokeError::DecoderError(e)),
                //                TAG_ARGUMENT => (),
                TAG_ERROR => r.read_string().map_err(|e| InvokeError::DecoderError(e)).and_then(|s| Err(InvokeError::RemoteError(s))),
                _ => Err(InvokeError::WrongResponse(data.clone())),
            })
    }
}
