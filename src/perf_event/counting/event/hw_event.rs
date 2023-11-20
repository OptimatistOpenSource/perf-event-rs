use crate::perf_event::counting::Event;
use crate::syscall::bindings::*;

pub enum CacheOp {
    Read,
    Write,
    Prefetch,
}

impl CacheOp {
    fn into_u64(self) -> u64 {
        use CacheOp::*;
        let id = match self {
            Read => perf_hw_cache_op_id_PERF_COUNT_HW_CACHE_OP_READ,
            Write => perf_hw_cache_op_id_PERF_COUNT_HW_CACHE_OP_WRITE,
            Prefetch => perf_hw_cache_op_id_PERF_COUNT_HW_CACHE_OP_WRITE,
        };
        id as u64
    }
}

pub enum CacheOpResult {
    Access,
    Miss,
}

impl CacheOpResult {
    fn into_u64(self) -> u64 {
        use CacheOpResult::*;
        let id = match self {
            Access => perf_hw_cache_op_result_id_PERF_COUNT_HW_CACHE_RESULT_ACCESS,
            Miss => perf_hw_cache_op_result_id_PERF_COUNT_HW_CACHE_RESULT_MISS,
        };
        id as u64
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
    pub(crate) fn is_cache_event(&self) -> bool {
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
    pub(crate) fn into_u64(self) -> u64 {
        use HwEvent::*;
        fn calc_cache_config(id: u64, op: u64, op_result: u64) -> u64 {
            id | (op << 8) | (op_result << 16)
        }
        match self {
            CpuCycles => perf_hw_id_PERF_COUNT_HW_CPU_CYCLES as u64,
            Instructions => perf_hw_id_PERF_COUNT_HW_INSTRUCTIONS as u64,
            CacheReferences => perf_hw_id_PERF_COUNT_HW_CACHE_REFERENCES as u64,
            CacheMisses => perf_hw_id_PERF_COUNT_HW_CACHE_MISSES as u64,
            BranchInstructions => perf_hw_id_PERF_COUNT_HW_BRANCH_INSTRUCTIONS as u64,
            BranchMisses => perf_hw_id_PERF_COUNT_HW_BRANCH_MISSES as u64,
            BusCycles => perf_hw_id_PERF_COUNT_HW_BUS_CYCLES as u64,
            StalledCyclesFrontend => perf_hw_id_PERF_COUNT_HW_STALLED_CYCLES_FRONTEND as u64,
            StalledCyclesBackend => perf_hw_id_PERF_COUNT_HW_STALLED_CYCLES_BACKEND as u64,
            RefCpuCycles => perf_hw_id_PERF_COUNT_HW_REF_CPU_CYCLES as u64,
            CacheL1d(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_L1D as u64,
                op.into_u64(),
                op_result.into_u64(),
            ),
            CacheL1i(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_L1I as u64,
                op.into_u64(),
                op_result.into_u64(),
            ),
            CacheLl(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_LL as u64,
                op.into_u64(),
                op_result.into_u64(),
            ),
            CacheDtlb(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_DTLB as u64,
                op.into_u64(),
                op_result.into_u64(),
            ),
            CacheItlb(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_ITLB as u64,
                op.into_u64(),
                op_result.into_u64(),
            ),
            CacheBpu(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_BPU as u64,
                op.into_u64(),
                op_result.into_u64(),
            ),
            CacheNode(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_NODE as u64,
                op.into_u64(),
                op_result.into_u64(),
            ),
        }
    }
}

impl From<HwEvent> for Event {
    fn from(value: HwEvent) -> Self {
        Self::Hw(value)
    }
}
