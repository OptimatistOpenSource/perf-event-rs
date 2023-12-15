use crate::sampling::record::sample_id::SampleId;
use std::ffi::CString;

mod raw;

#[derive(Debug, Clone)]
pub struct Body {
    pub id: u64,
    pub path: CString,
    pub sample_id: Option<SampleId>,
}

type RawBody = raw::Body;

impl Body {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_type: u64, sample_id_all: bool) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();

        Self {
            id: *raw.id(),
            path: CString::from_vec_unchecked(raw.path().to_vec()),
            sample_id: sample_id_all.then(|| raw.sample_id(sample_type)),
        }
    }
}
