use crate::sampling::record::sample_id::SampleId;

mod raw;

#[derive(Debug, Clone)]
pub struct Body {
    pub next_prev_pid: u32,
    pub next_prev_tid: u32,
    pub sample_id: Option<SampleId>,
}

impl Body {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_type: u64, sample_id_all: bool) -> Self {
        let raw = &*(ptr as *const raw::Raw);

        Self {
            next_prev_pid: raw.next_prev_pid,
            next_prev_tid: raw.next_prev_tid,
            sample_id: sample_id_all.then(|| raw.sample_id(sample_type)),
        }
    }
}
