use super::prelude::*;
use crate::protocol::commands::ext::LldbProcessInfo;
use crate::target::ext::lldb_process_info::ProcessInfo;

impl<T: Target, C: Connection> GdbStubImpl<T, C> {
    pub(crate) fn handle_lldb_process_info(
        &mut self,
        res: &mut ResponseWriter<'_, C>,
        target: &mut T,
        command: LldbProcessInfo,
    ) -> Result<HandlerStatus, Error<T::Error, C::Error>> {
        let ops = match target.support_lldb_process_info() {
            Some(ops) => ops,
            None => return Ok(HandlerStatus::Handled),
        };

        let write_process_info = |res: &mut ResponseWriter<'_, C>,
                                  p: &ProcessInfo|
         -> Result<(), Error<T::Error, C::Error>> {
            res.write_str("pid:")?;
            res.write_dec(p.pid)?;
            res.write_str(";ppid:")?;
            res.write_dec(p.parent_pid)?;
            res.write_str(";uid:")?;
            res.write_dec(p.uid)?;
            res.write_str(";gid:")?;
            res.write_dec(p.gid)?;
            res.write_str(";euid:")?;
            res.write_dec(p.euid)?;
            res.write_str(";name:")?;
            res.write_hex_buf(&p.name)?;
            res.write_str(";args:")?;
            res.write_hex_buf(&p.args)?;
            res.write_str(";triple:")?;
            // Even though the documentation states that the `triple` field is a
            // `String`, it is actually encoded as a hex buffer.
            res.write_hex_buf(&p.triple)?;
            res.write_str(";")?;
            Ok(())
        };

        match command {
            LldbProcessInfo::qProcessInfo(_) => {
                if let Some(p) = ops.get_current_process_info() {
                    write_process_info(res, &p)?
                } else {
                    // Return any EXX error when there are no processes remaining (See:
                    // `lldb_register_info.rs`).
                    res.write_str("E45")?;
                }
            }
            LldbProcessInfo::qfProcessInfo(a) => {
                // @todo: Implement `qfProcessInfo` command handling.
                _ = a;
                let mut i = 0;
                while let Some(p) = ops.get_process_info("", i) {
                    write_process_info(res, &p)?;
                    i += 1;
                }
                // Return any EXX error when there are no processes remaining (See:
                // `lldb_register_info.rs`).
                res.write_str("E45")?;
            }
            LldbProcessInfo::qsProcessInfo(_a) => {
                // Return any EXX error when there are no processes remaining (See:
                // `lldb_register_info.rs`).
                res.write_str("E45")?;
            }
        };

        Ok(HandlerStatus::Handled)
    }
}
