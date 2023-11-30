use crate::sampling::{Attr, Sampling};
use crate::syscall;
use crate::syscall::ioctl_wrapped;
use libc::pid_t;
use std::io;
use std::io::ErrorKind;
use std::os::fd::AsRawFd;

pub struct SamplingGroup {
    pid: pid_t,
    cpu: i32,
    members: Vec<Sampling>, // members[0] is the group leader, if it exists.
}

impl SamplingGroup {
    pub(crate) const unsafe fn new(pid: pid_t, cpu: i32) -> Self {
        Self {
            pid,
            cpu,
            members: vec![],
        }
    }

    fn leader(&self) -> Option<&Sampling> {
        self.members.get(0)
    }

    fn leader_mut(&mut self) -> Option<&mut Sampling> {
        self.members.get_mut(0)
    }

    pub fn add_member(&mut self, attr: &Attr) -> io::Result<u64> {
        let member = match self.leader() {
            None => unsafe { Sampling::new(attr, self.pid, self.cpu, -1, 0) },
            Some(leader) => {
                let group_fd = leader.file.as_raw_fd();
                unsafe { Sampling::new(attr, self.pid, self.cpu, group_fd, 0) }
            }
        }?;
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
                    syscall::bindings::perf_event_ioctls_ENABLE,
                    Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
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
                    syscall::bindings::perf_event_ioctls_DISABLE,
                    Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
                )
            },
        )
    }
}
