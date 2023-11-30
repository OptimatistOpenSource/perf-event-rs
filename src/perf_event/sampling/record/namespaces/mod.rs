use crate::sampling::record::SampleId;

mod raw;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Namespace {
    pub dev: u64,
    pub inode: u64,
}

#[derive(Debug)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
    pub namespaces: Vec<Namespace>,
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
            namespaces: raw.namespaces().to_vec(),
            sample_id: raw.sample_id().clone(),
        }
    }
}
