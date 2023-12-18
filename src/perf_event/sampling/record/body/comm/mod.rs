mod raw;

use crate::sampling::record::sample_id::SampleId;
use std::ffi::CString;

#[derive(Debug, Clone)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
    pub comm: CString,
    pub sample_id: Option<SampleId>,
}

impl Body {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_type: u64, sample_id_all: bool) -> Self {
        let mut raw = raw::Raw {
            read_ptr: ptr,
            sample_type,
        };

        let sized = raw.sized();
        Self {
            pid: sized.pid,
            tid: sized.tid,
            comm: CString::from_vec_unchecked(raw.comm().to_vec()),
            sample_id: sample_id_all.then(|| raw.sample_id()),
        }
    }
}
