use crate::syscall::bindings::*;

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum Abi {
    AbiNone,
    Abi32,
    Abi64,
}

impl Abi {
    pub(crate) fn from_raw(abi: u64) -> Self {
        match abi as _ {
            PERF_SAMPLE_REGS_ABI_NONE => Abi::AbiNone,
            PERF_SAMPLE_REGS_ABI_32 => Abi::Abi32,
            PERF_SAMPLE_REGS_ABI_64 => Abi::Abi64,
            abi => unreachable!("ABI: {}", abi),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AbiAndRegs {
    pub abi: Abi,
    pub regs: Vec<u64>,
}

impl AbiAndRegs {
    pub(crate) fn from_raw(raw: (&u64, &[u64])) -> Self {
        let (abi, regs) = raw;
        Self {
            abi: Abi::from_raw(*abi),
            regs: regs.to_vec(),
        }
    }
}
