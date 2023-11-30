mod raw;

use crate::sampling::record::SampleId;
use std::ffi::CString;

#[derive(Debug)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
    pub comm: CString,
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
            comm: CString::from_vec_unchecked(raw.comm().to_vec()),
            sample_id: raw.sample_id().clone(),
        }
    }
}
