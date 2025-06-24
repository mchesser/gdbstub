//! (LLDB Extension) Get process info for one or more processes on the remote
//! platform.

use crate::target::Target;

pub struct ProcessInfo {
    pub pid: u32,
    pub parent_pid: u32,
    pub uid: u32,
    pub gid: u32,
    pub euid: u32,
    pub egid: u32,
    /// The path to the executable file of the process (this is _not_ the
    /// process name).
    pub name: Vec<u8>,
    pub args: Vec<u8>,
    pub triple: Vec<u8>,
}

pub trait LldbProcessInfo: Target {
    fn get_process(&mut self, req: &str, nth: usize) -> Option<ProcessInfo>;
}

define_ext!(LldbProcessInfoOps, LldbProcessInfo);
