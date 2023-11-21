use crate::counting::{ioctl_wrapped, Attr, Counting};
use crate::infra::result::WrapResult;
use crate::syscall;
use libc::pid_t;
use std::collections::HashMap;
use std::io;
use std::io::{ErrorKind, Read};
use std::os::fd::AsRawFd;
use std::os::unix::fs::FileExt;

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

    fn leader_mut(&mut self) -> Option<&mut Counting> {
        self.members.get_mut(0)
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

    pub fn get_result(&mut self) -> io::Result<GroupCountingResult> {
        let Some(leader) = self.leader_mut() else {
            return Err(io::Error::new(ErrorKind::Other, "Group has no members"));
        };

        use std::mem::size_of;

        let header = {
            let mut buf = [0_u8; size_of::<GroupReadFormat>()];
            leader.file.read_exact(&mut buf)?;
            let ptr = buf.as_ptr() as *const GroupReadFormat;
            unsafe { ptr.read() }
        };

        let values = {
            let mut buf =
                vec![0_u8; size_of::<GroupReadFormatValueFollowed>() * header.member_len as usize];
            leader
                .file
                .read_exact_at(&mut buf, size_of::<GroupReadFormat>() as u64)?;

            let values_vec = unsafe {
                Vec::from_raw_parts(
                    buf.as_mut_ptr() as *mut GroupReadFormatValueFollowed,
                    header.member_len as usize,
                    header.member_len as usize,
                )
            };

            values_vec
                .into_iter()
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
