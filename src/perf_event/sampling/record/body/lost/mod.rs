use crate::sampling::record::sample_id::SampleId;

mod raw;

#[derive(Debug, Clone)]
pub struct Body {
    pub id: u64,
    pub lost: u64,
    pub sample_id: Option<SampleId>,
}

impl Body {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_type: u64, sample_id_all: bool) -> Self {
        let raw = &*(ptr as *const raw::Raw);
        Self {
            id: raw.id,
            lost: raw.lost,
            sample_id: sample_id_all.then(|| raw.sample_id(sample_type)),
        }
    }
}
