use crate::counting::{Config, Counter, CounterGroup};
use crate::infra::WrapResult;
use crate::{BuildError, Builder};

impl Builder {
    pub fn build_counting(&self, cfg: &Config) -> Result<Counter, BuildError> {
        match self {
            Self { pid: None, .. } => Err(BuildError::PidNotSet),
            Self { cpu: None, .. } => Err(BuildError::CpuNotSet),
            Self {
                pid: Some(pid),
                cpu: Some(cpu),
                ..
            } => {
                unsafe { Counter::new(cfg, *pid, *cpu, -1, 0) }.map_err(BuildError::SyscallFailed)
            }
        }
    }

    pub fn build_counting_group(&self) -> Result<CounterGroup, BuildError> {
        match self {
            Self { pid: None, .. } => Err(BuildError::PidNotSet),
            Self { cpu: None, .. } => Err(BuildError::CpuNotSet),
            Self {
                pid: Some(pid),
                cpu: Some(cpu),
                ..
            } => unsafe { CounterGroup::new(*pid, *cpu) }.wrap_ok(),
        }
    }
}
