use crate::sampling::record::sample_id::SampleId;
use std::ffi::CString;

mod raw;

#[derive(Debug, Clone)]
pub struct Body {
    pub addr: u64,
    pub len: u32,
    pub ksym_type: u16,
    pub flags: u16,
    pub name: CString,
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
            addr: sized.addr,
            len: sized.len,
            ksym_type: sized.ksym_type,
            flags: sized.flags,
            name: CString::from_vec_unchecked(raw.name().to_vec()),
            sample_id: sample_id_all.then(|| raw.sample_id()),
        }
    }
}
