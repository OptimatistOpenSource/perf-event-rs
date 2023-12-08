use crate::sampling::record::Record;
use crate::sampling::{Attr, Sampling};
use crate::syscall::bindings::*;
use crate::syscall::ioctl_wrapped;
use libc::pid_t;
use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;
use std::os::fd::AsRawFd;

pub struct Inner {
    leader_event_id: Option<u64>,
    members: HashMap<u64, Sampling>, // members[0] is the group leader, if it exists.
}

impl Inner {
    pub(crate) fn new() -> Self {
        Self {
            leader_event_id: None,
            members: HashMap::new(),
        }
    }

    fn leader(&self) -> Option<&Sampling> {
        self.leader_event_id.and_then(|id| self.members.get(&id))
    }

    #[allow(dead_code)]
    fn leader_mut(&mut self) -> Option<&mut Sampling> {
        self.leader_event_id
            .and_then(|id| self.members.get_mut(&id))
    }

    pub fn add_member(
        &mut self,
        pid: pid_t,
        cpu: i32,
        attr: &Attr,
        mmap_pages: usize,
    ) -> io::Result<u64> {
        let member = self.leader().map_or_else(
            || unsafe { Sampling::new(attr, pid, cpu, -1, 0, mmap_pages) },
            |leader| {
                let group_fd = leader.file.as_raw_fd();
                unsafe { Sampling::new(attr, pid, cpu, group_fd, 0, mmap_pages) }
            },
        )?;

        let event_id = member.event_id()?;
        if self.leader_event_id.is_none() {
            self.leader_event_id = Some(event_id);
        }
        self.members.insert(event_id, member);

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

    pub fn next_record(&mut self, event_id: u64) -> Option<Record> {
        self.members
            .get_mut(&event_id)
            .and_then(|member| member.next_record())
    }
}
