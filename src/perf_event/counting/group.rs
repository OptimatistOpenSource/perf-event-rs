use crate::counting::{ioctl_wrapped, Attr, Counting};
use crate::syscall;
use libc::pid_t;
use std::collections::HashMap;
use std::io;
use std::io::{ErrorKind, Read};
use std::os::fd::AsRawFd;

#[repr(C)]
pub(crate) struct GroupReadFormatValueFollowed {
    pub event_count: u64, // u64 value;
    pub event_id: u64,    // u64 id;
    #[cfg(feature = "kernel-6.0")]
    pub event_lost: u64, // u64 lost;
}

#[repr(C)]
pub(crate) struct GroupReadFormat {
    pub member_len: u64,   // u64 nr;
    pub time_enabled: u64, // u64 time_enabled;
    pub time_running: u64, // u64 time_running;
                           // follows: struct { .. } values[nr];
}

pub struct GroupCountingMemberResult {
    pub event_count: u64,
    pub event_lost: u64,
}

pub struct GroupCountingResult {
    pub time_enabled: u64,
    pub time_running: u64,
    pub member_results: HashMap<u64, GroupCountingMemberResult>,
}

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

    pub fn add_member(mut self, attr: Attr) -> io::Result<CountingGroup> {
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
