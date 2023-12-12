use crate::counting::{Attr, Counting, CountingGroup};
use crate::infra::WrapResult;
use crate::{BuildError, Builder};

impl Builder {
    pub fn build_counting(&self, attr: &Attr) -> Result<Counting, BuildError> {
        match self {
            Self { pid: None, .. } => Err(BuildError::PidNotSet),
            Self { cpu: None, .. } => Err(BuildError::CpuNotSet),
            Self {
                pid: Some(pid),
                cpu: Some(cpu),
                ..
            } => {
                unsafe { Counting::new(attr, *pid, *cpu, -1, 0) }.map_err(BuildError::SyscallFailed)
            }
        }
    }

    pub fn build_counting_group(&self) -> Result<CountingGroup, BuildError> {
        match self {
            Self { pid: None, .. } => Err(BuildError::PidNotSet),
            Self { cpu: None, .. } => Err(BuildError::CpuNotSet),
            Self {
                pid: Some(pid),
                cpu: Some(cpu),
                ..
            } => unsafe { CountingGroup::new(*pid, *cpu) }.wrap_ok(),
        }
    }
}
