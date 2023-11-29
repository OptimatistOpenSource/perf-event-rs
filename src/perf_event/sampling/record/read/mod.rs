use crate::counting::{GroupCountingMemberResult, GroupCountingResult};
use crate::sampling::record::sample_id;

mod raw;

#[derive(Debug)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
    pub values: GroupCountingResult,
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
            pid: *raw.pid(),
            tid: *raw.tid(),
            values: GroupCountingResult {
                time_enabled: raw.values_header().time_enabled,
                time_running: raw.values_header().time_running,
                member_results: raw
                    .values_body()
                    .iter()
                    .map(|it| {
                        (
                            it.event_id,
                            GroupCountingMemberResult {
                                event_count: it.event_count,
                                #[cfg(feature = "kernel-6.0")]
                                event_lost: it.event_lost,
                            },
                        )
                    })
                    .collect(),
            },
            sample_id: raw.sample_id().clone(),
        }
    }
}
