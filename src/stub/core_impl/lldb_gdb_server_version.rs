use super::prelude::*;
use crate::protocol::commands::ext::LldbGdbServerVersion;

impl<T: Target, C: Connection> GdbStubImpl<T, C> {
    pub(crate) fn handle_lldb_gdb_server_version(
        &mut self,
        res: &mut ResponseWriter<'_, C>,
        target: &mut T,
        _command: LldbGdbServerVersion,
    ) -> Result<HandlerStatus, Error<T::Error, C::Error>> {
        if let Some(v) = target.support_lldb_gdb_server_version() {
            res.write_str("name:")?;
            res.write_str(v.name)?;
            res.write_str(";version:")?;
            res.write_str(v.version)?;
            res.write_str(";")?;
        }
        Ok(HandlerStatus::Handled)
    }
}
