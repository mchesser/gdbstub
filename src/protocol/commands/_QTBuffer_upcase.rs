use super::prelude::*;
use crate::target::ext::tracepoints::BufferShape;
use crate::target::ext::tracepoints::TraceBufferConfig;

#[derive(Debug)]
pub struct QTBuffer(pub TraceBufferConfig);

impl ParseCommand<'_> for QTBuffer {
    #[inline(always)]
    fn from_packet(buf: PacketBuf<'_>) -> Option<Self> {
        match buf.into_body() {
            [b':', body @ ..] => {
                let mut s = body.splitn_mut(2, |b| *b == b':');
                let opt = s.next()?;
                // Clippy incorrect thinks this as_ref isn't needed, but it is.
                #[allow(clippy::useless_asref)]
                match opt.as_ref() {
                    b"circular" => {
                        let shape = s.next()?;
                        Some(QTBuffer(TraceBufferConfig::Shape(match shape {
                            [b'1'] => Some(BufferShape::Circular),
                            [b'0'] => Some(BufferShape::Linear),
                            _ => None,
                        }?)))
                    }
                    b"size" => {
                        let size = s.next()?;
                        Some(QTBuffer(TraceBufferConfig::Size(match size {
                            [b'-', b'1'] => None,
                            i => Some(decode_hex(i).ok()?),
                        })))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
