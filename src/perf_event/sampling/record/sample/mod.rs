use crate::counting::CountingGroupResult;
use crate::sampling::record::sample::abi_and_regs::AbiAndRegs;
use crate::sampling::record::sample::data_src::DataSrc;

mod abi_and_regs;
mod data_src;
mod raw;
mod raw_v2;
mod body_v2;

pub use abi_and_regs::*;
pub use data_src::*;

#[derive(Debug, Clone)]
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
    pub v: CountingGroupResult,
    pub ips: Option<Vec<u64>>,
    pub data_1: Vec<u8>,
    pub user_abi_and_regs: Option<AbiAndRegs>,
    pub data_2: Option<Vec<u8>>,
    pub data_src: DataSrc,
    pub transaction: u64,
    pub intr_abi_and_regs: Option<AbiAndRegs>,
    pub phys_addr: u64,
    pub cgroup: u64,
    pub data_page_size: u64,
    pub code_page_size: u64,
    pub data_3: Option<Vec<u8>>,
}

type RawBody = raw::Body;

impl Body {
    pub unsafe fn from_ptr(
        ptr: *const u8,
        sample_stack_user: bool,
        sample_callchain: bool,
        sample_aux: bool,
        user_regs_len: Option<usize>,
        intr_regs_len: Option<usize>,
    ) -> Self {
        let raw = RawBody {
            sample_stack_user,
            sample_callchain,
            sample_aux,
            user_regs_len,
            intr_regs_len,
            ptr,
        };
        Self::from_raw(raw)
    }

    unsafe fn from_raw(raw: RawBody) -> Self {
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
            v: CountingGroupResult::from_raw(raw.v_header(), raw.v_body()),
            ips: raw.ips().map(|it| it.to_vec()).ok(),
            data_1: raw.data_1().to_vec(),
            user_abi_and_regs: raw.user_abi_and_regs().ok().map(AbiAndRegs::from_raw),
            data_2: {
                let dyn_size = raw.dyn_size().map(|it| *it).unwrap_or(0) as _;
                raw.data_2()
                    .ok()
                    .map(|it| it.iter().take(dyn_size).cloned().collect())
            },
            data_src: DataSrc::from_raw(*raw.data_src()),
            transaction: *raw.transaction(),
            intr_abi_and_regs: raw.intr_abi_and_regs().ok().map(AbiAndRegs::from_raw),
            phys_addr: *raw.phys_addr(),
            cgroup: *raw.cgroup(),
            data_page_size: *raw.data_page_size(),
            code_page_size: *raw.code_page_size(),
            data_3: raw.data_3().ok().map(|it| it.to_vec()),
        }
    }
}
