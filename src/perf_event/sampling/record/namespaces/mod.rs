use crate::sampling::record::sample_id::SampleId;

mod raw;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Namespace {
    pub dev: u64,
    pub inode: u64,
}

#[derive(Debug, Clone)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
    pub namespaces: Vec<Namespace>,
    pub sample_id: Option<SampleId>,
}

type RawBody = raw::Body;

impl Body {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_type: u64, sample_id_all: bool) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();

        Self {
            pid: *raw.pid(),
            tid: *raw.tid(),
            namespaces: raw.namespaces().to_vec(),
            sample_id: sample_id_all.then(|| raw.sample_id(sample_type).clone()),
        }
    }
}
