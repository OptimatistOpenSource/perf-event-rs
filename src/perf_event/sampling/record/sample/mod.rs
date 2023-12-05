use crate::counting::CountingGroupResult;
use crate::sampling::record::Abi;
use crate::syscall::bindings::*;

mod raw;

#[derive(Debug)]
pub struct AbiAndRegs {
    pub abi: Abi,
    pub regs: Vec<u64>,
}

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
    pub v: CountingGroupResult,
    pub ips: Vec<u64>,
    pub data_1: Vec<u8>,
    pub user_abi_and_regs: Option<AbiAndRegs>,
    pub data_2: Vec<u8>,
    pub dyn_size: Option<u64>,
    pub data_src: u64,
    pub transaction: u64,
    pub intr_abi_and_regs: Option<AbiAndRegs>,
    pub phys_addr: u64,
    pub cgroup: u64,
    pub data_page_size: u64,
    pub code_page_size: u64,
    //pub data_3: Vec<u8>,
}

type RawBody = raw::Body;

impl Body {
    pub unsafe fn from_ptr(ptr: *const u8, user_regs_len: usize, intr_regs_len: usize) -> Self {
        let raw = RawBody {
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
            ips: raw.ips().to_vec(),
            data_1: raw.data_1().to_vec(),
            user_abi_and_regs: raw.user_abi_and_regs().map(|(abi, regs)| {
                #[allow(non_upper_case_globals)]
                let abi = match *abi as _ {
                    perf_sample_regs_abi_PERF_SAMPLE_REGS_ABI_NONE => Abi::AbiNone,
                    perf_sample_regs_abi_PERF_SAMPLE_REGS_ABI_32 => Abi::Abi32,
                    perf_sample_regs_abi_PERF_SAMPLE_REGS_ABI_64 => Abi::Abi64,
                    _ => unreachable!(),
                };
                AbiAndRegs {
                    abi,
                    regs: regs.to_vec(),
                }
            }),
            data_2: raw.data_2().to_vec(),
            dyn_size: raw.dyn_size().cloned(),
            data_src: *raw.data_src(),
            transaction: *raw.transaction(),
            intr_abi_and_regs: raw.intr_abi_and_regs().map(|(abi, regs)| {
                #[allow(non_upper_case_globals)]
                let abi = match *abi as _ {
                    perf_sample_regs_abi_PERF_SAMPLE_REGS_ABI_NONE => Abi::AbiNone,
                    perf_sample_regs_abi_PERF_SAMPLE_REGS_ABI_32 => Abi::Abi32,
                    perf_sample_regs_abi_PERF_SAMPLE_REGS_ABI_64 => Abi::Abi64,
                    _ => unreachable!(),
                };
                AbiAndRegs {
                    abi,
                    regs: regs.to_vec(),
                }
            }),
            phys_addr: *raw.phys_addr(),
            cgroup: *raw.cgroup(),
            data_page_size: *raw.data_page_size(),
            code_page_size: *raw.code_page_size(),
            //data_3: raw.data_3().to_vec(),
        }
    }
}
