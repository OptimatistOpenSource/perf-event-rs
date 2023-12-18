use crate::sampling::record::sample_id::SampleId;

mod raw;

#[derive(Debug, Clone)]
pub struct Body {
    pub sample_id: Option<SampleId>,
}

impl Body {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_type: u64, sample_id_all: bool) -> Self {
        let raw = &*(ptr as *const raw::Raw);

        Self {
            sample_id: sample_id_all.then(|| raw.sample_id(sample_type)),
        }
    }
}
