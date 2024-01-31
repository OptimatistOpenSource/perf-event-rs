use crate::syscall::bindings::*;
use crate::Event;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CacheOp {
    Read,
    Write,
    Prefetch,
}

impl CacheOp {
    const fn as_u64(&self) -> u64 {
        use CacheOp::*;
        #[rustfmt::skip]
        let val = match self {
            Read     => PERF_COUNT_HW_CACHE_OP_READ,
            Write    => PERF_COUNT_HW_CACHE_OP_WRITE,
            Prefetch => PERF_COUNT_HW_CACHE_OP_PREFETCH,
        };
        val as _
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CacheOpResult {
    Access,
    Miss,
}

impl CacheOpResult {
    const fn as_u64(&self) -> u64 {
        use CacheOpResult::*;
        #[rustfmt::skip]
        let val = match self {
            Access => PERF_COUNT_HW_CACHE_RESULT_ACCESS,
            Miss   => PERF_COUNT_HW_CACHE_RESULT_MISS,
        };
        val as _
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug)]
pub enum HardwareEvent {
    CpuCycles,
    Instructions,
    CacheReferences,
    CacheMisses,
    BranchInstructions,
    BranchMisses,
    BusCycles,
    StalledCyclesFrontend,
    StalledCyclesBackend,
    RefCpuCycles,
    CacheL1d (CacheOp, CacheOpResult),
    CacheL1i (CacheOp, CacheOpResult),
    CacheLl  (CacheOp, CacheOpResult),
    CacheDtlb(CacheOp, CacheOpResult),
    CacheItlb(CacheOp, CacheOpResult),
    CacheBpu (CacheOp, CacheOpResult),
    CacheNode(CacheOp, CacheOpResult),
}

impl HardwareEvent {
    pub(crate) const fn is_cache_event(&self) -> bool {
        use HardwareEvent::*;
        matches!(
            self,
            CacheL1d(..)
                | CacheL1i(..)
                | CacheLl(..)
                | CacheDtlb(..)
                | CacheItlb(..)
                | CacheBpu(..)
                | CacheNode(..)
        )
    }
    pub(crate) const fn as_u64(&self) -> u64 {
        use HardwareEvent::*;
        const fn calc_cache_config(id: perf_hw_id, op: u64, op_result: u64) -> perf_hw_id {
            (id as u64 | (op << 8) | (op_result << 16)) as _
        }

        #[rustfmt::skip]
        let val = match self {
            CpuCycles             => PERF_COUNT_HW_CPU_CYCLES,
            Instructions          => PERF_COUNT_HW_INSTRUCTIONS,
            CacheReferences       => PERF_COUNT_HW_CACHE_REFERENCES,
            CacheMisses           => PERF_COUNT_HW_CACHE_MISSES,
            BranchInstructions    => PERF_COUNT_HW_BRANCH_INSTRUCTIONS,
            BranchMisses          => PERF_COUNT_HW_BRANCH_MISSES,
            BusCycles             => PERF_COUNT_HW_BUS_CYCLES,
            StalledCyclesFrontend => PERF_COUNT_HW_STALLED_CYCLES_FRONTEND,
            StalledCyclesBackend  => PERF_COUNT_HW_STALLED_CYCLES_BACKEND,
            RefCpuCycles          => PERF_COUNT_HW_REF_CPU_CYCLES,
            CacheL1d (o, r) => calc_cache_config(PERF_COUNT_HW_CACHE_L1D,  o.as_u64(), r.as_u64()),
            CacheL1i (o, r) => calc_cache_config(PERF_COUNT_HW_CACHE_L1I,  o.as_u64(), r.as_u64()),
            CacheLl  (o, r) => calc_cache_config(PERF_COUNT_HW_CACHE_LL,   o.as_u64(), r.as_u64()),
            CacheDtlb(o, r) => calc_cache_config(PERF_COUNT_HW_CACHE_DTLB, o.as_u64(), r.as_u64()),
            CacheItlb(o, r) => calc_cache_config(PERF_COUNT_HW_CACHE_ITLB, o.as_u64(), r.as_u64()),
            CacheBpu (o, r) => calc_cache_config(PERF_COUNT_HW_CACHE_BPU,  o.as_u64(), r.as_u64()),
            CacheNode(o, r) => calc_cache_config(PERF_COUNT_HW_CACHE_NODE, o.as_u64(), r.as_u64()),
        };
        val as _
    }
}

impl From<HardwareEvent> for Event {
    fn from(value: HardwareEvent) -> Self {
        Self::Hardware(value)
    }
}
