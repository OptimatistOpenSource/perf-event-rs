use crate::counting::CountingGroupResult;
use crate::sampling::record::SampleId;

mod raw;

#[derive(Debug, Clone)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
    pub values: CountingGroupResult,
    pub sample_id: Option<SampleId>,
}

type RawBody = raw::Body;

impl Body {
    pub unsafe fn from_ptr(ptr: *const u8, sample_id_all: bool) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();

        Self {
            pid: *raw.pid(),
            tid: *raw.tid(),
            values: CountingGroupResult::from_raw(raw.values_header(), raw.values_body()),
            sample_id: sample_id_all.then(|| raw.sample_id().clone()),
        }
    }
}
