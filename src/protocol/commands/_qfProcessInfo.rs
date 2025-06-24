use super::prelude::*;

#[derive(Debug)]
pub struct qfProcessInfo;

impl<'a> ParseCommand<'a> for qfProcessInfo {
    #[inline(always)]
    fn from_packet(buf: PacketBuf<'a>) -> Option<Self> {
        if !buf.into_body().is_empty() {
            return None;
        }
        Some(qfProcessInfo)
    }
}
