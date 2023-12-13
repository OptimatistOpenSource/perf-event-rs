use crate::infra::WrapResult;
use crate::sampling::single::Sampling;
use crate::sampling::{Attr, SamplingGroup};
use crate::{BuildError, Builder};

impl Builder {
    pub fn build_sampling(&self, attr: &Attr) -> Result<Sampling, BuildError> {
        match self {
            Self { pid: None, .. } => Err(BuildError::PidNotSet),
            Self { cpu: None, .. } => Err(BuildError::CpuNotSet),
            Self {
                mmap_pages: None, ..
            } => Err(BuildError::RingBufferSizeNotSet),
            Self {
                pid: Some(pid),
                cpu: Some(cpu),
                mmap_pages: Some(mmap_pages),
                ..
            } => unsafe { Sampling::new(attr, *pid, *cpu, -1, 0, *mmap_pages) }
                .map_err(BuildError::SyscallFailed),
        }
    }

    pub fn build_sampling_group(&self) -> Result<SamplingGroup, BuildError> {
        match self {
            Self { pid: None, .. } => Err(BuildError::PidNotSet),
            Self { cpu: None, .. } => Err(BuildError::CpuNotSet),
            Self {
                mmap_pages: None, ..
            } => Err(BuildError::RingBufferSizeNotSet),
            Self {
                pid: Some(pid),
                cpu: Some(cpu),
                mmap_pages: Some(mmap_pages),
                ..
            } => unsafe { SamplingGroup::new(*pid, *cpu, *mmap_pages) }.wrap_ok(),
        }
    }
}