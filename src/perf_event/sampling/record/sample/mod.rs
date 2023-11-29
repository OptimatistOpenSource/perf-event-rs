use crate::counting::{GroupCountingMemberResult, GroupCountingResult};

mod raw;

#[derive(Debug)]
pub struct Body {
    pub sample_id: u64,
    pub ip: u64,
    pub pid: u32,
    pub tid: u32,
    pub time: u64,
    pub addr: u64,
    pub id: u64,
    pub stream_id: u64,
    pub cpu: u32,
    pub res: u32,
    pub period: u64,
    pub v: GroupCountingResult,
    pub ips: Vec<u64>,
    pub data_1: Vec<u8>,
    pub data_2: Vec<u8>,
    pub dyn_size: Option<u64>,
    pub data_src: u64,
    pub transaction: u64,
    pub phys_addr: u64,
    pub cgroup: u64,
    pub data_page_size: u64,
    pub code_page_size: u64,
    pub data_3: Vec<u8>,
}

type RawBody = raw::Body;

impl Body {
    pub unsafe fn from_ptr(ptr: *const u8) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();
        Self::from_raw(raw)
    }

    unsafe fn from_raw(raw: &RawBody) -> Self {
        Self {
            sample_id: *raw.sample_id(),
            ip: *raw.ip(),
            pid: *raw.pid(),
            tid: *raw.tid(),
            time: *raw.time(),
            addr: *raw.addr(),
            id: *raw.id(),
            stream_id: *raw.stream_id(),
            cpu: *raw.cpu(),
            res: *raw.res(),
            period: *raw.period(),
            v: GroupCountingResult {
                time_enabled: raw.v_header().time_enabled,
                time_running: raw.v_header().time_running,
                member_results: raw
                    .v_body()
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
            ips: raw.ips().to_vec(),
            data_1: raw.data_1().to_vec(),
            data_2: raw.data_2().to_vec(),
            dyn_size: raw.dyn_size().cloned(),
            data_src: *raw.data_src(),
            transaction: *raw.transaction(),
            phys_addr: *raw.phys_addr(),
            cgroup: *raw.cgroup(),
            data_page_size: *raw.data_page_size(),
            code_page_size: *raw.code_page_size(),
            data_3: raw.data_3().to_vec(),
        }
    }
}
