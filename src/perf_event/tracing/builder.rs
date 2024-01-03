use crate::tracing::{Config, Tracer};
use crate::{BuildError, Builder};

impl Builder {
    pub fn build_tracing(&self, cfg: &Config) -> Result<Tracer, BuildError> {
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
            } => unsafe { Tracer::new(cfg, *pid, *cpu, -1, 0, *mmap_pages) }
                .map_err(BuildError::SyscallFailed),
        }
    }
}
