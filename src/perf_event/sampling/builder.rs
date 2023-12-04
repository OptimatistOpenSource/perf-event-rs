use crate::infra::WrapResult;
use crate::sampling::single::Sampling;
use crate::sampling::{Attr, SamplingGroup};
use crate::{BuildError, Builder};

// TODO
impl Builder {
    pub const fn mmap_pages(mut self, mmap_pages: usize) -> Self {
        self.mmap_pages = Some(mmap_pages);
        self
    }

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
                mmap_pages: Some(mmap_pages),
                ..
            } => unsafe { Sampling::new(attr, *pid, *cpu, -1, 0, *mmap_pages) }
                .map_err(BuildError::SyscallFailed),
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
                mmap_pages: Some(mmap_pages),
                ..
            } => unsafe { SamplingGroup::new(*pid, *cpu, *mmap_pages) }.wrap_ok(),
            _ => todo!(),
        }
    }
}
