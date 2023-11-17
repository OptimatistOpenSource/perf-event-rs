use crate::syscall::bindings::*;

pub enum HwCountingEvent {
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
}

impl From<HwCountingEvent> for perf_hw_id {
    fn from(value: HwCountingEvent) -> Self {
        use HwCountingEvent::*;
        match value {
            CpuCycles => perf_hw_id_PERF_COUNT_HW_CPU_CYCLES,
            Instructions => perf_hw_id_PERF_COUNT_HW_INSTRUCTIONS,
            CacheReferences => perf_hw_id_PERF_COUNT_HW_CACHE_REFERENCES,
            CacheMisses => perf_hw_id_PERF_COUNT_HW_CACHE_MISSES,
            BranchInstructions => perf_hw_id_PERF_COUNT_HW_BRANCH_INSTRUCTIONS,
            BranchMisses => perf_hw_id_PERF_COUNT_HW_BRANCH_MISSES,
            BusCycles => perf_hw_id_PERF_COUNT_HW_BUS_CYCLES,
            StalledCyclesFrontend => perf_hw_id_PERF_COUNT_HW_STALLED_CYCLES_FRONTEND,
            StalledCyclesBackend => perf_hw_id_PERF_COUNT_HW_STALLED_CYCLES_BACKEND,
            RefCpuCycles => perf_hw_id_PERF_COUNT_HW_REF_CPU_CYCLES,
        }
    }
}
