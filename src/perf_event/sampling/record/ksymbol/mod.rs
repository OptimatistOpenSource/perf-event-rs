use crate::sampling::record::sample_id;
use std::ffi::CString;

mod raw;

#[derive(Debug)]
pub struct Body {
    pub addr: u64,
    pub len: u32,
    pub ksym_type: u16,
    pub flags: u16,
    pub name: CString,
    pub sample_id: sample_id,
}

type RawBody = raw::Body;

impl Body {
    pub unsafe fn from_ptr(ptr: *const u8) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();
        Self::from_raw(raw)
    }

    unsafe fn from_raw(raw: &RawBody) -> Self {
        Self {
            addr: *raw.addr(),
            len: *raw.len(),
            ksym_type: *raw.ksym_type(),
            flags: *raw.flags(),
            name: CString::from_vec_unchecked(raw.name().as_slice().to_vec()),
            sample_id: raw.sample_id().clone(),
        }
    }
}
