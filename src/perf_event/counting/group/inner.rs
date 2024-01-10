use crate::counting::{Config, Counter, CounterGroupResult};
use crate::infra::VecExt;
use crate::infra::WrapResult;
use crate::syscall;
use crate::syscall::bindings::*;
use crate::syscall::ioctl_wrapped;
use libc::pid_t;
use std::io::{ErrorKind, Read};
use std::os::fd::AsRawFd;
use std::{io, slice};

pub struct Inner {
    members: Vec<Counter>, // members[0] is the group leader, if it exists.
}

impl Inner {
    pub(crate) const fn new() -> Self {
        Self { members: vec![] }
    }

    fn leader(&self) -> Option<&Counter> {
        self.members.first()
    }

    fn leader_mut(&mut self) -> Option<&mut Counter> {
        self.members.get_mut(0)
    }

    pub fn add_member(&mut self, pid: pid_t, cpu: i32, cfg: &Config) -> io::Result<u64> {
        let member = self.leader().map_or_else(
            || unsafe { Counter::new(cfg, pid, cpu, -1, 0) },
            |leader| {
                let group_fd = leader.file.as_raw_fd();
                unsafe { Counter::new(cfg, pid, cpu, group_fd, 0) }
            },
        )?;

        let event_id = member.event_id();
        self.members.push(member);

        event_id
    }

    pub fn enable(&self) -> io::Result<()> {
        self.leader().map_or_else(
            || Err(io::Error::new(ErrorKind::Other, "Group has no members")),
            |leader| {
                ioctl_wrapped(
                    &leader.file,
                    syscall::bindings::PERF_EVENT_IOCTL_ENABLE,
                    Some(syscall::bindings::PERF_IOC_FLAG_GROUP),
                )
            },
        )
    }

    pub fn disable(&self) -> io::Result<()> {
        self.leader().map_or_else(
            || Err(io::Error::new(ErrorKind::Other, "Group has no members")),
            |leader| {
                ioctl_wrapped(
                    &leader.file,
                    syscall::bindings::PERF_EVENT_IOCTL_DISABLE,
                    Some(syscall::bindings::PERF_IOC_FLAG_GROUP),
                )
            },
        )
    }

    pub fn reset_count(&self) -> io::Result<()> {
        self.leader().map_or_else(
            || Err(io::Error::new(ErrorKind::Other, "Group has no members")),
            |leader| {
                ioctl_wrapped(
                    &leader.file,
                    PERF_EVENT_IOCTL_RESET,
                    Some(PERF_IOC_FLAG_GROUP),
                )
            },
        )
    }

    pub fn result(&mut self) -> io::Result<CounterGroupResult> {
        let members_len = self.members.len();
        let Some(leader) = self.leader_mut() else {
            return Err(io::Error::new(ErrorKind::Other, "Group has no members"));
        };

        use std::mem::size_of;

        let buf = {
            let len = size_of::<read_format_header>() + size_of::<read_format_body>() * members_len;

            let mut buf = unsafe { Vec::<u8>::with_len_uninit(len) };
            leader.file.read_exact(&mut buf)?;

            buf
        };

        let header = unsafe { &*(buf.as_ptr() as *const read_format_header) };
        let body = {
            let header_ptr = header as *const read_format_header;
            let values_ptr = unsafe { header_ptr.add(1) as *const read_format_body };
            unsafe { slice::from_raw_parts(values_ptr, self.members.len()) }
        };

        CounterGroupResult::from_raw(header, body).wrap_ok()
    }
}
