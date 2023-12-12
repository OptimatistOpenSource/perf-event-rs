use crate::counting::CountingGroupResult;
use crate::sampling::record::sample::abi_and_regs::AbiAndRegs;

mod abi_and_regs;
mod data_src;
mod raw;

pub use abi_and_regs::*;
pub use data_src::*;

#[derive(Debug, Clone)]
pub struct Body {
    pub sample_id: Option<u64>,
    pub ip: Option<u64>,
    pub pid: Option<u32>,
    pub tid: Option<u32>,
    pub time: Option<u64>,
    pub addr: Option<u64>,
    pub id: Option<u64>,
    pub stream_id: Option<u64>,
    pub cpu: Option<u32>,
    pub period: Option<u64>,
    pub v: Option<CountingGroupResult>,
    pub ips: Option<Vec<u64>>,
    pub data_raw: Option<Vec<u8>>,
    pub abi_and_regs_user: Option<AbiAndRegs>,
    pub data_stack_user: Option<Vec<u8>>,
    pub data_src: Option<DataSrc>,
    pub transaction: Option<u64>,
    pub abi_and_regs_intr: Option<AbiAndRegs>,
    pub phys_addr: Option<u64>,
    pub cgroup: Option<u64>,
    pub data_page_size: Option<u64>,
    pub code_page_size: Option<u64>,
}

type RawBody = raw::Body;

impl Body {
    pub(crate) unsafe fn from_ptr(
        ptr: *const u8,
        sample_type: u64,
        regs_user_len: usize,
        regs_intr_len: usize,
    ) -> Self {
        let mut raw = RawBody {
            read_ptr: ptr,
            sample_type,
        };

        Self {
            sample_id: raw.sample_id().cloned(),
            ip: raw.ip().cloned(),
            pid: raw.pid().cloned(),
            tid: raw.tid().cloned(),
            time: raw.time().cloned(),
            addr: raw.addr().cloned(),
            id: raw.id().cloned(),
            stream_id: raw.stream_id().cloned(),
            cpu: raw.cpu().cloned(),
            period: raw.period().cloned(),
            v: raw.v().map(|(h, b)| CountingGroupResult::from_raw(h, b)),
            ips: raw.ips().map(|it| it.to_vec()),
            data_raw: raw.data_raw().map(|it| it.to_vec()),
            abi_and_regs_user: raw
                .abi_and_regs_user(regs_user_len)
                .map(AbiAndRegs::from_raw),
            data_stack_user: raw.data_stack_user().map(|it| it.to_vec()),
            data_src: raw.data_src().cloned().map(DataSrc::from_raw),
            transaction: raw.transaction().cloned(),
            abi_and_regs_intr: raw
                .abi_and_regs_intr(regs_intr_len)
                .map(AbiAndRegs::from_raw),
            phys_addr: raw.phys_addr().cloned(),
            cgroup: raw.cgroup().cloned(),
            data_page_size: raw.data_page_size().cloned(),
            code_page_size: raw.code_page_size().cloned(),
        }
    }
}
