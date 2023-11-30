mod guard;
mod result;

use crate::counting::group::guard::CountingGuard;
use crate::counting::{Attr, Counting};
use crate::infra::VecExt;
use crate::infra::WrapResult;
use crate::syscall;
use crate::syscall::bindings::{read_format_body, read_format_header};
use crate::syscall::ioctl_wrapped;
use libc::pid_t;
pub use result::*;
use std::io::{ErrorKind, Read};
use std::os::fd::AsRawFd;
use std::{io, slice};

pub struct CountingGroup {
    pid: pid_t,
    cpu: i32,
    members: Vec<Counting>, // members[0] is the group leader, if it exists.
}

impl CountingGroup {
    pub(crate) const unsafe fn new(pid: pid_t, cpu: i32) -> Self {
        Self {
            pid,
            cpu,
            members: vec![],
        }
    }

    fn leader(&self) -> Option<&Counting> {
        self.members.get(0)
    }

    fn leader_mut(&mut self) -> Option<&mut Counting> {
        self.members.get_mut(0)
    }

    pub fn add_member(&mut self, attr: &Attr) -> io::Result<CountingGuard> {
        let member = match self.leader() {
            None => unsafe { Counting::new(attr, self.pid, self.cpu, -1, 0) },
            Some(leader) => {
                let group_fd = leader.file.as_raw_fd();
                unsafe { Counting::new(attr, self.pid, self.cpu, group_fd, 0) }
            }
        }?;

        let event_id = member.event_id()?;
        self.members.push(member);

        CountingGuard::new(event_id).wrap_ok()
    }

    pub fn result(&mut self) -> io::Result<CountingGroupResult> {
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

        let header = {
            let ptr = buf.as_ptr() as *const read_format_header;
            unsafe { ptr.as_ref().unwrap() }
        };

        let body = {
            let header_ptr = header as *const read_format_header;
            let values_ptr = unsafe { header_ptr.add(1) as *const read_format_body };
            unsafe { slice::from_raw_parts(values_ptr, members_len) }
        };

        CountingGroupResult::from_raw(header, body).wrap_ok()
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

    pub fn reset_count(&self) -> io::Result<()> {
        self.leader().map_or_else(
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
