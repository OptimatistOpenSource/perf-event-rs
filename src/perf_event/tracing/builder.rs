use crate::tracing::{Attr, Tracing};
use crate::{BuildError, Builder};

impl Builder {
    pub fn build_tracing(&self, attr: &Attr) -> Result<Tracing, BuildError> {
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
            } => unsafe { Tracing::new(attr, *pid, *cpu, -1, 0, *mmap_pages) }
                .map_err(BuildError::SyscallFailed),
        }
    }
}
