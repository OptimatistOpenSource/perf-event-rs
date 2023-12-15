use crate::sampling::record::sample_id::SampleId;

mod raw;

#[derive(Debug, Clone)]
pub struct Body {
    pub addr: u64,
    pub old_len: u16,
    pub new_len: u16,
    pub bytes: Vec<u8>,
    pub sample_id: Option<SampleId>,
}

type RawBody = raw::Body;

impl Body {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_type: u64, sample_id_all: bool) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();

        Self {
            addr: *raw.addr(),
            old_len: *raw.old_len(),
            new_len: *raw.new_len(),
            bytes: raw.bytes().to_vec(),
            sample_id: sample_id_all.then(|| raw.sample_id(sample_type)),
        }
    }
}
