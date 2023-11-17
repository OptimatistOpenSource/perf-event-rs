use crate::syscall::bindings::*;

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

pub enum CacheOp {
    Read,
    Write,
    Prefetch,
}

impl From<CacheOp> for u64 {
    fn from(value: CacheOp) -> Self {
        use CacheOp::*;
        let id = match value {
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

impl From<CacheOpResult> for u64 {
    fn from(value: CacheOpResult) -> Self {
        use CacheOpResult::*;
        let id = match value {
            Access => perf_hw_cache_op_result_id_PERF_COUNT_HW_CACHE_RESULT_ACCESS,
            Miss => perf_hw_cache_op_result_id_PERF_COUNT_HW_CACHE_RESULT_MISS,
        };
        id as u64
    }
}

impl From<HwEvent> for u64 {
    fn from(value: HwEvent) -> Self {
        use HwEvent::*;
        fn calc_cache_config(id: u64, op: u64, op_result: u64) -> u64 {
            id | (op << 8) | (op_result << 16)
        }
        match value {
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
                op.into(),
                op_result.into(),
            ),
            CacheL1i(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_L1I as u64,
                op.into(),
                op_result.into(),
            ),
            CacheLl(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_LL as u64,
                op.into(),
                op_result.into(),
            ),
            CacheDtlb(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_DTLB as u64,
                op.into(),
                op_result.into(),
            ),
            CacheItlb(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_ITLB as u64,
                op.into(),
                op_result.into(),
            ),
            CacheBpu(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_BPU as u64,
                op.into(),
                op_result.into(),
            ),
            CacheNode(op, op_result) => calc_cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_NODE as u64,
                op.into(),
                op_result.into(),
            ),
        }
    }
}
