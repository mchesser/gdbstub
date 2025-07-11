use super::copy_range_to_buf;
use crate::emu::Emu;
use gdbstub::target;
use gdbstub::target::TargetResult;

impl target::ext::auxv::Auxv for Emu {
    fn get_auxv(
        &mut self,
        offset: u64,
        length: usize,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        let auxv = b"\x00\x00\x00\x00\x00\x00\x00\x00";
        Ok(copy_range_to_buf(auxv, offset, length, buf))
    }
}
