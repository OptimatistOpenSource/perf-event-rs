use crate::sampling::record::sample_id::SampleId;

mod raw;

#[derive(Debug, Clone)]
pub struct Body {
    pub pid: u32,
    pub ppid: u32,
    pub tid: u32,
    pub ptid: u32,
    pub time: u64,
    pub sample_id: Option<SampleId>,
}

impl Body {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_type: u64, sample_id_all: bool) -> Self {
        let raw = &*(ptr as *const raw::Raw);
        Self {
            pid: raw.pid,
            ppid: raw.ppid,
            tid: raw.tid,
            ptid: raw.ptid,
            time: raw.time,
            sample_id: sample_id_all.then(|| raw.sample_id(sample_type)),
        }
    }
}
