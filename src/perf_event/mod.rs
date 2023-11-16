use crate::infra::result::WrapResult;
use crate::syscall::bindings::perf_event_attr;
use thiserror::Error;

pub struct PerfEvent {
    // TODO
}

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("This option has been set")]
    DuplicateSet,
    #[error("PID is invalid: {0}")]
    InvalidPid(String),
    #[error("CPU is invalid: {0}")]
    InvalidCpu(String),
    #[error("PID and CPU are not set")]
    PidAndCpuNotSet,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Builder {
    attr: perf_event_attr,

    /*
    pid == 0 and cpu == -1
           This measures the calling process/thread on any CPU.

    pid == 0 and cpu >= 0
           This measures the calling process/thread only when running
           on the specified CPU.

    pid > 0 and cpu == -1
           This measures the specified process/thread on any CPU.

    pid > 0 and cpu >= 0
           This measures the specified process/thread only when
           running on the specified CPU.

    pid == -1 and cpu >= 0
           This measures all processes/threads on the specified CPU.
           This requires CAP_PERFMON (since Linux 5.8) or
           CAP_SYS_ADMIN capability or a
           /proc/sys/kernel/perf_event_paranoid value of less than 1.

    pid == -1 and cpu == -1
           This setting is invalid and will return an error.
    */
    pid: Option<i32>,
    cpu: Option<i32>,

    group_fd: Option<i32>,

    flags: Option<u64>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            attr: Default::default(),
            pid: None,
            cpu: None,
            group_fd: None,
            flags: None,
        }
    }

    pub fn current_pid(mut self) -> Result<Self, BuildError> {
        match self.pid {
            None => {
                self.pid = Some(0);
                Ok(self)
            }
            _ => Err(BuildError::DuplicateSet),
        }
    }

    pub fn any_cpu(mut self) -> Result<Self, BuildError> {
        match self.pid {
            None => {
                self.pid = Some(-1);
                Ok(self)
            }
            _ => Err(BuildError::DuplicateSet),
        }
    }

    pub fn with_pid(mut self, pid: u32) -> Result<Self, BuildError> {
        match self.pid {
            None => match pid {
                0 => BuildError::InvalidPid("PID is 0".to_string()),
                _ if pid > 2 ^ 22 => BuildError::InvalidPid(format!("PID {} is too big", pid)),
                _ => {
                    self.pid = Some(pid as i32);
                    return Ok(self);
                }
            },
            _ => BuildError::DuplicateSet,
        }
        .wrap_err()
    }

    pub fn with_cpu(mut self, cpu: u32) -> Result<Self, BuildError> {
        match self.cpu {
            None => match cpu {
                _ if cpu > i32::MAX as u32 => {
                    BuildError::InvalidCpu(format!("CPU {} is too big", cpu))
                }
                _ => {
                    self.cpu = Some(cpu as i32);
                    return Ok(self);
                }
            },
            _ => BuildError::DuplicateSet,
        }
        .wrap_err()
    }

    pub fn build(self) -> Result<PerfEvent, BuildError> {
        match self {
            Builder {
                pid: None,
                cpu: None,
                ..
            } => Err(BuildError::PidAndCpuNotSet),
            _ => todo!(),
        }
    }
}
