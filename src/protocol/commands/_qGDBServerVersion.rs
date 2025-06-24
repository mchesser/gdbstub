use super::prelude::*;

#[derive(Debug)]
pub struct qGDBServerVersion;

impl<'a> ParseCommand<'a> for qGDBServerVersion {
    #[inline(always)]
    fn from_packet(buf: PacketBuf<'a>) -> Option<Self> {
        if !buf.into_body().is_empty() {
            return None;
        }
        Some(qGDBServerVersion)
    }
}
