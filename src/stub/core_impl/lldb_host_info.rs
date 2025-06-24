use super::prelude::*;
use crate::protocol::commands::ext::LldbHostInfo;

impl<T: Target, C: Connection> GdbStubImpl<T, C> {
    pub(crate) fn handle_lldb_host_info(
        &mut self,
        res: &mut ResponseWriter<'_, C>,
        target: &mut T,
        command: LldbHostInfo,
    ) -> Result<HandlerStatus, Error<T::Error, C::Error>> {
        let ops = match target.support_lldb_host_info() {
            Some(ops) => ops,
            None => return Ok(HandlerStatus::Handled),
        };

        let handler_status = match command {
            LldbHostInfo::qHostInfo(_) => {
                if let Some(cputype) = ops.cputype() {
                    res.write_str("cputype:")?;
                    res.write_dec(cputype)?;
                    res.write_str(";")?;
                }
                if let Some(cpusubtype) = ops.cpusubtype() {
                    res.write_str("cpusubtype:")?;
                    res.write_dec(cpusubtype)?;
                    res.write_str(";")?;
                }
                if let Some(arch) = ops.arch() {
                    res.write_str("arch:")?;
                    res.write_str(arch)?;
                    res.write_str(";")?;
                }
                if let Some(triple) = ops.triple() {
                    res.write_str("triple:")?;
                    res.write_hex_buf(&triple)?;
                    res.write_str(";")?;
                }
                if let Some(vendor) = ops.vendor() {
                    res.write_str("vendor:")?;
                    res.write_str(vendor)?;
                    res.write_str(";")?;
                }
                if let Some(ostype) = ops.ostype() {
                    res.write_str("ostype:")?;
                    res.write_str(ostype)?;
                    res.write_str(";")?;
                }
                if let Some(endian) = ops.endian() {
                    res.write_str("endian:")?;
                    res.write_str(endian)?;
                    res.write_str(";")?;
                }
                if let Some(ptrsize) = ops.ptrsize() {
                    res.write_str("ptrsize:")?;
                    res.write_dec(ptrsize)?;
                    res.write_str(";")?;
                }
                if let Some(hostname) = ops.hostname() {
                    res.write_str("hostname:")?;
                    res.write_str(hostname)?;
                    res.write_str(";")?;
                }
                if let Some(os_build) = ops.os_build() {
                    res.write_str("os_build:")?;
                    res.write_str(os_build)?;
                    res.write_str(";")?;
                }
                if let Some(os_kernel) = ops.os_kernel() {
                    res.write_str("os_kernel:")?;
                    res.write_str(os_kernel)?;
                    res.write_str(";")?;
                }
                if let Some(os_version) = ops.os_version() {
                    res.write_str("os_version:")?;
                    res.write_str(os_version)?;
                    res.write_str(";")?;
                }
                if let Some(watchpoint_exceptions_received) = ops.watchpoint_exceptions_received() {
                    res.write_str("watchpoint_exceptions_received:")?;
                    res.write_str(watchpoint_exceptions_received)?;
                    res.write_str(";")?;
                }
                if let Some(default_packet_timeout) = ops.default_packet_timeout() {
                    res.write_str("default_packet_timeout:")?;
                    res.write_dec(default_packet_timeout)?;
                    res.write_str(";")?;
                }
                if let Some(distribution_id) = ops.distribution_id() {
                    res.write_str("distribution_id:")?;
                    res.write_str(distribution_id)?;
                    res.write_str(";")?;
                }
                if let Some(osmajor) = ops.osmajor() {
                    res.write_str("osmajor:")?;
                    res.write_dec(osmajor)?;
                    res.write_str(";")?;
                }
                if let Some(osminor) = ops.osminor() {
                    res.write_str("osminor:")?;
                    res.write_dec(osminor)?;
                    res.write_str(";")?;
                }
                if let Some(ospatch) = ops.ospatch() {
                    res.write_str("ospatch:")?;
                    res.write_dec(ospatch)?;
                    res.write_str(";")?;
                }
                if let Some(vm_page_size) = ops.vm_page_size() {
                    res.write_str("vm_page_size:")?;
                    res.write_dec(vm_page_size)?;
                    res.write_str(";")?;
                }
                if let Some(addressing_bits) = ops.addressing_bits() {
                    res.write_str("addressing_bits:")?;
                    res.write_dec(addressing_bits)?;
                    res.write_str(";")?;
                }
                if let Some(low_mem_addressing_bits) = ops.low_mem_addressing_bits() {
                    res.write_str("low_mem_addressing_bits:")?;
                    res.write_dec(low_mem_addressing_bits)?;
                    res.write_str(";")?;
                }
                if let Some(high_mem_addressing_bits) = ops.high_mem_addressing_bits() {
                    res.write_str("high_mem_addressing_bits:")?;
                    res.write_dec(high_mem_addressing_bits)?;
                    res.write_str(";")?;
                }

                HandlerStatus::Handled
            }
        };

        Ok(handler_status)
    }
}
