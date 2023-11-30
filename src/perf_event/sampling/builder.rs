use crate::infra::WrapResult;
use crate::sampling::single::Sampling;
use crate::sampling::{Attr, SamplingGroup};
use crate::{BuildError, Builder};

// TODO
impl Builder {
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

    pub fn build_sampling_group(&self) -> Result<SamplingGroup, BuildError> {
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
            } => unsafe { SamplingGroup::new(*pid, *cpu) }.wrap_ok(),
            _ => todo!(),
        }
    }
}
