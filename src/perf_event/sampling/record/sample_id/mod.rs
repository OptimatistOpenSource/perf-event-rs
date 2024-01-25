mod raw;

#[derive(Debug, Clone)]
pub struct SampleId {
    pub pid: Option<u32>,
    pub tid: Option<u32>,
    pub time: Option<u64>,
    pub id_1: Option<u64>,
    pub stream_id: Option<u64>,
    pub cpu: Option<u32>,
    #[cfg(feature = "linux-3.12")]
    pub id_2: Option<u64>,
}

impl SampleId {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_type: u64) -> Self {
        let mut raw = raw::Raw {
            read_ptr: ptr,
            sample_type,
        };

        Self {
            pid: raw.pid().cloned(),
            tid: raw.tid().cloned(),
            time: raw.time().cloned(),
            id_1: raw.id_1().cloned(),
            stream_id: raw.stream_id().cloned(),
            cpu: raw.cpu().cloned(),
            #[cfg(feature = "linux-3.12")]
            id_2: raw.id_2().cloned(),
        }
    }
}
