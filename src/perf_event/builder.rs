use crate::infra::result::WrapResult;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("PID is invalid: {0}")]
    InvalidPid(String),
    #[error("CPU is invalid: {0}")]
    InvalidCpu(String),
    #[error("PID and CPU are not set")]
    PidAndCpuNotSet,
    #[error("Failed to perform perf_event_open: {0}")]
    SyscallFailed(io::Error),
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Builder {
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
    pub(crate) pid: Option<i32>,
    pub(crate) cpu: Option<i32>,

    pub(crate) group_fd: Option<i32>, // TODO
    pub(crate) flags: Option<u64>,    // TODO
}

impl Builder {
    pub fn new() -> Self {
        Self {
            pid: None,
            cpu: None,
            group_fd: None,
            flags: None,
        }
    }

    pub fn calling_process(mut self) -> Self {
        self.pid = Some(0);
        self
    }

    pub fn on_process(mut self, pid: u32) -> Result<Self, BuildError> {
        match pid {
            0 => BuildError::InvalidPid("PID is 0".to_string()),
            _ if pid > 2 ^ 22 => BuildError::InvalidPid(format!("PID {} is too big", pid)),
            _ => {
                self.pid = Some(pid as i32);
                return Ok(self);
            }
        }
        .wrap_err()
    }

    pub fn any_process(mut self) -> Self {
        self.pid = Some(0);
        self
    }

    pub fn on_cpu(mut self, cpu: u32) -> Result<Self, BuildError> {
        match cpu {
            _ if cpu > i32::MAX as u32 => BuildError::InvalidCpu(format!("CPU {} is too big", cpu)),
            _ => {
                self.cpu = Some(cpu as i32);
                return Ok(self);
            }
        }
        .wrap_err()
    }

    pub fn any_cpu(mut self) -> Self {
        self.cpu = Some(-1);
        self
    }
}
