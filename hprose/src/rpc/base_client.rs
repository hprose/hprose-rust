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

use std::time::Duration;

use io;
use io::{Writer, Encoder, Encodable, Reader, Decoder, Decodable};
use io::tags::*;

use super::*;

/// BaseClient is the hprose base client
pub struct BaseClient<T: Transporter> {
    trans: T,
    filter_manager: FilterManager,
    uri: String,
    timeout: Option<Duration>
}

impl<T: Transporter> BaseClient<T> {
    #[inline]
    pub fn new(trans: T, uri: String) -> BaseClient<T> {
        BaseClient {
            trans: trans,
            filter_manager: FilterManager::new(),
            uri: uri,
            timeout: Some(Duration::from_secs(30))
        }
    }

    pub fn invoke<R: Decodable, A: Encodable>(&self, name: &str, args: &mut Vec<A>, settings: Option<InvokeSettings>) -> InvokeResult<R> {
        let odata = self.encode(name, args, settings);
        self.trans.send_and_receive(&self.uri, &odata).and_then(|idata| self.decode(idata, args))
    }

    fn encode<A: Encodable>(&self, name: &str, args: &mut Vec<A>, settings: Option<InvokeSettings>) -> Vec<u8> {
        let mut w = Writer::new(true);
        w.write_byte(TAG_CALL);
        w.write_str(name);
        let by_ref = settings.map_or(false, |s| s.by_ref);
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

    fn decode<R: Decodable, A: Encodable>(&self, data: Vec<u8>, args: &mut Vec<A>) -> InvokeResult<R> {
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

    //    fn before_filter(&self, request: &[u8]) -> &[u8] {
    //        let request = self.filter_manager.output(request);
    //        let response = self.after_filter(request);
    //        self.filter_manager.input(response)
    //    }
    //
    //    fn after_filter(&self, request: &[u8]) -> &[u8] {
    //        self.trans.send_and_receive(&self.uri, request)
    //    }
}
