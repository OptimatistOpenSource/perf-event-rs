use crate::perf_event::RawAttr;
use crate::sampling::group::stat::inner_stat;
use crate::sampling::record::Record;
use crate::sampling::{Sampler, SamplerGroupStat};
use crate::syscall::bindings::*;
use crate::syscall::{ioctl_wrapped, perf_event_open_wrapped};
use libc::pid_t;
use memmap2::MmapOptions;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::os::fd::{AsRawFd, FromRawFd};

pub struct Inner {
    leader_event_id: Option<u64>,
    pub(crate) members: HashMap<u64, Sampler>, // members[0] is the group leader, if it exists.
}

impl Inner {
    pub(crate) fn new() -> Self {
        Self {
            leader_event_id: None,
            members: HashMap::new(),
        }
    }

    pub(crate) fn leader(&self) -> Option<&Sampler> {
        self.leader_event_id.and_then(|id| self.members.get(&id))
    }

    #[allow(dead_code)]
    pub(crate) fn leader_mut(&mut self) -> Option<&mut Sampler> {
        self.leader_event_id
            .and_then(|id| self.members.get_mut(&id))
    }

    pub fn add_member(
        &mut self,
        pid: pid_t,
        cpu: i32,
        mmap_pages: usize,
        raw_attr: &RawAttr,
    ) -> io::Result<u64> {
        let group_fd = self.leader().map(|it| it.file.as_raw_fd()).unwrap_or(-1);
        let fd = unsafe { perf_event_open_wrapped(raw_attr, pid, cpu, group_fd, 0) }?;
        let file = unsafe { File::from_raw_fd(fd) };
        let mmap = unsafe {
            MmapOptions::new()
                .len(page_size::get() * mmap_pages)
                .map_mut(&file)
        }
        .unwrap();

        let page_size = page_size::get();

        let member = Sampler {
            mmap,
            file,
            data_size: ((mmap_pages - 1) * page_size) as _,
            data_offset: page_size as _,
            sample_type: raw_attr.sample_type,
            sample_id_all: raw_attr.sample_id_all() > 0,
            regs_user_len: raw_attr.sample_regs_user.count_ones() as _,
            #[cfg(feature = "linux-3.19")]
            regs_intr_len: raw_attr.sample_regs_intr.count_ones() as _,
        };

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

    pub fn reset(&self) -> io::Result<()> {
        self.leader().map_or_else(
            || Err(io::Error::new(ErrorKind::Other, "Group has no members")),
            |leader| {
                ioctl_wrapped(
                    &leader.file,
                    PERF_EVENT_IOCTL_RESET,
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

    pub fn stat(&mut self) -> io::Result<SamplerGroupStat> {
        inner_stat(self)
    }
}
