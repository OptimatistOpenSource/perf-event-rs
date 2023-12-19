use crate::infra::WrapResult;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("PID is invalid: {0}")]
    InvalidPid(String),
    #[error("CPU is invalid: {0}")]
    InvalidCpu(String),
    #[error("PID not set")]
    PidNotSet,
    #[error("CPU not set")]
    CpuNotSet,
    #[error("Ring buffer size not set")]
    RingBufferSizeNotSet,
    #[error("Failed to perform perf_event_open: {0}")]
    SyscallFailed(io::Error),
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
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
    pub(crate) mmap_pages: Option<usize>,

    #[allow(dead_code)]
    pub(crate) flags: Option<u64>, // TODO
}

impl Builder {
    pub const fn new() -> Self {
        Self {
            pid: None,
            cpu: None,
            mmap_pages: None,
            flags: None,
        }
    }

    pub const fn calling_process(mut self) -> Self {
        self.pid = Some(0);
        self
    }

    pub fn on_process(mut self, pid: u32) -> Result<Self, BuildError> {
        match pid {
            0 => BuildError::InvalidPid("PID is 0".to_string()),
            _ if pid > 2_u32.pow(22) => BuildError::InvalidPid(format!("PID {} is too big", pid)),
            _ => {
                self.pid = Some(pid as _);
                return Ok(self);
            }
        }
        .wrap_err()
    }

    pub const fn any_process(mut self) -> Self {
        self.pid = Some(-1);
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

    pub const fn any_cpu(mut self) -> Self {
        self.cpu = Some(-1);
        self
    }

    pub const fn ring_buffer_pages(mut self, pages: usize) -> Self {
        self.mmap_pages = Some(pages);
        self
    }
}
