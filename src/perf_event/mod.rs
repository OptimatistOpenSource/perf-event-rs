use crate::infra::result::WrapResult;
use crate::syscall;
use crate::syscall::bindings::{perf_event_attr, perf_event_ioctls};
use crate::syscall::ioctl;
use std::fs::File;
use std::io;
use std::os::fd::{AsRawFd, RawFd};
use thiserror::Error;

pub struct PerfEventAttr {
    inner: perf_event_attr,
}

pub enum PerfEventCount {
    User,
    Kernel,
    Hv,
    Idle,
    Host,
    Guest,
    CallchainKernel,
    CallchainUser,
}

impl PerfEventAttr {
    pub fn new() -> Self {
        let mut attr = perf_event_attr::default();
        attr.size = std::mem::size_of_val(&attr) as libc::__u32;
        attr.set_exclude_user(1);
        attr.set_exclude_kernel(1);
        attr.set_exclude_hv(1);
        attr.set_exclude_idle(1);
        attr.set_exclude_host(1);
        attr.set_exclude_guest(1);
        attr.set_exclude_callchain_kernel(1);
        attr.set_exclude_callchain_user(1);
        Self { inner: attr }
    }
    pub fn include_count(&mut self, count: PerfEventCount) {
        use PerfEventCount::*;

        let inner = &mut self.inner;
        match count {
            User => inner.set_exclude_user(0),
            Kernel => inner.set_exclude_kernel(0),
            Hv => inner.set_exclude_hv(0),
            Idle => inner.set_exclude_idle(0),
            Host => inner.set_exclude_host(0),
            Guest => inner.set_exclude_guest(0),
            CallchainKernel => inner.set_exclude_callchain_kernel(0),
            CallchainUser => inner.set_exclude_callchain_user(0),
        }
    }
    pub fn include_counts(&mut self, includes: impl Iterator<Item = PerfEventCount>) {
        includes.for_each(|include| self.include_count(include));
    }
}

pub struct PerfEvent {
    // TODO
    raw_fd: RawFd,
}

impl PerfEvent {
    fn perf_event_ioctl(&self, op: perf_event_ioctls) -> io::Result<()> {
        let i32 = unsafe { ioctl(self.raw_fd as libc::c_int, op as libc::c_ulong, 0) };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            _ => Ok(()),
        }
    }

    fn perf_event_ioctl_with_arg<A>(&self, op: perf_event_ioctls, arg: A) -> io::Result<()> {
        let i32 = unsafe { ioctl(self.raw_fd as libc::c_int, op as libc::c_ulong, arg) };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            _ => Ok(()),
        }
    }

    pub fn enable(&self) -> io::Result<()> {
        self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_ENABLE)
    }

    pub fn enable_group(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(
            syscall::bindings::perf_event_ioctls_ENABLE,
            syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP,
        )
    }

    pub fn disable(&self) -> io::Result<()> {
        self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_DISABLE)
    }

    pub fn disable_group(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(
            syscall::bindings::perf_event_ioctls_DISABLE,
            syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP,
        )
    }

    pub fn refresh(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_REFRESH)
        todo!()
    }

    pub fn reset_count(&self) -> io::Result<()> {
        self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_RESET)
    }

    pub fn reset_count_group(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(
            syscall::bindings::perf_event_ioctls_RESET,
            syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP,
        )
    }

    pub fn update_period(&self, new: u64) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PERIOD, &new)
    }

    pub fn set_output(&self, new: File) -> io::Result<()> {
        let raw_fd = new.as_raw_fd() as i64;
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_SET_OUTPUT, raw_fd)
    }

    pub fn ignore_output(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_SET_OUTPUT, -1i64)
    }

    pub fn set_filter(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_SET_FILTER)
        todo!()
    }

    pub fn id(&self) -> io::Result<u64> {
        let mut id = 0_u64;
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_ID, &mut id)?;
        Ok(id)
    }

    pub fn set_bpf(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_SET_BPF)
        todo!()
    }

    pub fn pause_output(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PAUSE_OUTPUT, 1i32)
    }

    pub fn resume_output(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PAUSE_OUTPUT, 0i32)
    }

    pub fn query_bpf(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_QUERY_BPF)
        todo!()
    }

    pub fn modify_attributes(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_MODIFY_ATTRIBUTES)
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("This option has been set")]
    AlreadySet,
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
            _ => Err(BuildError::AlreadySet),
        }
    }

    pub fn any_cpu(mut self) -> Result<Self, BuildError> {
        match self.pid {
            None => {
                self.pid = Some(-1);
                Ok(self)
            }
            _ => Err(BuildError::AlreadySet),
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
            _ => BuildError::AlreadySet,
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
            _ => BuildError::AlreadySet,
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
