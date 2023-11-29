use crate::sampling::record::sample_id;

mod raw;

#[derive(Debug)]
pub struct Body {
    pub addr: u64,
    pub old_len: u16,
    pub new_len: u16,
    pub bytes: Vec<u8>,
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
            old_len: *raw.old_len(),
            new_len: *raw.new_len(),
            bytes: raw.bytes().as_slice().to_vec(),
            sample_id: raw.sample_id().clone(),
        }
    }
}
