use crate::counting::{ioctl_wrapped, Attr, Counting};
use crate::syscall;
use libc::pid_t;
use std::io;
use std::io::ErrorKind;
use std::os::fd::AsRawFd;

pub struct CountingGroup {
    pid: pid_t,
    cpu: i32,
    members: Vec<Counting>, // members[0] is the group leader, if it exists.
}

impl CountingGroup {
    pub(crate) unsafe fn new(pid: pid_t, cpu: i32) -> CountingGroup {
        Self {
            pid,
            cpu,
            members: vec![],
        }
    }

    fn leader(&self) -> Option<&Counting> {
        self.members.get(0)
    }

    pub fn add_member<F>(mut self, attr: Attr) -> io::Result<CountingGroup> {
        match self.leader() {
            None => {
                let leader = unsafe { Counting::new(attr, self.pid, self.cpu, -1, 0) }?;
                self.members.push(leader);
            }
            Some(leader) => {
                let group_fd = leader.file.as_raw_fd();
                let member = unsafe { Counting::new(attr, self.pid, self.cpu, group_fd, 0) }?;
                self.members.push(member);
            }
        };

        Ok(self)
    }

    pub fn enable(&self) -> io::Result<()> {
        if let Some(leader) = self.leader() {
            ioctl_wrapped(
                &leader.file,
                syscall::bindings::perf_event_ioctls_ENABLE,
                Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
            )
        } else {
            Err(io::Error::new(ErrorKind::Other, "Group has no members"))
        }
    }

    pub fn disable(&self) -> io::Result<()> {
        if let Some(leader) = self.leader() {
            ioctl_wrapped(
                &leader.file,
                syscall::bindings::perf_event_ioctls_DISABLE,
                Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
            )
        } else {
            Err(io::Error::new(ErrorKind::Other, "Group has no members"))
        }
    }

    pub fn reset_count(&self) -> io::Result<()> {
        if let Some(leader) = self.leader() {
            ioctl_wrapped(
                &leader.file,
                syscall::bindings::perf_event_ioctls_RESET,
                Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
            )
        } else {
            Err(io::Error::new(ErrorKind::Other, "Group has no members"))
        }
    }
}
