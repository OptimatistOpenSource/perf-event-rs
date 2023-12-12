use crate::sampling::record::SampleId;
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

type RawBody = raw::Body;

impl Body {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_id_all: bool) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();

        Self {
            addr: *raw.addr(),
            len: *raw.len(),
            ksym_type: *raw.ksym_type(),
            flags: *raw.flags(),
            name: CString::from_vec_unchecked(raw.name().to_vec()),
            sample_id: sample_id_all.then(|| raw.sample_id().clone()),
        }
    }
}
