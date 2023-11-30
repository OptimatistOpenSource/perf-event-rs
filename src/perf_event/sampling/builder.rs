use crate::sampling::{Attr, Sampling};
use crate::{BuildError, Builder};

impl Builder {
    // TODO
    pub fn build_sampling(&self, attr: &Attr) -> Result<Sampling, BuildError> {
        match self {
            Self {
                pid: None,
                cpu: None,
                ..
            } => Err(BuildError::PidAndCpuNotSet),
            Self {
                pid: Some(pid),
                cpu: Some(cpu),
                ..
            } => {
                unsafe { Sampling::new(attr, *pid, *cpu, -1, 0) }.map_err(BuildError::SyscallFailed)
            }
            _ => todo!(),
        }
    }
}
