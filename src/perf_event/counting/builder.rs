use crate::counting::{Counting, CountingAttr};
use crate::{BuildError, Builder};

impl Builder {
    // TODO
    pub fn build_counting(self, attr: CountingAttr) -> Result<Counting, BuildError> {
        match self {
            Builder {
                pid: None,
                cpu: None,
                ..
            } => Err(BuildError::PidAndCpuNotSet),
            Builder {
                pid: Some(pid),
                cpu: Some(cpu),
                group_fd: Some(group_fd),
                flags: Some(flags),
            } => unsafe { Counting::new(attr, pid, cpu, group_fd, flags) }
                .map_err(BuildError::SyscallFailed),
            _ => todo!(),
        }
    }
}
