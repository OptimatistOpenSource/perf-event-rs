use crate::sampling::record::SampleId;

mod raw;

#[derive(Debug, Clone)]
pub struct Body {
    pub next_prev_pid: u32,
    pub next_prev_tid: u32,
    pub sample_id: Option<SampleId>,
}

type RawBody = raw::Body;

impl Body {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_id_all: bool) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();

        Self {
            next_prev_pid: raw.next_prev_pid,
            next_prev_tid: raw.next_prev_tid,
            sample_id: sample_id_all.then(|| raw.sample_id().clone()),
        }
    }
}
