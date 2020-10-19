//! Enables [Reverse Execution](https://sourceware.org/gdb/current/onlinedocs/gdb/Reverse-Execution.html)
//! functionality.

use crate::{arch::Arch, target::Target};

use super::base::singlethread::StopReason;

/// Target Extension - Support
/// [Reverse Execution](https://sourceware.org/gdb/current/onlinedocs/gdb/Reverse-Execution.html) functionality.
pub trait ReverseExecution: Target {
    /// Execute a single instruction in reverse.
    fn reverse_step(&mut self) -> Result<StopReason<<Self::Arch as Arch>::Usize>, Self::Error>;

    /// Execute the target system in reverse.
    fn reverse_continue(&mut self) -> Result<StopReason<<Self::Arch as Arch>::Usize>, Self::Error>;
}

define_ext!(ReverseExecutionOps, ReverseExecution);
