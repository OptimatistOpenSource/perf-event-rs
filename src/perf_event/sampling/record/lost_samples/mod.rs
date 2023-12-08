use crate::sampling::record::SampleId;

mod raw;

#[derive(Debug, Clone)]
pub struct Body {
    pub lost: u64,
    pub sample_id: Option<SampleId>,
}

type RawBody = raw::Body;

impl Body {
    pub unsafe fn from_ptr(ptr: *const u8, sample_id_all: bool) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();

        Self {
            lost: raw.lost,
            sample_id: sample_id_all.then(|| raw.sample_id().clone()),
        }
    }
}
