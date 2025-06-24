use super::prelude::*;

#[derive(Debug)]
pub struct qHostInfo;

impl<'a> ParseCommand<'a> for qHostInfo {
    #[inline(always)]
    fn from_packet(buf: PacketBuf<'a>) -> Option<Self> {
        if !buf.into_body().is_empty() {
            return None;
        }
        Some(qHostInfo)
    }
}
