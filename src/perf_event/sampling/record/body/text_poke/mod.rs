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

impl Body {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_type: u64, sample_id_all: bool) -> Self {
        let mut raw = raw::Raw {
            read_ptr: ptr,
            sample_type,
        };

        let sized = raw.sized();
        Self {
            addr: sized.addr,
            old_len: sized.old_len,
            new_len: sized.new_len,
            bytes: raw.bytes().to_vec(),
            sample_id: sample_id_all.then(|| raw.sample_id()),
        }
    }
}
