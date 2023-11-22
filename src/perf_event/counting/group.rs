use crate::counting::{ioctl_wrapped, Attr, Counting};
use crate::infra::result::WrapResult;
use crate::infra::vec::VecExt;
use crate::syscall;
use libc::pid_t;
use std::collections::HashMap;
use std::io::{ErrorKind, Read};
use std::os::fd::AsRawFd;
use std::{io, ptr};

#[repr(C)]
#[derive(Debug)]
pub(crate) struct GroupReadFormatValueFollowed {
    pub event_count: u64, // u64 value;
    pub event_id: u64,    // u64 id;
    #[cfg(feature = "kernel-6.0")]
    pub event_lost: u64, // u64 lost;
}

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct GroupReadFormat {
    pub members_len: u64,  // u64 nr;
    pub time_enabled: u64, // u64 time_enabled;
    pub time_running: u64, // u64 time_running;
                           // follows: struct { .. } values[nr];
}

#[derive(Debug)]
pub struct GroupCountingMemberResult {
    pub event_count: u64,
    pub event_lost: u64,
}

#[derive(Debug)]
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

    fn leader_mut(&mut self) -> Option<&mut Counting> {
        self.members.get_mut(0)
    }

    pub fn add_member(&mut self, attr: Attr) -> io::Result<u64> {
        let member = match self.leader() {
            None => unsafe { Counting::new(attr, self.pid, self.cpu, -1, 0) },
            Some(leader) => {
                let group_fd = leader.file.as_raw_fd();
                unsafe { Counting::new(attr, self.pid, self.cpu, group_fd, 0) }
            }
        }?;
        let event_id = member.get_event_id()?;
        self.members.push(member);

        Ok(event_id)
    }

    pub fn get_result(&mut self) -> io::Result<GroupCountingResult> {
        let members_len = self.members.len();

        let Some(leader) = self.leader_mut() else {
            return Err(io::Error::new(ErrorKind::Other, "Group has no members"));
        };

        use std::mem::size_of;

        let buf = {
            let len = size_of::<GroupReadFormat>()
                + size_of::<GroupReadFormatValueFollowed>() * members_len;

            let mut buf = unsafe { Vec::<u8>::with_len_uninit(len) };
            leader.file.read_exact(&mut buf)?;

            buf
        };

        let header = {
            let ptr = buf.as_ptr() as *const GroupReadFormat;
            unsafe { ptr.read() }.clone() // clone to stack
        };

        let values = {
            let header_ptr = buf.as_ptr() as *const GroupReadFormat;
            let values_ptr = unsafe { header_ptr.offset(1) as *const GroupReadFormatValueFollowed };
            let values = unsafe { &*ptr::slice_from_raw_parts(values_ptr, members_len) };

            values
                .iter()
                .map(|it| {
                    (
                        it.event_id,
                        GroupCountingMemberResult {
                            event_count: it.event_count,
                            event_lost: it.event_lost,
                        },
                    )
                })
                .collect::<HashMap<_, _>>()
        };

        GroupCountingResult {
            time_enabled: header.time_enabled,
            time_running: header.time_running,
            member_results: values,
        }
        .wrap_ok()
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
