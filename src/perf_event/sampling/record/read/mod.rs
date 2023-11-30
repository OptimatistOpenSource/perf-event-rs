use crate::counting::GroupCountingResult;
use crate::sampling::record::SampleId;

mod raw;

#[derive(Debug)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
    pub values: GroupCountingResult,
    pub sample_id: SampleId,
}

type RawBody = raw::Body;

impl Body {
    pub unsafe fn from_ptr(ptr: *const u8) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();
        Self::from_raw(raw)
    }

    unsafe fn from_raw(raw: &RawBody) -> Self {
        Self {
            pid: *raw.pid(),
            tid: *raw.tid(),
            values: GroupCountingResult::from_raw(raw.values_header(), raw.values_body()),
            sample_id: raw.sample_id().clone(),
        }
    }
}
