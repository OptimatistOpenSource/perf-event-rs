use std::ffi::CString;

mod raw;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
    pub addr: u64,
    pub len: u64,
    pub pgoff: u64,
    pub filename: CString,
}

impl Body {
    pub(crate) unsafe fn from_ptr(ptr: *const u8) -> Self {
        let raw = (ptr as *const raw::Raw).as_ref().unwrap();

        Self {
            pid: raw.pid,
            tid: raw.tid,
            addr: raw.addr,
            len: raw.len,
            pgoff: raw.pgoff,
            filename: CString::from_vec_unchecked(raw.filename.as_slice().to_vec()),
        }
    }
}
