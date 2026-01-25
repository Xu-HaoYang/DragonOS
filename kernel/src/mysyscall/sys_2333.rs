use system_error::SystemError;

use crate::arch::interrupt::TrapFrame;
use crate::syscall::table::Syscall;
use alloc::string::ToString;
use alloc::vec::Vec;

pub const SYS_2333: usize = 2333;

/// System call handler for the `2333` syscall
pub struct Sys2333Handle;

impl Syscall for Sys2333Handle {
    fn num_args(&self) -> usize {
        0
    }

    fn handle(&self, args: &[usize], frame: &mut TrapFrame) -> Result<usize, SystemError> {
        log::info("syscall 2333 called");
        return Ok(6666);
    }

    fn entry_format(&self, args: &[usize]) -> Vec<FormattedSyscallParam> {
        vec![]
    }
}

impl Sys2333Handle {}

syscall_table_macros::declare_syscall!(SYS_2333, Sys2333Handle);
