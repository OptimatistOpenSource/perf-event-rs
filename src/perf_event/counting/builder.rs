use crate::counting::{Attr, Counting};
use crate::{BuildError, Builder};

impl Builder {
    // TODO
    pub fn build_counting(self, attr: Attr) -> Result<Counting, BuildError> {
        match self {
            Builder {
                pid: None,
                cpu: None,
                ..
            } => Err(BuildError::PidAndCpuNotSet),
            Builder {
                pid: Some(pid),
                cpu: Some(cpu),
                ..
            } => unsafe { Counting::new(attr, pid, cpu, -1, 0) }.map_err(BuildError::SyscallFailed),
            _ => todo!(),
        }
    }
}
