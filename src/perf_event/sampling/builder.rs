use crate::infra::WrapResult;
use crate::sampling::single::Sampler;
use crate::sampling::{Config, SamplerGroup};
use crate::{BuildError, Builder};

impl Builder {
    pub fn build_sampling(&self, cfg: &Config) -> Result<Sampler, BuildError> {
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
            } => unsafe { Sampler::new(cfg, *pid, *cpu, -1, 0, *mmap_pages) }
                .map_err(BuildError::SyscallFailed),
        }
    }

    pub fn build_sampling_group(&self) -> Result<SamplerGroup, BuildError> {
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
            } => unsafe { SamplerGroup::new(*pid, *cpu, *mmap_pages) }.wrap_ok(),
        }
    }
}
