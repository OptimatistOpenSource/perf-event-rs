use crate::counting::CountingGroupResult;
use crate::sampling::record::sample_id::SampleId;

mod raw;

#[derive(Debug, Clone)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
    pub values: CountingGroupResult,
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
            pid: sized.pid,
            tid: sized.tid,
            values: {
                let (header, body) = raw.values();
                CountingGroupResult::from_raw(header, body)
            },
            sample_id: sample_id_all.then(|| raw.sample_id()),
        }
    }
}
