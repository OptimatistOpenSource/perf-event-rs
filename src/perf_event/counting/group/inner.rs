use crate::counting::{inner_stat, Counter, CounterGroupStat};
use crate::perf_event::RawAttr;
use crate::syscall::bindings::*;
use crate::syscall::{ioctl_wrapped, perf_event_open_wrapped};
use libc::pid_t;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::os::fd::{AsRawFd, FromRawFd};

pub struct Inner {
    pub(crate) members: Vec<Counter>, // members[0] is the group leader, if it exists.
}

impl Inner {
    pub(crate) const fn new() -> Self {
        Self { members: vec![] }
    }

    pub(crate) fn leader(&self) -> Option<&Counter> {
        self.members.first()
    }

    pub(crate) fn leader_mut(&mut self) -> Option<&mut Counter> {
        self.members.get_mut(0)
    }

    pub fn add_member(&mut self, pid: pid_t, cpu: i32, raw_attr: &RawAttr) -> io::Result<u64> {
        let group_fd = self.leader().map(|it| it.file.as_raw_fd()).unwrap_or(-1);
        let fd = unsafe { perf_event_open_wrapped(raw_attr, pid, cpu, group_fd, 0) }?;
        let member = Counter {
            file: unsafe { File::from_raw_fd(fd) },
        };

        let event_id = member.event_id()?;
        self.members.push(member);

        Ok(event_id)
    }

    pub fn enable(&self) -> io::Result<()> {
        self.leader().map_or_else(
            || Err(io::Error::new(ErrorKind::Other, "Group has no members")),
            |leader| {
                ioctl_wrapped(
                    &leader.file,
                    PERF_EVENT_IOCTL_ENABLE,
                    Some(PERF_IOC_FLAG_GROUP),
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
                    PERF_EVENT_IOCTL_DISABLE,
                    Some(PERF_IOC_FLAG_GROUP),
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

    pub fn stat(&mut self) -> io::Result<CounterGroupStat> {
        inner_stat(self)
    }
}
