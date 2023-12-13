use crate::syscall::bindings::*;
use crate::Event;

pub enum CacheOp {
    Read,
    Write,
    Prefetch,
}

impl CacheOp {
    const fn into_u64(self) -> u64 {
        use CacheOp::*;
        let id = match self {
            Read => PERF_COUNT_HW_CACHE_OP_READ,
            Write => PERF_COUNT_HW_CACHE_OP_WRITE,
            Prefetch => PERF_COUNT_HW_CACHE_OP_WRITE,
        };
        id as _
    }
}

pub enum CacheOpResult {
    Access,
    Miss,
}

impl CacheOpResult {
    const fn into_u64(self) -> u64 {
        use CacheOpResult::*;
        let id = match self {
            Access => PERF_COUNT_HW_CACHE_RESULT_ACCESS,
            Miss => PERF_COUNT_HW_CACHE_RESULT_MISS,
        };
        id as _
    }
}

pub enum HwEvent {
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
    CacheL1d(CacheOp, CacheOpResult),
    CacheL1i(CacheOp, CacheOpResult),
    CacheLl(CacheOp, CacheOpResult),
    CacheDtlb(CacheOp, CacheOpResult),
    CacheItlb(CacheOp, CacheOpResult),
    CacheBpu(CacheOp, CacheOpResult),
    CacheNode(CacheOp, CacheOpResult),
}

impl HwEvent {
    pub(crate) const fn is_cache_event(&self) -> bool {
        use HwEvent::*;
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
    pub(crate) const fn into_u64(self) -> u64 {
        use HwEvent::*;
        const fn calc_cache_config(id: perf_hw_id, op: u64, op_result: u64) -> perf_hw_id {
            (id as u64 | (op << 8) | (op_result << 16)) as _
        }
        let config = match self {
            CpuCycles => PERF_COUNT_HW_CPU_CYCLES,
            Instructions => PERF_COUNT_HW_INSTRUCTIONS,
            CacheReferences => PERF_COUNT_HW_CACHE_REFERENCES,
            CacheMisses => PERF_COUNT_HW_CACHE_MISSES,
            BranchInstructions => PERF_COUNT_HW_BRANCH_INSTRUCTIONS,
            BranchMisses => PERF_COUNT_HW_BRANCH_MISSES,
            BusCycles => PERF_COUNT_HW_BUS_CYCLES,
            StalledCyclesFrontend => PERF_COUNT_HW_STALLED_CYCLES_FRONTEND,
            StalledCyclesBackend => PERF_COUNT_HW_STALLED_CYCLES_BACKEND,
            RefCpuCycles => PERF_COUNT_HW_REF_CPU_CYCLES,
            CacheL1d(op, op_result) => {
                calc_cache_config(PERF_COUNT_HW_CACHE_L1D, op.into_u64(), op_result.into_u64())
            }
            CacheL1i(op, op_result) => {
                calc_cache_config(PERF_COUNT_HW_CACHE_L1I, op.into_u64(), op_result.into_u64())
            }
            CacheLl(op, op_result) => {
                calc_cache_config(PERF_COUNT_HW_CACHE_LL, op.into_u64(), op_result.into_u64())
            }
            CacheDtlb(op, op_result) => calc_cache_config(
                PERF_COUNT_HW_CACHE_DTLB,
                op.into_u64(),
                op_result.into_u64(),
            ),
            CacheItlb(op, op_result) => calc_cache_config(
                PERF_COUNT_HW_CACHE_ITLB,
                op.into_u64(),
                op_result.into_u64(),
            ),
            CacheBpu(op, op_result) => {
                calc_cache_config(PERF_COUNT_HW_CACHE_BPU, op.into_u64(), op_result.into_u64())
            }
            CacheNode(op, op_result) => calc_cache_config(
                PERF_COUNT_HW_CACHE_NODE,
                op.into_u64(),
                op_result.into_u64(),
            ),
        };
        config as _
    }
}

impl From<HwEvent> for Event {
    fn from(value: HwEvent) -> Self {
        Self::Hw(value)
    }
}
