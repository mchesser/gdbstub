use super::prelude::*;

#[derive(Debug)]
pub struct qsProcessInfo;

impl<'a> ParseCommand<'a> for qsProcessInfo {
    #[inline(always)]
    fn from_packet(buf: PacketBuf<'a>) -> Option<Self> {
        if !buf.into_body().is_empty() {
            return None;
        }
        Some(qsProcessInfo)
    }
}
