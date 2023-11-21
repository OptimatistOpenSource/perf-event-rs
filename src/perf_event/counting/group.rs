use crate::counting::{ioctl_wrapped, Counting};
use crate::syscall;
use std::io;

pub struct CountingGroup {
    members: Vec<Counting>, // members[0] is the group leader, if it exists.
}

impl CountingGroup {
    pub fn enable(&self) -> io::Result<()> {
        let Some(leader) = self.members.get(0) else {
            todo!()
        };
        ioctl_wrapped(
            &leader.file,
            syscall::bindings::perf_event_ioctls_ENABLE,
            Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
        )
    }

    pub fn disable(&self) -> io::Result<()> {
        let Some(leader) = self.members.get(0) else {
            todo!()
        };
        ioctl_wrapped(
            &leader.file,
            syscall::bindings::perf_event_ioctls_DISABLE,
            Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
        )
    }

    pub fn reset_count(&self) -> io::Result<()> {
        let Some(leader) = self.members.get(0) else {
            todo!()
        };
        ioctl_wrapped(
            &leader.file,
            syscall::bindings::perf_event_ioctls_RESET,
            Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
        )
    }
}
