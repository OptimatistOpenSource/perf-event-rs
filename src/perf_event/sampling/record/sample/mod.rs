use crate::counting::{GroupCountingMemberResult, GroupCountingResult};

pub mod raw;

pub struct Body {
    sample_id: u64,
    ip: u64,
    pid: u32,
    tid: u32,
    time: u64,
    addr: u64,
    id: u64,
    stream_id: u64,
    cpu: u32,
    res: u32,
    period: u64,
    v: GroupCountingResult,
    ips: Vec<u64>,
    data_1: Vec<u8>,
    data_2: Vec<u8>,
    dyn_size: Option<u64>,
    data_src: u64,
    transaction: u64,
    phys_addr: u64,
    cgroup: u64,
    data_page_size: u64,
    code_page_size: u64,
    data_3: Vec<u8>,
}

type RawBody = raw::Body;

impl Body {
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
