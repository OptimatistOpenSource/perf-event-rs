use crate::sampling::record::Abi;
use crate::syscall::bindings::*;

#[derive(Debug, Clone)]
pub struct AbiAndRegs {
    pub abi: Abi,
    pub regs: Vec<u64>,
}

impl AbiAndRegs {
    pub(crate) fn from_raw(raw: (&u64, &[u64])) -> Self {
        let (abi, regs) = raw;
        #[allow(non_upper_case_globals)]
        let abi = match *abi as _ {
            PERF_SAMPLE_REGS_ABI_NONE => Abi::AbiNone,
            PERF_SAMPLE_REGS_ABI_32 => Abi::Abi32,
            PERF_SAMPLE_REGS_ABI_64 => Abi::Abi64,
            abi => unreachable!("ABI: {}", abi),
        };
        Self {
            abi,
            regs: regs.to_vec(),
        }
    }
}
