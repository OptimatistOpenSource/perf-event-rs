use crate::counting::{CountingGroup, CountingGroupResult};
use crate::syscall;
use crate::syscall::ioctl_wrapped;
use std::io;
use std::io::ErrorKind;

pub struct FixedCountingGroup {
    group: CountingGroup,
}

impl FixedCountingGroup {
    pub(crate) const fn new(group: CountingGroup) -> Self {
        Self { group }
    }

    pub fn result(&mut self) -> io::Result<CountingGroupResult> {
        self.group.result()
    }

    pub fn enable(&self) -> io::Result<()> {
        self.group.leader().map_or_else(
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
        self.group.leader().map_or_else(
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

    pub fn reset_count(&self) -> io::Result<()> {
        self.group.leader().map_or_else(
            || Err(io::Error::new(ErrorKind::Other, "Group has no members")),
            |leader| {
                ioctl_wrapped(
                    &leader.file,
                    syscall::bindings::perf_event_ioctls_RESET,
                    Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
                )
            },
        )
    }
}
