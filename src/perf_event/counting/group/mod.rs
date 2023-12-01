mod fixed;
mod guard;
mod result;

use crate::counting::group::guard::CountingGuard;
use crate::counting::{Attr, Counting};
use crate::infra::WrapResult;
use crate::infra::{VecExt, WrapOption};
use crate::syscall;
use crate::syscall::bindings::{read_format_body, read_format_header};
use crate::syscall::ioctl_wrapped;
use libc::pid_t;
pub use result::*;
use std::io::{ErrorKind, Read};
use std::os::fd::AsRawFd;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{io, slice};

use crate::counting::group::fixed::FixedCountingGroup;
pub use guard::*;
pub use result::*;

pub struct CountingGroup {
    pid: pid_t,
    cpu: i32,
    leader: Option<Arc<RwLock<Counting>>>,
    pub(crate) members_len: usize,
}

impl CountingGroup {
    pub(crate) const unsafe fn new(pid: pid_t, cpu: i32) -> Self {
        Self {
            pid,
            cpu,
            leader: None,
            members_len: 0,
        }
    }

    fn leader(&self) -> Option<RwLockReadGuard<'_, Counting>> {
        self.leader.as_ref().map(|rw| rw.read().unwrap())
    }

    fn leader_mut(&self) -> Option<RwLockWriteGuard<'_, Counting>> {
        self.leader.as_ref().map(|rw| rw.write().unwrap())
    }

    pub fn add_member(&mut self, attr: &Attr) -> io::Result<CountingGuard> {
        let member = match &self.leader {
            None => {
                let counting = Arc::new(RwLock::new(unsafe {
                    Counting::new(attr, self.pid, self.cpu, -1, 0)
                }?));
                self.leader = counting.clone().wrap_some();
                counting
            }
            Some(leader) => {
                let group_fd = leader.read().unwrap().file.as_raw_fd();
                Arc::new(RwLock::new(unsafe {
                    Counting::new(attr, self.pid, self.cpu, group_fd, 0)
                }?))
            }
        };

        self.members_len += 1;

        CountingGuard::new(member).wrap_ok()
    }

    pub fn result(&mut self) -> io::Result<CountingGroupResult> {
        let Some(mut leader) = self.leader_mut() else {
            return Err(io::Error::new(ErrorKind::Other, "Group has no members"));
        };

        use std::mem::size_of;

        let buf = {
            let len =
                size_of::<read_format_header>() + size_of::<read_format_body>() * self.members_len;

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
            unsafe { slice::from_raw_parts(values_ptr, self.members_len) }
        };

        CountingGroupResult::from_raw(header, body).wrap_ok()
    }

    pub fn enable(self) -> io::Result<FixedCountingGroup> {
        self.leader().map_or_else(
            || Err(io::Error::new(ErrorKind::Other, "Group has no members")),
            |leader| {
                ioctl_wrapped(
                    &leader.file,
                    syscall::bindings::perf_event_ioctls_ENABLE,
                    Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
                )
            },
        )?;

        FixedCountingGroup::new(self).wrap_ok()
    }
}
