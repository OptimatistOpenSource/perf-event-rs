use crate::sampling::record::SampleId;
use std::ffi::CString;

mod raw;

#[derive(Debug)]
pub struct Body {
    pub id: u64,
    pub path: CString,
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
            id: *raw.id(),
            path: CString::from_vec_unchecked(raw.path().to_vec()),
            sample_id: raw.sample_id().clone(),
        }
    }
}
